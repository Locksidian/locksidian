//! Peer-to-Peer networking trait.

use error::*;

use blockchain::peer::Peer;
use blockchain::block::Block;
use blockchain::identity::Identity;

/// Peer-to-Peer client trait definition.
pub trait Client {

    /// Register the specified `Identity` on this Peer-to-Peer client.
    fn register(&self, peer: &Peer) -> LocksidianResult<Peer>;

    /// Get the list of all `Peer`s registered on this Peer-to-Peer client.
    fn get_peers(&self) -> LocksidianResult<Vec<Peer>>;
    
    /// Replicate the specified `Block` to this Peer-to-Peer client.
    fn replicate(&self, block: &Block, identity: &Identity) -> LocksidianResult<()>;
    
    /// Propagate the `Block` through a list of `Peer`s.
    fn propagate(block: &Block, identity: &Identity, peers: Vec<Peer>) -> LocksidianResult<()>;
}