//! Peer Repository module.

use persistence::prelude::*;
use blockchain::peer::Peer;

table! {
    peers(identity) {
        identity -> VarChar,
        key -> VarChar,
        address -> VarChar,
        last_sent -> Integer,
        last_recv -> Integer,
    }
}

#[derive(
    Debug, Clone,
    Queryable, Insertable, AsChangeset
)]
#[table_name = "peers"]
pub struct PeerEntity {
    pub identity: String,
    pub key: String,
    pub address: String,

    pub last_sent: i32,
    pub last_recv: i32
}

impl PeerEntity {

    /// Instantiate a new `PeerEntity` based on the provided `Peer`.
    pub fn new(peer: &Peer) -> Result<Self, String> {
        Ok(PeerEntity {
            identity: peer.identity(),
            key: peer.key_to_hex()?,
            address: peer.address(),
            
            last_sent: peer.last_sent() as i32,
            last_recv: peer.last_recv() as i32
        })
    }
}

pub struct PeerRepository<'pool> {
    connection: &'pool SqliteConnection
}

impl<'pool> PeerRepository<'pool> {

    /// Instantiate a new `PeerRepository` whose lifetime is bound to its pooled connection.
    pub fn new(connection: &SqliteConnection) -> PeerRepository {
        PeerRepository {
            connection: connection
        }
    }
}

crud_repository!(peers, PeerEntity, String, identity, PeerRepository<'pool>);