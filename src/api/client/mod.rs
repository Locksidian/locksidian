//! HTTP client module.
//!
//! The HTTP client is based on `hyper::Client` and uses `Iron`'s request extension mechanism to
//! share a thread-safe structure at runtime.

#[macro_use]
mod macros;
pub mod prelude;