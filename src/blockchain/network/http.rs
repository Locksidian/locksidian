//! Blockchain networking client.

#![allow(dead_code)]

use hyper::Client as HttpClient;

use error::*;
use blockchain::network::p2p;

use blockchain::identity::Identity;
use blockchain::peer::Peer;

pub struct Client {
    client: HttpClient,
    address: String
}

impl Client {

    pub fn new(client: HttpClient, address: String) -> Self {
        Client {
            client: client,
            address: format!("http://{}", address)
        }
    }

    pub fn from_address(address: String) -> Self {
        Client::new(Client::default_client(), address)
    }
    
    pub fn from_peer(peer: &Peer) -> Self {
        Client::new(Client::default_client(), peer.address())
    }

    fn default_client() -> HttpClient {
        HttpClient::new()
    }
}

impl p2p::Client for Client {
    
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
        let client = Client::from_address(String::from("127.0.0.1"));
        assert_eq!("http://127.0.0.1", client.address);
    }
}