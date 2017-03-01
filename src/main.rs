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
//! ## Installation
//!
//! The `Locksidian` blockchain is available to download as a [Docker](https://www.docker.com/) image,
//! with 2 different tags:
//!
//!  - `dev`: the latest, in-development and potentially unstable, version;
//!  - `master`: the current, stable, `Locksidian` version.
//!
//! ```bash
//! $ docker login registry.gitlab.com
//! $ docker pull registry.gitlab.com/locksidian/locksidian:master
//! $ docker run --name locksidian -d registry.gitlab.com/locksidian/locksidian:master
//! ```
//!
//! ## Sources
//!
//! The canonical location for the `Locksidian` project is on [GitLab](https://gitlab.com/locksidian)
//! (the access is currently restricted to authorized users only).
//!
//! With each major releases, the project will be updated on GitHub under the [Locksidian Organization](https://github.com/locksidian).
//!
//! ## Deeper into the rabbit hole
//!
//! Some of the application concepts are explained below in what we could call a *specs draft*.
//!
//! ### Identity management
//!
//! Run the executable using an existing identity: `locksidian --identity={hash}`, or define the
//! following environment variable: `LS_IDENTITY={hash}`.
//!
//! Generate a new identity using an existing PEM-encoded RSA keypair: `locksidian --identity-import="/path/to/keypair.pem"`.
//!
//! Generate a new identity from scratch by specifying the keypair size (defaults to 4096) : `locksidian --identity-new={keypair_size}`.
//!
//! The `Identity` structure *could* be defined as follows:
//!
//! ```rust
//! struct Identity {
//!     hash: String,       // Identity hash generated from the Public Key
//!     keypair: PKey,      // RSA keypair associated to this Identity
//! }
//! ```
//!
//! If no identity is specified at startup, or if the provided identity is invalid, a new one will
//! be automatically generated.
//!
//! First, a new RSA keypair will be generated. If the `--identity-new` flag is set with a value
//! respecting the constraint: `{value} >= 2048 && {value} % 1024 == 0`, its value is used as the
//! size of the generated keypair. If no value is specified or if the specified value violates the
//! constraint, a default value of `4096` is used.
//!
//! Once the keypair is generated, the Public Key is extracted in a PEM-encoded format (jump directly
//! to this step if the `--identity-import` flag is set and points to a valid PEM-encoded RSA keypair).
//! Its checksum will be computed in order to create a unique identifier for this identity:
//! `RIPEMD160(SHA512(pem_public_key))`. The resulting hash, which is 40 hexadecimal characters long,
//! is stored in the `hash` attribute of the `Identity` structure.
//!
//! When a new identity is generated, it is automatically stored in the node's registry and flagged
//! as the `active` identity, in order to be selected at startup if no identity hash is specified.
//!
//! When an identity is loaded from the registry, thanks to the `active` attribute or the `--identity`
//! flag, its Public Key hash is recalculated and compared to the stored hash. If they don't
//! match, it means that this identity is corrupted: one of its hash or keypair was modified.
//! The node automatically shuts himself down in order to keep the network safe, and this
//! identity is removed from its local registry.
//!
//! When a node emits a request to one of its peer using its REST API, it adds the following HTTP
//! headers to the request: `X-LS-IDENTITY={hash}`, in order to be identified by its peer; and
//! `X-LS-SIGNATURE={json_payload_signature}`, which corresponds to the request's payload signature,
//! signed using the node's private key. When another node receives this request, it can verify that
//! the request is emitted by the node identified by the provided identity hash by checking the payload
//! signature using its peer's public key. If the signature can't be validated, the request is
//! discarded.
//!
//! ### Peer-to-Peer network
//!
//! Run the executable in *peer mode* by specifying an entrypoint: `locksidian --entrypoint={addr}`,
//! or define the following environment variable: `LS_ENTRYPOINT={addr}`
//!
//! The `Peer`structure *could* be defined as follows:
//!
//! ```rust
//! struct Peer {
//!     identity: String,   // Unique identifier for this peer
//!     key: PKey,          // RSA public key
//!     address: String     // HTTP(S) URL with port number
//! }
//! ```
//!
//! The `entrypoint` is the address of any node in a `Locksidian` peer-to-peer network. During the
//! node startup, a request containing the node public key and its address - in order to be reachable -
//! is sent to the `POST /peers/register` endpoint of its `entrypoint`.
//!
//! The API request *could* be defined as follows:
//!
//! ```json
//! Content-Type: "application/json"
//! X-LS-IDENTITY: {identity.hash}
//! X-LS-SIGNATURE: {json_payload_signature}
//! {
//!     "key": {identity.keypair.public_key},
//!     "address": {node.address}
//! }
//! ```
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
//! ### Store a JSON document inside the blockchain
//!
//! The `Block` structure *could* be defined as follows:
//!
//! ```rust
//! struct Block {
//!     data: String,           // JSON document
//!     bytes: u64,             // Document's size in bytes
//!     hash: String,           // SHA512 data checksum
//!     timestamp: u64,         // Creation timestamp of the block
//!
//!     nonce: u32,             // Proof of Work solution
//!     author: String,         // Identity hash of the block author
//!     signature: String,      // Signature of the block's hash using the author Private Key
//!
//!     height: u64,            // Block index relative to the main chain
//!     previous: String,       // Hash of the previous block in the chain
//!     next: String,           // Hash of the next block in the chain
//!
//!     received_at: u64,       // Reception timestamp of the block by the sending peer
//!     received_from: String   // Identity hash of the peer from which this block has been received
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
//! If there is no block with the same checksum in the chain, the following fields of the `Block`
//! structure are initialized (with `HEAD` the Block representing the current head of the blockchain):
//!
//! ```text
//! block.received_at = block.timestamp
//! block.previous = HEAD.hash
//! block.next = (empty string)
//! HEAD.next = block.hash
//! block.height = HEAD.height + 1
//! block.received_from = (current node identity)
//! ```
//!
//! But in order to add this new block to the blockchain, the node has to solve a [Proof of Work](https://en.bitcoin.it/wiki/Proof_of_work)
//! challenge. In `Locksidian`, the **Proof of Work** is implemented the following way:
//!
//!  - The 8 [most significant bits](https://en.wikipedia.org/wiki/Most_significant_bit) (the first
//!    byte) of the block's hash is used to compute the PoW objective: the number of leading `0`
//!    required in the PoW hash. The number is calculated as follows: `{byte} % 5 + 1`, which gives
//!    us a window of `1` to `5` leading zeros, which feels a good compromise for this prototype.
//!
//!  - The `nonce`, an unsigned 32 bit number, is initialized to `0` and appended to the end of the
//!    block's data (example: `{"message": "Hello World!"}0`). The SHA512 checksum of this payload
//!    is computed and the number of leading zeros is counted. If it matches the PoW objective, the current
//!    `nonce` value is stored in the structure. If the PoW is not satisfied, the `nonce` is incremented
//!    and the payload checksum is recomputed; loop until the PoW is solved.
//!
//! Once the PoW is solved, the block hash is signed using the `Identity` RSA Private Key to prove
//! the ownership of the block and the remaining fields of the `Block` structure are initialized:
//!
//! ```text
//! block.nonce = {nonce}
//! block.author = {current node identity}
//! block.signature = {block.hash signed using the node's private key}
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
//! `PUT /blocks` endpoint of a `Locksidian` node:
//!
//! ```json
//! {
//!     "data": {Block's data},
//!     "bytes": {Data size in bytes},
//!     "hash": {Block's hash},
//!     "timestamp": {Block's timestamp},
//!
//!     "nonce": {Proof of Work solution},
//!     "author": {Identity of the block's author},
//!     "signature": {Signature proving ownership},
//!
//!     "previous": {Previous block hash},
//!     "height": {Block height}
//! }
//! ```
//!
//! Only a peer of the network is allowed to replicate a block: thus the payload signature is validated
//! and a `403 Unauthorized` status is thrown in case of error.
//!
//! The document size in bytes and checksum will be recalculated and a match will be searched in the
//! node's registry. If the recalculated size or hash does not match the provided values, a `400 Bad request`
//! status will be thrown. If a match is found in the node's registry: `409 Conflict`.
//!
//! Then the **Proof of Work** is verified. Using the block's hash, the required number of zeros is
//! recalculated. The provided `nonce` is appended to the data and their checksum is computed. If the
//! resulting hash effectively starts with the correct number of zeros, the PoW is validated. Else,
//! the block is rejected.
//!
//! If the recalculated data are valid, no match is found in the node's registry and the supplied
//! Proof of Work is correct, the empty structure values are initialized:
//!
//! ```text
//! block.received_at = (current timestamp)
//! block.received_from = (requester's identity hash)
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
//! `HEAD` (i.e. more than **5 blocks behind** the current `HEAD`), it is purely rejected and the
//! registry is *not* updated. Otherwise the new block is nevertheless stored in the registry but
//! the `previous.next` hash is **not** updated, making it an *orphan block*. This approach is used
//! because if we consider that the new block has a chance to become part of the future main chain,
//! it will automatically be linked to its previous block when a *prune* of the registry will happen
//! in the future.
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
//! can be pruned by making a request on the `DELETE /blocks` endpoint.
//!
//! The prune process is rather simple: all the blocks that are not part of the main chain are
//! discarded (i.e. removed from the node's registry). The **main chain** can be described as the
//! succession of blocks respectively linked by their `previous` and `next` fields, that lie between
//! the `HEAD` and `ORIGIN` blocks.

// Third-party dependencies
extern crate getopts;
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
mod opts;
pub mod sec;

mod cli;
mod api;

const PACKAGE: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

/// Locksidian entry point.
fn main() {
    let matches = opts::init();
    cli::handle(matches);
}