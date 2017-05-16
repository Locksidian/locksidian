//! Blockchain networking client.

#![allow(dead_code)]

use hyper::Client as HttpClient;

use error::*;
use blockchain::network::p2p::P2PClient;

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
            address: address
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

impl P2PClient for Client {
    
    fn register(&self, _: &Identity) -> LocksidianResult<bool> {
        unimplemented!()
    }

    fn get_peers(&self) -> LocksidianResult<Vec<Peer>> {
        unimplemented!()
    }
}