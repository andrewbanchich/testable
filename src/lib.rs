use std::{any::Any, collections::HashMap};

#[macro_export]
macro_rules! make_testable {
    () => {
        pub trait Test<Out> {
            fn call_dependency(self) -> Out;
        }

        /// If we're not testing anything, just call the inner function.
        #[cfg(not(test))]
        impl<const ID: usize, Out, Func> Test<Out> for Testable<ID, Func>
        where
            Func: FnOnce() -> Out,
        {
            fn call_dependency(self) -> Out {
                (self.func)()
            }
        }
    };
}

#[macro_export]
macro_rules! testable {
    ($loc_id:literal, $func_body:block) => {
        Testable::<$loc_id, _> {
            func: || $func_body,
        }
        .call_dependency()
    };
}

#[macro_export]
macro_rules! mock {
    ($($loc_id:literal => $mock_type:ty), *$(,)?) => {
        thread_local!(
	    static __MOCKS: once_cell::sync::Lazy<std::sync::RwLock<testable::AnyMap>> = once_cell::sync::Lazy::new(|| {
		std::sync::RwLock::new(testable::AnyMap::default())
	    })
	);

	$(

	    impl<Func> Test<$mock_type> for Testable<$loc_id, Func>
	    where
		Func: FnOnce() -> $mock_type + 'static
        {

            fn call_dependency(self) -> $mock_type {
		crate::tests::__MOCKS.with(|mocks| {
                    let mocks = mocks.read().unwrap();
		    let boxed = mocks.get(&$loc_id)
			.expect(&format!("Test did not specify mock value for locationID ({})", $loc_id));
                    boxed.downcast_ref::<$mock_type>().unwrap().clone()
		})

            }
        }
	)+

    };
}

#[macro_export]
macro_rules! with_context {
    ($test_code:expr, { $($loc_id:literal => $mock_val:expr), *$(,)? }) => ({

	$(
	    __MOCKS.with(|m| {
		m.write().unwrap().insert($loc_id, Box::new($mock_val))
	    });
	)+

            $test_code
    });
}

pub type AnyMap = HashMap<usize, Box<dyn Any + Send + Sync>>;

/// Make any type testable.
/// Accepts a function which wraps what you want to mock.
/// The return value of that function must be able to be constructed by you.
pub struct Testable<const ID: usize, Func> {
    pub func: Func,
}
