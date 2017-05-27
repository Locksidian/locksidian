//! Peer-to-Peer networking trait.

use error::*;

use blockchain::peer::Peer;

/// Peer-to-Peer client trait definition.
pub trait Client {

    /// Register the specified `Identity` on this Peer-to-Peer client.
    fn register(&self, peer: &Peer) -> LocksidianResult<Peer>;

    /// Get the list of all `Peer`s registered on this Peer-to-Peer client.
    fn get_peers(&self) -> LocksidianResult<Vec<Peer>>;
}