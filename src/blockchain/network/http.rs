//! Blockchain networking client.

#![allow(dead_code)]

use error::*;
use api::client::prelude::*;

use hyper::header::{Headers, ContentType};
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use blockchain::network::p2p;
use blockchain::peer::{Peer, PeerDto};

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
	
	fn headers(&self) -> Headers {
		let mut headers = Headers::new();
		headers.set(ContentType(Mime(
			TopLevel::Application, SubLevel::Json,
			vec![(Attr::Charset, Value::Utf8)])
		));
		
		headers
	}
}

impl p2p::Client for HttpClient {
    
    fn register(&self, peer: &Peer) -> LocksidianResult<Peer> {
        let url = format!("{}/peers/register", self.address.clone());
		let dto = PeerDto::new(&peer)?;
		
		match ::serde_json::to_string(&dto) {
			Ok(json) =>  match self.client.post(&url).headers(self.headers()).body(&json).send() {
				Ok(mut res) => match client_body!(res, PeerDto) {
					Ok(dto) => dto.to_peer(),
					Err(err) => Err(LocksidianError::from_err(err))
				},
				Err(err) => Err(LocksidianError::from_err(err))
			},
			Err(err) => Err(LocksidianError::from_err(err))
		}
    }

    fn get_peers(&self) -> LocksidianResult<Vec<Peer>> {
        let url = format!("{}/peers", self.address.clone());
		
        match self.client.get(&url).send() {
            Ok(mut res) => match client_body!(res, Vec<PeerDto>) {
				Ok(dto) => {
					let peers: Vec<Peer> = dto.iter()
						.map(|dto| dto.to_peer())
						.filter(|peer| peer.is_ok())
						.map(|peer| peer.unwrap())
						.collect();
					
					Ok(peers)
				},
				Err(err) => Err(LocksidianError::from_err(err))
			},
            Err(err) => Err(LocksidianError::from_err(err))
        }
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