//! HTTP client middleware.
//!
//! `BeforeMiddleware` used to create an HTTP client shared between the API endpoints using Iron's
//! request extension system.
//!
//! A client can then be gathered from the pool by using:
//!
//! ```rust
//! match req.get_client() {
//!     Ok(client) => {
//!         ...
//!     },
//!     Err(msg) => response!(InternalServerError, {"error": msg})
//! }
//! ```

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};

use api::client::prelude::*;

pub struct ClientMiddleware {
    client: Arc<Client>
}

impl typemap::Key for ClientMiddleware {
    type Value = Arc<Client>;
}

impl ClientMiddleware {
    pub fn new() -> ClientMiddleware {
        ClientMiddleware {
            client: Arc::new(Client::new())
        }
    }
}

impl BeforeMiddleware for ClientMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ClientMiddleware>(self.client.clone());
        Ok(())
    }
}

impl<'a, 'b> ClientExtractor for Request<'a, 'b> {
    fn get_client(&self) -> IronResult<&Arc<Client>> {
        match self.extensions.get::<ClientMiddleware>() {
            Some(client) => Ok(client),
            None => error!(InternalServerError, "No HTTP client is embedded in this request")
        }
    }
}