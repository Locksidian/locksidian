//! Endpoints used when an error has occurred, such as a `404 Not Found`.

use iron::prelude::*;

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    response!(NotFound, {"error": "Not Found"})
}

#[cfg(test)]
mod test {
    use iron::{Headers, status};
    use iron_test::{request, response};

    use api::endpoints::error;

    #[test]
    fn not_found() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            &error::not_found
        ).unwrap();

        assert_eq!(res.status.unwrap(), status::NotFound);

        let json_body = response::extract_body_to_string(res);
        assert_eq!(json_body, json!({
                "error": "Not Found"
            }).to_string()
        );
    }
}