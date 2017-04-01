//! Expose various information about this node.

use iron::prelude::*;

/// Basic information about this node, include its package name, current version, description and
/// authors.
///
/// TODO: add the active `Identity` public data in an `identity` attribute of the HTTP response.
pub fn node_info(_: &mut Request) -> IronResult<Response> {
    response!(Ok, {
        "package": ::PACKAGE,
        "version": ::VERSION,
        "description": ::DESCRIPTION,
        "authors": ::AUTHORS
    })
}

#[cfg(test)]
mod test {
    use iron::{Headers, status};
    use iron_test::{request, response};

    use api::endpoints::node;

    #[test]
    fn should_get_the_accurate_node_info() {
        let res = request::get(
            "http://localhost:8080/test",
            Headers::new(),
            &node::node_info
        ).unwrap();

        assert_eq!(res.status.unwrap(), status::Ok);

        let body = response::extract_body_to_string(res);
        assert!(body.contains(format!(r#""package":"{}""#, ::PACKAGE).as_str()));
        assert!(body.contains(format!(r#""version":"{}""#, ::VERSION).as_str()));
        assert!(body.contains(format!(r#""description":"{}""#, ::DESCRIPTION).as_str()));
        assert!(body.contains(format!(r#""authors":"{}""#, ::AUTHORS).as_str()));
    }
}