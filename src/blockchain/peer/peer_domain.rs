//! Peer Domain module.

use error::*;
use sec::rsa::Rsa;
use sec::hex::*;

use blockchain::peer::PeerEntity;
use blockchain::identity::identity_cli::compute_key_hash;

pub struct Peer {
    identity: String,
    key: Rsa,
    address: String,

    last_sent: u64,
    last_recv: u64
}

impl Peer {

    /// Instantiate a new `Peer` based on its identity, public key and address.
    pub fn new(key: String, address: String) -> LocksidianResult<Self> {
        match key.from_hex() {
            Ok(pem) => {
                let rsa = Rsa::from_public_key(pem.as_slice())?;

                Ok(Peer {
                    identity: compute_key_hash(&rsa)?,
                    key: rsa,
                    address: address,
                    last_sent: 0,
                    last_recv: 0
                })
            },
            Err(err) => Err(LocksidianError::from_err(err))
        }
    }

    /// Instantiate a new `Peer` from the given `PeerEntity`, consuming the entity instance.
    pub fn from_entity(entity: &PeerEntity) -> LocksidianResult<Self> {
        let mut peer = Peer::new(entity.key.clone(), entity.address.clone())?;
        peer.last_sent = entity.last_sent as u64;
        peer.last_recv = entity.last_recv as u64;

        Ok(peer)
    }

    /// `identity` getter.
    pub fn identity(&self) -> String {
        self.identity.clone()
    }

    /// `key` getter.
    pub fn key(&self) -> &Rsa {
        &self.key
    }

    pub fn key_to_hex(&self) -> LocksidianResult<String> {
		let pem = self.key.export_public_key()?;
		let hex = pem.to_hex();
		
		Ok(hex)
	}

    /// `address` getter.
    pub fn address(&self) -> String {
        self.address.clone()
    }

    /// `last_sent` getter.
    pub fn last_sent(&self) -> u64 {
        self.last_sent
    }

    /// `last_sent` setter.
    pub fn set_last_sent(&mut self, timestamp: u64) {
        self.last_sent = timestamp;
    }

    /// `last_recv` getter.
    pub fn last_recv(&self) -> u64 {
        self.last_recv
    }

    /// `last_recv` setter.
    pub fn set_last_recv(&mut self, timestamp: u64) {
        self.last_recv = timestamp;
    }
}