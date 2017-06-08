//! Block data transfer objects.

#![allow(dead_code)] // TODO: Remove this when implementing the block replication feature (see issue #19)

use sec::hex::ToHex;

use blockchain::block::Block;
use blockchain::identity::Identity;

/// Simple `BlockDto` representing the entire `Block` structure.
///
/// Used to display all the data of the given `Block`.
#[derive(
	Debug, Clone,
	Serialize, Deserialize
)]
pub struct BlockDto {
    data: String,

    data_hash: String,
    signature: String,
    timestamp: u64,
    nonce: u32,
    previous: String,

    hash: String,
    height: u64,
    next: String,
    author: String,
    received_at: u64,
    received_from: String
}

impl BlockDto {

    /// Instantiate a new `BlockDto` based on the given `Block`.
    pub fn new(block: &Block) -> Self {
        BlockDto {
            data: block.data(),

            data_hash: block.data_hash(),
            signature: block.signature().to_hex(),
            timestamp: block.timestamp(),
            nonce: block.nonce(),
            previous: block.previous(),

            hash: block.hash(),
            height: block.height(),
            next: block.next(),
            author: block.author(),
            received_at: block.received_at(),
            received_from: block.received_from()
        }
    }
}

/// DTO used for `Block` replication requests.
///
/// The fields `next`, `received_at` and `received_from` are omitted because they are
/// linked to the context of a node.
#[derive(
	Debug, Clone,
	Serialize, Deserialize
)]
pub struct BlockReplicationDto {
    pub data: String,

    pub data_hash: String,
    pub signature: String,
    pub timestamp: u64,
    pub nonce: u32,
    pub previous: String,

    pub hash: String,
    pub height: u64,
    pub author: String,
    pub received_from: String
}

impl BlockReplicationDto {

    /// Instantiate a new `BlockReplicationDto` based on the given `Block`.
    pub fn new(block: &Block, current_identity: &Identity) -> Self {
        BlockReplicationDto {
            data: block.data(),

            data_hash: block.data_hash(),
            signature: block.signature().to_hex(),
            timestamp: block.timestamp(),
            nonce: block.nonce(),
            previous: block.previous(),

            hash: block.hash(),
            height: block.height(),
            author: block.author(),
            received_from: current_identity.hash()
        }
    }
}