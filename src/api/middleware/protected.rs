//! HTTP protected middleware.
//!
//! `BeforeMiddleware` used to :
//!
//! - Check if URL is protected
//! - Get the current identity
//! - Check if X-LS-SIGNATURE header is present and has hexa data
//! - Get sha512 request body hash checksum
//! - Compare request body hash with X-LS-SIGNATURE header
//!
//! Sends 403 error if protection blocked the request
//! Gives access to the requested page if request is authorized

use iron::prelude::*;
use iron::BeforeMiddleware;

use persistence::prelude::*;
use blockchain::identity::identity_cli::get_active_identity;
use blockchain::identity::Identity;
use sec::sha::sha512;
use sec::rsa::Rsa;
use iron::status::Status::Forbidden;

use std::error::Error;
use std::fmt::{self, Debug};

static ENDPOINTS_FILTER: &'static [&'static str] = &["/blocks"];


#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

pub struct ProtectedMiddleware {

}

impl ProtectedMiddleware {
    pub fn new() -> ProtectedMiddleware {
        ProtectedMiddleware {

        }
    }

    fn process_request(&self, req: &mut Request) -> IronResult<()>{
        match self.check_signature(req) {
            Ok(_) => Ok(()),
            Err(_) => Err(IronError::new(StringError("Forbidden".to_string()), Forbidden))
        }
    }

    fn is_protected_route(&self, req: &mut Request) -> bool {
        let mut referer: String = String::from("/");
        let path: String = req.url.path().join("/");
        referer.push_str(&path);

        ENDPOINTS_FILTER.contains(&&referer[..])
    }

    fn get_identity(&self, req: &mut Request) -> Result<Identity, String> {
        let connection = req.get_connection()?;

        get_active_identity(&*connection)
    }

    fn check_signature(&self, req: &mut Request) -> Result<bool, String> {

        let hash_raw = self.get_body_hash(req)?;
        let signature_raw = self.get_header(req, "X-LS-SIGNATURE")?;
        let identity_raw = self.get_identity(req)?;
        let hash : &[u8] = hash_raw.as_bytes();
        let signature : &[u8] = signature_raw.as_slice();
        let key : &Rsa = identity_raw.key();

        key.verify_signature(hash, signature)
    }

    fn get_header(&self, req: &mut Request, name : &str) -> Result<Vec<u8>, String> {
        match req.headers.get_raw(name) {
            Some(header) => self.get_first_header_item(header),
            None => Err(format!("Header \"{}\" not found", name))
        }
    }

    fn get_first_header_item(&self, header : &[Vec<u8>]) -> Result<Vec<u8>, String> {
        match header.get(0) {
            Some(value) => Ok(value.clone()),
            None => Err(String::from("Requested header has no content"))
        }
    }

    fn get_body_hash(&self, req: &mut Request) -> Result<String, String> {
        match body_raw!(req) {
            Ok(body) => self.calculate_body_hash(body),
            Err(_) => Err(String::from("Error while parsing HTTP request body as raw data"))
        }
    }

    fn calculate_body_hash(&self, body: Option<String>) -> Result<String, String> {
        match body {
            Some(data) => Ok(sha512(data.as_bytes())),
            None => Err(String::from("No body available"))
        }
    }

}

impl BeforeMiddleware for ProtectedMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("Protection middleware active");
        if self.is_protected_route(req) {
            println!("Route is protected");
            self.process_request(req)
        }
        else {
            Ok(())
        }
    }
}