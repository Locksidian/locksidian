//! Locksidian's REST API.

mod router;
mod server;
mod middleware;

pub use self::server::Server;
pub use self::router::routes;