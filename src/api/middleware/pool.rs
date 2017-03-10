//! Connection pool middleware.
//!
//! `BeforeMiddleware` used to create a pool of connections at daemon startup and distribute it
//! to the Iron handlers.
//!
//! A connection can then be gathered from the pool by using `req.get_connection()`.

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
    pub fn new(database_path: String) -> Result<PoolMiddleware, String>{
        check_database_path(database_path.as_ref());

        let config = Config::default();
        let manager = ConnectionManager::<SqliteConnection>::new(database_path.as_str());

        match Pool::new(config, manager) {
            Ok(pool) => Ok(PoolMiddleware {
                pool: Arc::new(pool)
            }),
            Err(err) => Err(err.to_string())
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
    fn get_connection(&self) -> Result<PooledConnection, String> {
        match self.extensions.get::<PoolMiddleware>() {
            Some(pool) => match pool.get() {
                Ok(connection) => Ok(connection),
                Err(err) => Err(err.to_string())
            },
            None => Err(String::from("No connection pool is embedded in this request"))
        }
    }
}