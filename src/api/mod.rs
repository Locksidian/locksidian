//! Locksidian's REST API.

mod router;
mod server;
mod middleware;
mod endpoints;

pub use self::server::Server;
pub use self::router::routes as router;