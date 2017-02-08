//! The root crate for the Locksidian project.
//!
//! ## Overview
//!
//! Locksidian is a pure [Rust](https://www.rust-lang.org/) implementation of the
//! [blockchain](https://en.wikipedia.org/wiki/Blockchain_(database)) technology.

// Third-party dependencies
extern crate time;
extern crate crypto;

#[macro_use]
extern crate serde_json;

#[macro_use(router, url_for)]
extern crate router;
extern crate iron;
extern crate bodyparser;
extern crate iron_test;

// Project modules
pub mod sec;
mod api;

/// Locksidian entry point.
fn main() {
    let server = api::Server::new(String::from("localhost:8080"));
    server.start(api::router());
}