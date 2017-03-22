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