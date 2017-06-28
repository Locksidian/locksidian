//! HTTP protected middleware.
//!
//! `BeforeMiddleware` used to:
//!
//! - Check if URL is protected under specified method;
//! - Get the current identity;
//! - Check if X-LS-SIGNATURE header is present and has hexadecimal data;
//! - Get sha512 request body hash checksum;
//! - Compare request body hash with X-LS-SIGNATURE header and verfiy signature.
//!
//! Sends 403 error if protection blocked the request.
//!
//! Gives access to the requested page if request is authorized.

use error::*;
use iron::prelude::*;
use iron::BeforeMiddleware;

use persistence::prelude::*;
use blockchain::identity::identity_cli::get_active_identity;
use blockchain::identity::Identity;
use sec::sha::sha512;
use sec::rsa::Rsa;

use std::collections::HashMap;

pub struct ProtectedMiddleware {
    endpoints_filter: HashMap<&'static str, Vec<&'static str>>
}

impl ProtectedMiddleware {
    pub fn new() -> ProtectedMiddleware {
        let mut endpoints_filter = HashMap::new();

        ProtectedMiddleware::init(&mut endpoints_filter);

        ProtectedMiddleware {
            endpoints_filter: endpoints_filter
        }
    }

    fn init(endpoints_filter : &mut HashMap<&'static str, Vec<&'static str>>) {
        endpoints_filter.insert("/blocks", vec!["POST"]);
    }

    fn process_request(&self, req: &mut Request) -> IronResult<()> {
        match self.check_signature(req) {
            Ok(_) => Ok(()),
            Err(_) => http_error!(Forbidden, {"error": "Forbidden"})
        }
    }

    fn is_protected_route(&self, req: &mut Request) -> bool {
        let referer: String = self.get_referer(req);
        let method: &str = req.method.as_ref();

        self.is_method_protected(referer.as_str(), method)
    }

    fn is_method_protected(&self, referer: &str, method: &str) -> bool {
        match self.endpoints_filter.get(&referer) {
            Some(methods) => methods.contains(&method),
            None => false
        }
    }

    fn get_referer(&self, req: &mut Request) -> String {
        let mut referer: String = String::from("/");
        let path: String = req.url.path().join("/");
        referer.push_str(&path);

        referer
    }

    fn get_identity(&self, req: &mut Request) -> LocksidianResult<Identity> {
        match req.get_connection() {
            Ok(connection) => get_active_identity(&*connection),
            Err(err) => Err(LocksidianError::from_err(err))
        }
    }

    fn check_signature(&self, req: &mut Request) -> LocksidianResult<bool> {
        let hash_raw = self.get_body_hash(req)?;
        let signature_raw = self.get_header(req, "X-LS-SIGNATURE")?;
        let identity_raw = self.get_identity(req)?;
        let key: &Rsa = identity_raw.key();

        key.verify_signature(hash_raw.as_bytes(), signature_raw.as_slice())
    }

    fn get_header(&self, req: &mut Request, name : &str) -> LocksidianResult<Vec<u8>> {
        match req.headers.get_raw(name) {
            Some(header) => self.get_first_header_item(header),
            None => Err(LocksidianError::new(format!("Header \"{}\" not found", name)))
        }
    }

    fn get_first_header_item(&self, header : &[Vec<u8>]) -> LocksidianResult<Vec<u8>> {
        match header.get(0) {
            Some(value) => Ok(value.clone()),
            None => Err(LocksidianError::new(String::from("Requested header has no content")))
        }
    }

    fn get_body_hash(&self, req: &mut Request) -> LocksidianResult<String> {
        match body_raw!(req) {
            Ok(body) => self.calculate_body_hash(body),
            Err(_) => Err(LocksidianError::new(String::from("Error while parsing HTTP request body as raw data")))
        }
    }

    fn calculate_body_hash(&self, body: Option<String>) -> LocksidianResult<String> {
        match body {
            Some(data) => Ok(sha512(data.as_bytes())),
            None => Err(LocksidianError::new(String::from("No body available")))
        }
    }

}

impl BeforeMiddleware for ProtectedMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        match self.is_protected_route(req) {
            true => self.process_request(req),
            false => Ok(())
        }
    }
}