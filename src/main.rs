//! The root crate for the `Locksidian` project.
//!
//! ## Contributors
//!
//! [Valentin Fries](https://www.fries.io) ([GitHub](https://github.com/MrKloan)) & Aurélien Duval
//! ([GitHub](https://github.com/acid-killa666))
//!
//! ## Overview
//!
//! Locksidian is a pure [Rust](https://www.rust-lang.org/) implementation of the
//! [blockchain](https://en.wikipedia.org/wiki/Blockchain_(database)) technology, and was developed
//! as a study project during our first year of Master degree in *Software Architecture*.
//!
//! Our objective is to provide the ability to store and certify the existence and integrity of any
//! JSON document by storing and cryptographically signing them inside of the `Locksidian`
//! blockchain.
//!
//! All the interactions between the peers of the peer-to-peer network is done using the HTTP(S)/1.1
//! protocol through the use of REST APIs.
//!
//! ## Deeper into the rabbit hole
//!
//! Some of the application concepts are explained below in what we could call a specs draft.
//!
//! ### Identity management
//!
//! Run the executable using an existing identity: `locksidian --identity={uuid}`
//!
//! Or define the following environment variable: `LS_IDENTITY={uuid}`
//!
//! The `Identity` structure *could* be defined as follows:
//!
//! ```rust
//! struct Identity {
//!     uuid: UUID,         // Unique identifier for this Identity
//!     keypair: PKey,      // RSA keypair associated to this Identity
//!     signature: String   // Signed UUID
//! }
//! ```
//!
//! If no identity is specified at startup, or if the provided identity is invalid, a new one will
//! be automatically generated. First, a new UUID is generated along with a new 4096 bits RSA keypair.
//! The UUID is signed with the private key in order to provide a value to check for the integrity
//! of this identity.
//!
//! When a new identity is generated, it is automatically stored in the node's registry and flagged
//! as the `default` identity, in order to be selected at startup if no UUID is specified.
//!
//! When an identity is loaded from the registry, thanks to the `default` flag or the `identity`
//! argument/env, its signature is recalculated and compared to the stored signature. If they don't
//! match, it means that this identity is corrupted: one of its UUID, keypair or signature was
//! modified. The node automatically shuts himself down in order to keep the network safe, and this
//! identity is removed from its local registry.
//!
//! When a node emits a request to one of its peer using its REST API, it adds the following HTTP
//! headers to the request: `X-LS-UUID={uuid}`, in order to be identified by its peer; and
//! `X-LS-SIGNATURE={json_payload_signature}`, which corresponds to the request's payload signature,
//! signed using the node's private key. When another node receives this request, it can verify that
//! the request is emitted by the node identified by the provided UUID by checking the payload
//! signature using its peer's public key. If the signature can't be validated, the request is
//! discarded.
//!
//! ### Peer-to-Peer network
//!
//! Run the executable in *peer mode* by specifying an entrypoint: `locksidian --entrypoint={addr}`
//!
//! Or define the following environment variable: `LS_ENTRYPOINT={addr}`
//!
//! The `Peer`structure *could* be defined as follows:
//!
//! ```rust
//! struct Peer {
//!     uuid: UUID,         // Unique identifier for this peer
//!     key: PKey,          // RSA public key
//!     address: String     // HTTP(S) URL with port number
//! }
//! ```
//!
//! The `entrypoint` is the address of any node in a `Locksidian` peer-to-peer network. During the
//! node startup, a request containing the node public key and its address - in order to be reachable -
//! is sent to the `POST /peers/register` endpoint of its `entrypoint`.
//!
//! The entrypoint will initialize a new `Peer` structure using the provided information, and check
//! the `X-LS-SIGNATURE` header value using the provided public key. If the signature is valid, the
//! new peer is added to the entrypoint's local peers registry and the list of all its peers (including
//! himself) is sent back to the new node.
//!
//! Then the entrypoint will broadcast the new peer data to all the peers in the network, by sending
//! the serialized structure on their `POST /peers` endpoint. Each node will - obviously - check that
//! the request comes from one of their peer (again, by validating the payload signature) before
//! adding the new `Peer` structure to their local peers registry.
//!
//! This way, a single peer address is needed to join the peer-to-peer network, and each new peer
//! data are broadcasted to the entire network making it self-sufficient.
//!
//! But a node can also start without specifying an `entrypoint`, in what we'll call a *standalone mode*.
//! When a node starts in standalone mode, it will not try to join any existing peer-to-peer network
//! but will instead be the first `entrypoint` of a new `Locksidian` network! This way, you can
//! easily create at will your own private network, hence your own private `Locksidian` blockchain.
//!
//! When a node starts in *standalone mode*, it has to create a first block in the blockchain called
//! `ORIGIN`, with a sample JSON payload: `{"message": "Hello World!"}`. The `ORIGIN` block will be
//! the anchor of any subsequent block added to the chain.
//!
//! ### Store a JSON document into the blockchain
//!
//! The `Block` structure *could* be defined as follows:
//!
//! ```rust
//! struct Block {
//!     data: String,       // JSON document
//!     bytes: u64,         // Document's size in bytes
//!     hash: String,       // SHA512 data checksum
//!     timestamp: u64,     // Creation timestamp of the block
//!     previous: String,   // Hash of the previous block in the chain
//!     next: String,       // Hash of the next block in the chain
//!     height: u64,        // Block index relative to the main chain
//!     received_at: u64,   // Reception timestamp of the block by the sending peer
//!     received_from: UUID // UUID of the peer from which this block has been received
//! }
//! ```
//!
//! In order to register a new JSON document into the `Locksidian` blockchain, you just have to `POST`
//! it to the `/blocks` endpoint of any node of the peer-to-peer network.
//!
//! The origin of the requests `POST`ed on this endpoints are not checked: anyone can submit a
//! document to the blockchain!
//!
//! A new `Block` structure is initialized with the current timestamp and the JSON document as its
//! `data` field and the document's size in bytes as its `bytes` field.
//!
//! Then the node will calculate the SHA512 checksum of the `data` and store it in the `hash` field.
//! Then it will browse the blockchain, searching for a block of the exact same checksum.
//!
//! If a block *does exists* with the exact same `hash` anywhere in the chain, the node will throw
//! a `409 Conflict` and send the checksum value in the HTTP response.
//!
//! If there is no block with the same checksum in the chain, the remaining field of the `Block`
//! structure are initialized (with `HEAD` the Block representing the current head of the blockchain):
//!
//! ```text
//! block.received_at = block.timestamp
//! block.previous = HEAD.hash
//! block.next = (empty string)
//! HEAD.next = block.hash
//! block.height = HEAD.height + 1
//! block.received_from = (current node UUID)
//! ```
//!
//! The block is then stored in the node's registry and the HEAD reference is updated.
//!
//! Finally, the node will broadcast the newly forged block to all its peers on the peer-to-peer
//! network.
//!
//! ### Block replication 101
//!
//! In order to replicated a block, the following fields of the `Block` structure are sent to the
//! `PUT /blocks` endpoint of a `Locksidian` node : `{data, bytes, hash, timestamp, previous, height}`.
//!
//! Only a peer of the network is allowed to replicate a block: thus the payload signature is validated
//! and a `403 Unauthorized` status is thrown in case of error.
//!
//! The document size in bytes and checksum will be recalculated and a match will be searched in the
//! node's registry. If the recalculated size or hash does not match the provided values, a `400 Bad request`
//! status will be thrown. If a match is found in the node's registry: `409 Conflict`.
//!
//! If the recalculated data are valid and no match is found in the node's registry, the empty
//! structure values are initialized:
//!
//! ```text
//! block.received_at = (current timestamp)
//! block.received_from = (requester's UUID)
//! block.next = (empty string)
//! ```
//!
//! The block referenced by the `previous` field will then be searched in the registry. If it is not
//! present, the new `Block` is added to the registry and the node will request the missing block to
//! its peers (**TODO**: define the method used to automatically request a missing block. Broadcast
//! a message? Check peer by peer until the missing block is found?).
//!
//! If the `previous` block is found and its `next` field is empty, the new `Block` is stored in the
//! registry and the previous block is updated: `previous.next = block.hash`, making the new block
//! a member of the chain.
//!
//! If the `previous` block is found but its `next` field is **not** empty, then the `height` of the
//! block is compared to the `height` of the current `HEAD`. If the new block is too far behind the
//! `HEAD`, it is purely rejected and the registry is *not* updated. Otherwise the new block is
//! nevertheless stored in the registry but the `previous.next` hash is **not** updated, making it
//! an *orphan block* (**TODO**: See `11. Calculations` of the [Bitcoin original document](https://bitcoin.org/bitcoin.pdf)
//! to determine what is the best way to evaluate the chance of a block to keep up the pace of the
//! main chain). This approach is used because, if we consider that the new block has a chance
//! to become part of the future main chain, it will automatically be linked to its previous block
//! when a *prune* of the registry will happen in the future.
//!
//! ### Retrieving a block
//!
//! In order to retrieve a block from the `Locksidian` blockchain, you just have to `GET /blocks/{hash}`
//! on any of the available nodes (anyone can access the data of a block).
//!
//! A request on `GET /blocks/HEAD` will return the current head of the main chain. In order to do
//! that, the block having the `MAX(height)` will be used. If more than one block are returned by
//! this query, the real HEAD is the one which `previous` block's `next` field is itself
//! (`HEAD.previous.next == HEAD`).
//!
//! ### Prune the blockchain!
//!
//! In order to keep the blockchain consistent and to get rid of unused, useless or potentially
//! altered, corrupted or falsified data, the blockchain (i.e. the entire registry of a node)
//! can be prune by making a request on the `DELETE /blocks` endpoint.
//!
//! The prune process is rather simple: all the blocks that are not part of the main chain are
//! discarded (i.e. removed from the node's registry). The **main chain** can be described as the
//! succession of blocks respectively linked by their `previous` and `next` fields, that lies between
//! the `HEAD` and `ORIGIN` blocks.

// Third-party dependencies
extern crate time;
extern crate crypto;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use(router)]
extern crate router;
extern crate iron;
extern crate bodyparser;
extern crate iron_test;

// Project modules
pub mod sec;
mod api;

/// Locksidian entry point.
fn main() {
    let server = api::Server::new(String::from("localhost:8080"));
    server.start(api::router());
}