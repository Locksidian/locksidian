//! The root crate for the `Locksidian` project.
//!
//! ## Contributors
//!
//! [Valentin Fries](https://www.fries.io) ([GitHub](https://github.com/MrKloan)) & Aurélien Duval
//! ([GitHub](https://github.com/acid-killa666))
//!
//! ## Overview
//!
//! Locksidian is a pure [Rust](https://www.rust-lang.org/) implementation of the
//! [blockchain](https://en.wikipedia.org/wiki/Blockchain_(database)) technology, and was developed
//! as a study project during our first year of Master degree in *Software Architecture*.
//!
//! Our objective is to provide the ability to store and certify the existence and integrity of any
//! JSON document by storing and cryptographically signing them inside of the `Locksidian`
//! blockchain.
//!
//! All the interactions between the peers of the peer-to-peer network is done using the HTTP(S)/1.1
//! protocol through the use of REST APIs.
//!
//! ## Deeper into the rabbit hole
//!
//! *TODO*

// Third-party dependencies
extern crate time;
extern crate crypto;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use(router)]
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