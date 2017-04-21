//! Prelude for the `persistence` module.
//!
//! Re-export all the required dependencies when using a persisted context.

use std::sync::Arc;

pub use diesel::prelude::*;
pub use diesel::sqlite::SqliteConnection;

pub use r2d2::{Config, Pool};
pub use r2d2_diesel::ConnectionManager;

pub use persistence::*;
pub use persistence::repository::*;

pub type ConnectionPool = Arc<Pool<ConnectionManager<SqliteConnection>>>;
pub type PooledConnection = ::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub trait PoolExtractor {
    fn get_connection(&self) -> Result<PooledConnection, String>;
}