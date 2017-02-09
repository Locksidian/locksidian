//! Endpoints used when an error has occurred, such as a `404 Not Found`.

use iron::prelude::*;
use iron::status;

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::NotFound, json!({
            "status": 404,
            "error": "Not Found"
        }).to_string()
    )))
}

#[cfg(test)]
mod test {
    use iron::Headers;
    use iron_test::{request, response};

    use api::endpoints::error;

    #[test]
    fn not_found() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            &error::not_found
        ).unwrap();

        let json_body = response::extract_body_to_string(res);
        assert_eq!(json_body, json!({
                "status": 404,
                "error": "Not Found"
            }).to_string()
        );
    }
}