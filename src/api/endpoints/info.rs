//! Expose various information about this node.

use iron::prelude::*;

pub fn node_info(_: &mut Request) -> IronResult<Response> {
    response!(Ok, {
        "package": ::PACKAGE,
        "version": ::VERSION,
        "description": ::DESCRIPTION,
        "authors": ::AUTHORS
    })
}