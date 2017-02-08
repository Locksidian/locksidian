//! Locksidian's REST API.

mod router;
mod server;

pub use self::server::Server;
pub use self::router::routes;