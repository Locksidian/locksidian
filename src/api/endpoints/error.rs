//! Endpoints used when an error has occurred, such as a `404 Not Found`.

use iron::prelude::*;

/// `404 Not Found` endpoint.
pub fn not_found(_: &mut Request) -> IronResult<Response> {
    http_response!(NotFound, {"error": "Not Found"})
}