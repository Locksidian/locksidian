//! HTTP client prelude.
//!
//! Re-export the `hyper::Client` structure and define the `ClientExtractor` structure.

pub use std::sync::Arc;
pub use std::io::Read;
pub use hyper::Client;

pub trait ClientExtractor {
    fn get_client(&self) -> Result<&Arc<Client>, String>;
}