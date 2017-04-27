//! Block Repository module.

use persistence::prelude::*;
use blockchain::block::Block;

use sec::hex::ToHex;

table! {
    blocks(hash) {
        data -> VarChar,

        data_hash -> VarChar,
        signature -> VarChar,
        timestamp -> Integer,
        nonce -> Integer,
        previous -> VarChar,

        hash -> VarChar,
        height -> Integer,
        next -> VarChar,
        author -> VarChar,
        received_at -> Integer,
        received_from -> VarChar,
    }
}

#[derive(
    Debug, Clone,
    Queryable, Insertable, AsChangeset
)]
#[table_name = "blocks"]
pub struct BlockEntity {
    pub data: String,

    pub data_hash: String,
    pub signature: String,
    pub timestamp: i32,
    pub nonce: i32,
    pub previous: String,

    pub hash: String,
    pub height: i32, 
    pub next: String,
    pub author: String,
    pub received_at: i32,
    pub received_from: String
}

impl BlockEntity {

    /// Instantiate a new `BlockEntity` based on the given `Block
    pub fn new(block: &Block) -> Self {
        BlockEntity {
            data: block.data(),

            data_hash: block.data_hash(),
            signature: block.signature().to_hex(),
            timestamp: block.timestamp() as i32,
            nonce: block.nonce() as i32,
            previous: block.previous(),

            hash: block.hash(),
            height: block.height() as i32,
            next: block.next(),
            author: block.author(),
            received_at: block.received_at() as i32,
            received_from: block.received_from()
        }
    }
}

pub struct BlockRepository<'pool> {
    connection: &'pool SqliteConnection
}

impl<'pool> BlockRepository<'pool> {

    /// Instantiate a new `BlockRepository` whose lifetime is bound to its pooled connection.
    pub fn new(connection: &SqliteConnection) -> BlockRepository {
        BlockRepository {
            connection: connection
        }
    }

    /// Select a `BlockEntity` using its `data_hash` rather than its `hash` primary key.
    ///
    /// Method used when crawling the blockchain for an existing document.
    fn get_by_data_hash(&self, data_hash: String) -> Option<BlockEntity> {
        match blocks::table.filter(blocks::data_hash.eq(data_hash)).first(self.connection) {
            Ok(entity) => Some(entity),
            Err(_) => None
        }
    }

    /// Select the `BlockEntity` with the greater `height` value.
    ///
    /// TODO: Select the `BlockEntity` with the greater `height` value and having its `previous` block linked back to it through its `next` column,
    /// in order to avoid conflicts if a fork were to occur.
    fn get_head(&self) -> Option<BlockEntity> {
        match blocks::table.order(blocks::height.desc()).first(self.connection) {
            Ok(entity) => Some(entity),
            Err(_) => None
        }
    }
}

crud_repository!(blocks, BlockEntity, String, hash, BlockRepository<'pool>);