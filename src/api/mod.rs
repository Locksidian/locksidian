//! Locksidian's REST API.

#[macro_use]
mod macros;

mod router;
mod server;
mod middleware;
mod endpoints;
pub mod cli;

pub use self::server::Server;
pub use self::router::routes as router;