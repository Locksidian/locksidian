//! HTTP client prelude.
//!
//! Re-export the `hyper::Client` structure and define the `ClientExtractor` structure.

use error::*;

pub use std::sync::Arc;
pub use std::io::Read;
pub use hyper::Client;

pub trait ClientExtractor {
    fn get_client(&self) -> LocksidianResult<&Arc<Client>>;
}