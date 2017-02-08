//! [![build status](https://gitlab.com/locksidian/locksidian/badges/master/build.svg)](https://gitlab.com/locksidian/locksidian/pipelines)
//!
//! The root crate for the Locksidian project.
//!
//! ## Overview
//!
//! Locksidian is a pure [Rust](https://www.rust-lang.org/) implementation of the
//! [blockchain](https://en.wikipedia.org/wiki/Blockchain_(database)) technology.

// Third-party dependencies
extern crate crypto;
extern crate iron;
#[macro_use(router, url_for)]
extern crate router;

// Project modules
pub mod sec;
mod api;

/// Locksidian entry point.
fn main() {
    let server = api::Server::new(String::from("localhost:8080"));
    server.start(api::routes());
}