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
use sec::sha::sha512;
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

    fn process_request(&self, req: &mut Request) -> Result<String, String> {
        if self.has_identity(req) && self.check_signature(req){
            return Ok(String::from("done"));
        }
        else {
            return Err(String::from("Forbidden"));
        }
    }

    fn is_protected_route(&self, req: &mut Request) -> bool {
        let mut referer: String = String::from("/");
        let path: String = req.url.path().join("/");
        referer.push_str(&path);
        return ENDPOINTS_FILTER.contains(&&referer[..]);
    }

    fn has_identity(&self, req: &mut Request) -> bool {
        match req.get_connection() {
            Ok(connection) => match get_active_identity(&connection) {
                Ok(identity) => return true,
                Err(err) => return false
            },
            Err(err) => return false
        }

    }

    fn check_signature(&self, req: &mut Request) -> bool {
        let hash = self.get_body_hash(req);
        return match req.headers.get_raw("X-LS-SIGNATURE") {
            Some(signature) => self.header_to_string(signature).eq(&hash),
            None => false
        }

    }

    fn header_to_string(&self, header : &[Vec<u8>] ) -> String {

        let buf = self.get_first_header_item(header);

        return match String::from_utf8(buf) {
            Ok(data) => data,
            Err(err) => String::from("")
        }
    }

    fn get_first_header_item(&self, header : &[Vec<u8>]) -> Vec<u8> {

        return match header.get(0) {
            Some(value) => value.clone(),
            None => Vec::new()
        }
    }

    fn get_body_hash(&self, req: &mut Request) -> String {
        let empty_str : String = String::from("");
        match body_raw!(req) {
            Ok(result) => match result {
                Some(data) => return sha512(data.as_bytes()),
                None => return empty_str
            },
            Err(err) => return empty_str
        }
    }

}

impl BeforeMiddleware for ProtectedMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("Protection middleware active");
        if self.is_protected_route(req) {
            println!("Route is protected");
            return match self.process_request(req) {
                Ok(data) => Ok(()),
                Err(err) => Err(IronError::new(StringError("Forbidden".to_string()), Forbidden))
            }
        }
        else {
            return Ok(());
        }
    }
}