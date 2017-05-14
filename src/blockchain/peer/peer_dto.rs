//! Peer Data Transfer Object module.

use blockchain::peer::Peer;

#[derive(
	Debug, Clone,
	Serialize, Deserialize
)]
pub struct PeerDto {
    identity: String,
    key: String,
    address: String
}

impl PeerDto {

    /// Instantiate a new `PeerDto` based on the given `Peer`.
    pub fn new(peer: &Peer) -> Result<Self, String> {
        Ok(PeerDto {
            identity: peer.identity(),
            key: peer.key_to_hex()?,
            address: peer.address()
        })
    }

    /// Instantiate a new `Peer` based on this DTO instance.
    pub fn to_peer(&self) -> Result<Peer, String> {
        Peer::new(self.identity.clone(), self.key.clone(), self.address.clone())
    }
}