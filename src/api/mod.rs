//! Locksidian's REST API.

#[macro_use]
mod macros;

#[macro_use]
mod client;

mod router;
mod server;
mod middleware;
mod endpoints;

pub use self::server::Server;
pub use self::router::routes as router;