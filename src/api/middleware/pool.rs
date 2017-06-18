//! Connection pool middleware.
//!
//! `BeforeMiddleware` used to create a pool of connections at daemon startup and distribute it
//! to the Iron handlers.
//!
//! A connection can then be gathered from the pool by using:
//!
//! ```rust
//! match req.get_connection() {
//!     Ok(connection) => {
//!         ...
//!     },
//!     Err(msg) => response!(InternalServerError, {"error": msg})
//! }
//! ```

use error::*;
use std::sync::Arc;

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};

use persistence::prelude::*;

pub struct PoolMiddleware {
    pool: ConnectionPool
}

impl typemap::Key for PoolMiddleware {
    type Value = ConnectionPool;
}

impl PoolMiddleware {
    /// Connection pool configuration using a custom `Config` builder.
    pub fn new(database_path: String) -> LocksidianResult<PoolMiddleware> {
        check_database_path(database_path.as_ref());

        let config = Config::default();
        let manager = ConnectionManager::<SqliteConnection>::new(database_path.as_str());

        match Pool::new(config, manager) {
            Ok(pool) => Ok(PoolMiddleware {
                pool: Arc::new(pool)
            }),
            Err(err) => Err(LocksidianError::from_err(err))
        }
    }
}

impl BeforeMiddleware for PoolMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<PoolMiddleware>(self.pool.clone());
        Ok(())
    }
}

impl<'a, 'b> PoolExtractor for Request<'a, 'b> {
    fn get_connection(&self) -> IronResult<PooledConnection> {
        match self.extensions.get::<PoolMiddleware>() {
            Some(pool) => match pool.get() {
                Ok(connection) => Ok(connection),
                Err(err) => http_error!(InternalServerError, {"error": err.description()})
            },
            None => http_error!(InternalServerError, {"error": "No connection pool is embedded in this request"})
        }
    }
}