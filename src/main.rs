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
//! as a study project during our first year of Master's degree in *Software Architecture*.
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
//! ### From sources
//!
//! ```bash
//! $ docker build -t locksidian:latest .
//! $ docker run --name locksidian -v .:/opt/locksidian -p 8080:8080 -d locksidian:latest
//! ```
//!
//! Or alternatively using Docker Compose:
//!
//! ```bash
//! $ docker-compose up -d
//! ```
//!
//! ### From precompiled binaries
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
//! $ docker run --name locksidian -v .:/opt/locksidian -p 8080:8080 -d registry.gitlab.com/locksidian/locksidian:master
//! ```
//!
//! ## Sources
//!
//! The canonical location for the `Locksidian` project is on [GitLab](https://gitlab.com/locksidian)
//! (the access is currently restricted to the development team members only).
//!
//! With each major releases, the project will be updated on GitHub under the
//! [Locksidian Organization](https://github.com/locksidian).
//!
//! ## Specifications
//!
//! Some of the application concepts are explained below in what we could call a *specs draft*.
//!
//! ### Identity management
//!
//! Run the executable using an existing identity: `locksidian --identity={hash}`, or define the
//! environment variable `LS_IDENTITY={hash}` to set an identity as *active*.
//!
//! Generate a new identity using an existing PEM-encoded RSA keypair:
//! `locksidian --identity-import="/path/to/keypair.pem"`.
//!
//! Generate a new identity from scratch by specifying the keypair size (defaults to 4096):
//! `locksidian --identity-new={keypair_size}`.
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
//! In order to start the `Locksidian` service, run the executable by specifying a listening address:
//! `locksidian --daemon={listen_addr}`, or define the following environment variable: `LS_DAEMON={listen_addr}`.
//!
//! Run the executable in *peer mode* by specifying an entrypoint using `--entrypoint={addr}`,
//! or define the following environment variable: `LS_ENTRYPOINT={addr}`
//!
//! The `Peer`structure *could* be defined as follows:
//!
//! ```rust
//! struct Peer {
//!     identity: String,   // Unique identifier for this peer
//!     key: PKey,          // RSA public key
//!     address: String,    // HTTP(S) URL with port number
//!     last_sent: u64,     // Timestamp of the last time data were sent to this peer
//!     last_recv: u64      // Timestamp of the last time data were received from this peer
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
//!     "address": {node.address},
//!     "height": {HEAD.height}
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
//! data are sent to the entire network making it self-sufficient.
//!
//! If no entrypoint is specified at startup but the current node already has some peer addresses
//! stored in its local registry, it will try to contact them one by one until its request to join
//! the network is accepted.
//!
//! Beside of propagating the new peer to the network, the entrypoint node will use the `height`
//! attribute of the new node to send him the next block that exists in the blockchain, if one exists.
//! This way, a node that could have been disconnected for any period of time will still be able to
//! catch up with its peers by fetching the missing blocks of its local registry.
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
//!     data: String,           // JSON document                            | Block data
//!
//!     data_hash: String,      // SHA512 data checksum                     | Block Header
//!     signature: String,      // Signature of the data_hash               |
//!     timestamp: u64,         // Creation timestamp of the block          |
//!     nonce: u32,             // Proof of Work solution                   |
//!     previous: String,       // Hash of the previous block in the chain  |
//!
//!     hash: String,           // SHA512 Block Header checksum                                         | Block metadata
//!     height: u64,            // Block index relative to the main chain                               |
//!     author: String          // Identity hash of the block author                                    |
//!     next: String,           // Hash of the next block in the chain                                  |
//!     received_at: u64,       // Reception timestamp of the block by the sending peer                 |
//!     received_from: String,  // Identity hash of the peer from which this block has been received    |
//! }
//! ```
//!
//! In order to register a new JSON document into the `Locksidian` blockchain, you just have to `POST`
//! it to the `/blocks` endpoint of any node of the peer-to-peer network.
//!
//! By default, the origin of the requests `POST`ed on this endpoints are not checked: anyone can
//! submit a document to the blockchain!
//!
//! A new `Block` structure is initialized with the current `timestamp`, the JSON document as its
//! `data` field and the document's SHA512 checksum as its `data_hash` field.
//!
//! The node will then browse the blockchain, searching for a block of the exact same checksum.
//! If a block *does* exists with the exact same `data_hash` anywhere in the chain, the node will throw
//! a `409 Conflict` and send the existing block `hash` in the HTTP response.
//!
//! If there is no block with the same `data_hash` checksum in the chain, the following fields of the `Block`
//! structure are initialized (with `HEAD` the Block representing the current head of the blockchain):
//!
//! ```text
//! block.signature = {block.data_hash signed using the node's private key}
//! block.previous = HEAD.hash
//! block.next = (empty string)
//!
//! block.height = HEAD.height + 1
//! block.received_at = block.timestamp
//! block.received_from = (current node identity)
//! block.author = (current node identity)
//! ```
//!
//! But in order to add this new block into the blockchain, the node has to solve a
//! [Proof of Work](https://en.bitcoin.it/wiki/Proof_of_work) challenge.
//!
//! As you may have understood, all the checksum in `Locksidian` are computed using the SHA512
//! hashing algorithm - [which is faster than SHA256 on 64-bit devices](http://crypto.stackexchange.com/questions/26336/sha512-faster-than-sha256).
//! The base 10 value of a SHA512 hash could (theoretically) exist in a range of `0` to `2^512`.
//! As the main objective of the `Locksidian` blockchain is to allow anyone to store JSON documents
//! of *any size*, we thought about using the size in bytes of the data stored in a block in order to
//! determine the difficulty of its mining process.
//!
//! In `Locksidian`, the **Proof of Work** is implemented in the following way:
//!
//!  - The size (in bytes) of the block's data is divided by `32` and then floored. As an example,
//!    if the JSON document is composed of 135 chars, `135/32 ~= 4.22 = 4`. This result is then used
//!    to *decrease* the number of bits used to calculate the PoW *target*. In our example, a result
//!    of `4` will produce `512 - 4 = 508` bits. At the end of this first step, the Proof of Work
//!    target value is `2^508`. This mechanism is used to prevent malevolent nodes or individuals to
//!    spam the network with huge documents, as the computation power required to mine the block
//!    will increase exponentially every `32` bytes.
//!
//!  - The `nonce`, an unsigned 32 bit number, is initialized to `0` and stored in the Block Header
//!    whose SHA512 checksum is computed. If the checksum value is lower than the PoW target, then
//!    the current `nonce` value is stored in the structure. If the PoW is not satisfied, the `nonce`
//!    is incremented and the payload checksum is recomputed; loop until the PoW is solved.
//!
//! Once the PoW is solved, the nonce is stored in the Block Header whose SHA512 checksum is computed
//! and stored into the Block Metadata. Finally, the `next` field of the current HEAD block is updated:
//!
//! ```text
//! block.nonce = {nonce}
//! block.hash = {Block Header SHA512 checksum}
//!
//! HEAD.next = block.hash
//! ```
//!
//! The block is then stored in the node's registry and the HEAD reference is updated.
//!
//! Finally, the node will broadcast the newly forged block to all its peers on the peer-to-peer
//! network.
//!
//! ### "Protecting" your node
//!
//! As explained earlier, anyone can publish any JSON document to a `Locksidian` node... *by default*.
//!
//! In fact, you have the possibility to "protect" your node using either the `--protected` command
//! line argument or the `LS_PROTECTED` environment variable.
//!
//! When running in protected mode, the node will check for a valid signature matching its current
//! `Identity` when receiving a new JSON document on its `/blocks` endpoint.
//!
//! If there is no signature provided or if the signature does not match, a `403 Unauthorized` HTTP
//! status will be returned to the client.
//!
//! ### Block replication 101
//!
//! In order to replicated a block, the following fields of the `Block` structure are sent to the
//! `PUT /blocks` endpoint of a `Locksidian` node:
//!
//! ```json
//! {
//!     "data": {Block's data},                     | Block Data
//!
//!     "data_hash": {Data checksum},               | Block Header
//!     "signature": {data_hash signature},         |
//!     "timestamp": {Block's timestamp},           |
//!     "nonce": {Proof of Work solution},          |
//!     "previous": {Previous block hash},          |
//!
//!     "hash": {Block Header checksum}             | Block Metadata
//!     "height": {Block height},                   |
//!     "author": {Identity of the block's author}  |
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
//! block.next = (empty string)
//! block.received_at = (current timestamp)
//! block.received_from = (requester's identity hash)
//! ```
//!
//! The block referenced by the `previous` field will then be searched in the registry. If it is not
//! present, the new `Block` is added to the registry and the node will request the missing block to
//! its peers.
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

extern crate num;
extern crate num_bigint;

extern crate openssl;
extern crate crypto;
extern crate rustc_serialize;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate hyper;

#[macro_use(router)]
extern crate router;
extern crate iron;
extern crate bodyparser;
extern crate iron_test;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate r2d2;
extern crate r2d2_diesel;

// Project modules
mod opts;
pub mod sec;

mod cli;
#[macro_use]
mod persistence;
mod api;

mod blockchain;

use std::process::exit;

/// Process executed successfully
const EXIT_SUCCESS: i32 = 0;

/// An error occured during runtime which caused the process to stop
const EXIT_FAILURE: i32 = 1;

/// Package name
const PACKAGE: &'static str = env!("CARGO_PKG_NAME");
/// Current version
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// Package description
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
/// Package authors
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

/// Locksidian entry point.
fn main() {
    match opts::init() {
        Ok(matches) => match cli::handle(matches) {
            Ok(success) => {
                println!("{}", success);
                exit(EXIT_SUCCESS);
            },
            Err(msg) => {
                println!("{}", msg);
                exit(EXIT_FAILURE);
            }
        },
        Err(msg) => {
            println!("{}\n\n{}", msg, opts::usage());
            exit(EXIT_FAILURE);
        }
    }
}