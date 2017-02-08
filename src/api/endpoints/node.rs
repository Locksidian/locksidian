//! Node specific endpoint: `/node/*`

use iron::prelude::*;
use iron::status;
use bodyparser::Json;

pub fn home(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<Json>();

    match body {
        Ok(Some(json)) => {
            match json.get("values") {
                Some(values) => {
                    if values.is_array() {
                        let arr = values.as_array().unwrap();
                        let mut sum = 0;

                        for i in arr {
                            sum = sum + i.as_u64().unwrap()
                        }

                        println!("{:?}", sum);
                        println!("{}", json!({"key": "value"}));
                    }
                    else {
                        println!("Values is not an array...");
                    }

                },
                None => println!("No values...")
            }
        },
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with((status::Ok, "Hello Node!")))
}

#[cfg(test)]
mod test {
    use iron::Headers;
    use iron_test::{request, response};

    use api::endpoints::node;

    #[test]
    fn home() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            &node::home
        ).unwrap();

        let bytes = response::extract_body_to_bytes(res);
        assert_eq!(bytes, b"Hello Node!");
    }
}