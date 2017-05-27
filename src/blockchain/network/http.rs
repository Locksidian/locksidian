//! Blockchain networking client.

#![allow(dead_code)]

use hyper::Client;

use error::*;
use blockchain::network::p2p;

use blockchain::identity::Identity;
use blockchain::peer::Peer;

pub struct HttpClient {
    client: Client,
    address: String
}

impl HttpClient {

    pub fn new(client: Client, address: String) -> Self {
        HttpClient {
            client: client,
            address: format!("http://{}", address)
        }
    }

    pub fn from_address(address: String) -> Self {
        HttpClient::new(HttpClient::default_client(), address)
    }
    
    pub fn from_peer(peer: &Peer) -> Self {
        HttpClient::new(HttpClient::default_client(), peer.address())
    }

    fn default_client() -> Client {
        Client::new()
    }
}

impl p2p::Client for HttpClient {
    
    fn register(&self, _: &Identity) -> LocksidianResult<bool> {
        unimplemented!()
    }

    fn get_peers(&self) -> LocksidianResult<Vec<Peer>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_append_http_protocol() {
        let client = HttpClient::from_address(String::from("127.0.0.1"));
        assert_eq!("http://127.0.0.1", client.address);
    }
}