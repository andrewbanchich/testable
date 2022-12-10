use std::{fs::File, io::Read};
use testable::{make_testable, testable, Testable};

make_testable!();

fn main() {
    println!("\nHTTP RESPONSE:\n\n{:#}\n", real_http_request());

    println!(
        "\nHTTP RESPONSE CHARS:\n\n{}\n",
        count_chars_of_http_response()
    );

    println!(
        "\nCONCATENATED POST TITLES:\n\n{:#}\n",
        concat_response_titles()
    );

    println!("\nFILE CONTENTS:\n\n{:#}\n", real_file_io());
}

fn real_http_request() -> serde_json::Value {
    testable!(1, {
        reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts/1")
            .unwrap()
            .json()
            .unwrap()
    })
}

fn count_chars_of_http_response() -> usize {
    real_http_request()["title"]
        .as_str()
        .unwrap()
        .chars()
        .count()
}

fn another_real_http_request() -> serde_json::Value {
    testable!(2, {
        reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts/2")
            .unwrap()
            .json()
            .unwrap()
    })
}

fn yet_another_real_http_request() -> serde_json::Value {
    testable!(1, {
        reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts/3")
            .unwrap()
            .json()
            .unwrap()
    })
}

fn concat_response_titles() -> String {
    let a = real_http_request();
    let a = a["title"].as_str().unwrap();

    let b = another_real_http_request();
    let b = b["title"].as_str().unwrap();

    format!("{a} + {b}")
}

fn real_file_io() -> String {
    testable!(3, {
        let mut file = File::open("Cargo.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use testable::{mock, with_context};

    mock! {
    1 => serde_json::Value,
    2 => serde_json::Value,
    3 => String
    }

    #[test]
    fn http_request() {
        let res = with_context!(real_http_request(), {
            1 => serde_json::json!({ "title": "world" })
        });

        assert_eq!(res["title"].as_str().unwrap(), "world");
    }

    #[test]
    fn higher_up_stack() {
        let res = with_context!(count_chars_of_http_response(), {
            1 => serde_json::json!({ "title": "world" })
        });

        assert_eq!(res, 5);
    }

    #[test]
    fn file_io() {
        let res = with_context!(real_file_io(), {
            3 => String::from("foo")
        });

        assert_eq!(res, String::from("foo"));
    }

    #[test]
    fn concat_works() {
        let res = with_context!(concat_response_titles(), {
            1 => serde_json::json!({ "title": "foo" }),
            2 => serde_json::json!({ "title": "bar" })
        });

        assert_eq!(res, "foo + bar".to_string());
    }

    #[test]
    fn you_can_also_do_this() {
        let res = with_context!({
            let a = real_http_request();
            let a = a["title"].as_str().unwrap();

            let b = another_real_http_request();
            let b = b["title"].as_str().unwrap();

            format!("{a} + {b}")
        }, {
                1 => serde_json::json!({ "title": "foo" }),
                2 => serde_json::json!({ "title": "bar" })
            });

        assert_eq!(res, "foo + bar".to_string());
    }

    #[test]
    fn you_can_assign_the_same_location_id_to_multiple_places() {
        let res = with_context!({
            let a = real_http_request();
            let a = a["title"].as_str().unwrap();

            let b = another_real_http_request();
            let b = b["title"].as_str().unwrap();

            let c = yet_another_real_http_request();
            let c = c["title"].as_str().unwrap();

            format!("{a} + {b} + {c}")
        }, {
                1 => serde_json::json!({ "title": "foo" }),
                2 => serde_json::json!({ "title": "bar" })
            });

        assert_eq!(res, "foo + bar + foo".to_string());
    }
}
