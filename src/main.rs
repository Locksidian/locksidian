//! The root crate for the `Locksidian` project.
//!
//! ## Contributors
//!
//! [Valentin Fries](https://www.fries.io) ([GitHub](https://github.com/MrKloan)) & Aurélien Duval
//! ([GitHub](https://github.com/acid-killa666))
//!
//! ## Overview
//!
//! Locksidian offers the ability to store and certify the existence and integrity of any
//! JSON document using a REST API. The documents are cryptographically signed and stored using a
//! *blockchain* data structure distributed across a peer-to-peer network.
//!
//! Locksidian was developed as a study project during our first year of Master's degree in
//! *Software Architecture* using the language [Rust](https://www.rust-lang.org/).
//!
//! *Shard*, the Locksidian [Progressive Web App](https://developers.google.com/web/progressive-web-apps/),
//! is available at: [shard.fries.io](http://shard.fries.io/)
//!
//! The slides of the project presentation are available on [Google Drive](https://docs.google.com/presentation/d/1FumdQ6knop6-JQBaDjnq2sM8Oj5-1EI8OtxyQlNKbJU/).
//!
//! The latest code coverage report generated for the *master* branch can be found here: [locksidian.fries.io/coverage](http://locksidian.fries.io/coverage/)
//!
//! The Locksidian CLI documentation can be found here: [fn.main](fn.main.html)
//!
//! ## Installation
//!
//! ### From sources (executable)
//!
//! ```bash
//! $ cargo build --release
//! $ mv target/release/locksidian /usr/bin/locksidian
//! $ locksidian -i $(locksidian --identity-new 4096) -d 0.0.0.0:8080
//! ```
//!
//! ### From sources (Docker)
//!
//! ```bash
//! $ docker build -t locksidian:latest .
//! $ docker run --name locksidian -v .:/root/.locksidian -p 8080:8080 -d locksidian:latest
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
//! $ docker run --name locksidian -v .:/root/.locksidian -p 8080:8080 -d registry.gitlab.com/locksidian/locksidian:master
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
//! ### Identity management
//!
//! Run the executable using an existing identity: `locksidian --identity={hash}` to set an identity
//! as *active*.
//!
//! Generate a new identity using an existing PEM-encoded RSA keypair:
//! `locksidian --identity-import="/path/to/keypair.pem"`.
//!
//! Generate a new identity from scratch by specifying the keypair size (defaults to 4096):
//! `locksidian --identity-new={keypair_size}`.
//!
//! The `Identity` structure is defined as follows:
//!
//! ```rust
//! struct Identity {
//!     hash: String,   // Identity hash generated from the Public Key
//!     keypair: PKey,  // RSA keypair associated to this Identity
//! }
//! ```
//!
//! The `--identity-new` flag can be set with a value respecting the constraint:
//! `{value} >= 2048 && {value} % 1024 == 0`, which is used as the size of the generated keypair.
//! If no value is specified or if the specified value violates the constraint, an error is thrown.
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
//! When an identity is loaded from the registry - thanks to the `active` attribute set by the `--identity`
//! flag - its Public Key hash is recalculated and compared to the stored hash. If they don't
//! match, it means that this identity is corrupted: one of its hash or keypair was modified.
//! The node automatically shuts himself down in order to keep the network safe.
//!
//! These identities are used to sign the data that is sent to the Locksidian blockchain: a node is
//! always considered as the author of the block it mines, and therefore it has the right to sign the
//! data it contains.
//!
//! ### Peer-to-Peer network
//!
//! In order to start the `Locksidian` service, run the executable by specifying a listening address:
//! `locksidian --daemon={listen_addr}`, or define the following environment variable: `LS_DAEMON={listen_addr}`.
//!
//! Run the executable in *peer mode* by specifying an entrypoint using `--entrypoint={addr}`.
//!
//! The `Peer`structure is defined as follows:
//!
//! ```rust
//! struct Peer {
//!     identity: String,   // Unique identifier for this peer
//!     key: PKey,          // RSA public key
//!     address: String,    // HTTP(S) URL with port number
//!
//!     last_sent: u64,     // Timestamp of the last time data were sent to this peer
//!     last_recv: u64      // Timestamp of the last time data were received from this peer
//! }
//! ```
//!
//! The `entrypoint` is the address of any node in a `Locksidian` peer-to-peer network. During the
//! node startup, it will fetch its own publicly accessible IP address by parsing monip.org website.
//! Then a first request is issued to check that the `entrypoint`'s version of the Locksidian daemon
//! matches its own. Then a request containing the node public key and public address is sent to the
//! `POST /peers/register` endpoint of its `entrypoint`.
//!
//! Note that a node can be started in a local-only mode using the `--local` flag. This mode will
//! ignore the public IP discovery and advertise the `listen_addr`. Use it for testing purposes or
//! strict local network only, otherwise your node will not be able to join any publicly accessible
//! network.
//!
//! The entrypoint will initialize a new `Peer` structure using the provided information, and check
//! back the node's version of the Locksidian daemon. If the version matches, it sends back its own
//! information.
//!
//! Finally, the node will gather all of its entrypoint's peers by sending a `GET /peers` request
//! in order to create its own list of peers. For each of them, it will check their daemon version
//! before registering them. This way, a single peer address is needed to join the peer-to-peer network.
//!
//! Once the registration process is completed, the node will check the current `HEAD` reference of
//! its entrypoint, and sync down its blockchain if this block hash is unknown. This way, a node
//! that could have been disconnected for any period of time will still be able to catch up with its
//! peers by fetching the missing blocks in its local registry.
//!
//! But a node can also start without specifying an `entrypoint`, in what we'll call a *standalone mode*.
//! When a node starts in standalone mode, it will not try to join any existing peer-to-peer network
//! but will instead be the first `entrypoint` of a new `Locksidian` network! This way, you can
//! easily create at will your own private network, hence your own private `Locksidian` blockchain.
//!
//! ### Store a JSON document inside the blockchain
//!
//! The `Block` structure is defined as follows:
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
//! In fact, you have the possibility to "protect" your node using the `--protected` command
//! line argument.
//!
//! When running in protected mode, the node will check for a valid body signature inside the
//! `X-LS-SIGNATURE` HTTP header matching its current `Identity` when receiving a new JSON document
//! on its `/blocks` endpoint.
//!
//! If there is no signature provided or if the signature does not match, a `403 Unauthorized` HTTP
//! status will be returned to the client.
//!
//! ### Block replication 101
//!
//! In order to replicate a block, the following fields of the `Block` structure are sent to the
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
//! Once the replication process is successful, the node will broadcast it to all of its peers, to
//! ensure that it reaches all of the network nodes.
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
//! *Not Implemented Yet*
//!
//! In order to keep the blockchain consistent and to get rid of unused, useless or potentially
//! altered, corrupted or falsified data, the blockchain (i.e. the entire registry of a node)
//! can be pruned by making a request on the `DELETE /blocks` endpoint.
//!
//! The prune process is rather simple: all the blocks that are not part of the main chain are
//! discarded (i.e. removed from the node's registry). The **main chain** can be described as the
//! succession of blocks respectively linked by their `previous` and `next` fields, that lie between
//! the `HEAD` and `ORIGIN` blocks.

// Custom compiler lint checks
#![forbid(
    exceeding_bitshifts, mutable_transmutes, no_mangle_const_items, unknown_crate_types, warnings
)]
#![deny(
    deprecated, improper_ctypes, missing_docs,
    non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
    private_no_mangle_fns, private_no_mangle_statics, stable_features,
    unconditional_recursion, unknown_lints, unsafe_code, unused, unused_allocation,
    unused_attributes, unused_comparisons, unused_features, unused_parens, while_true
)]
#![warn(
    trivial_casts, trivial_numeric_casts, unused_import_braces,
    /*unused_extern_crates, unused_qualifications, unused_results*/
)]

// Documentation configuration
#![doc(
    html_logo_url = "https://avatars0.githubusercontent.com/u/25910673?v=3&s=200",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/25910673?v=3&s=200",
    html_root_url = "http://locksidian.fries.io"
)]

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
#[macro_use]
extern crate serde_wat;

extern crate hyper;
extern crate igd;
extern crate ipnetwork;

#[macro_use(router)]
extern crate router;
extern crate iron;
extern crate bodyparser;
extern crate iron_test;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

#[macro_use]
extern crate log;
extern crate mowl;

extern crate r2d2;
extern crate r2d2_diesel;

// Project modules
mod error;
mod opts;
mod sec;

mod cli;
#[macro_use]
mod persistence;
#[macro_use]
mod api;

mod blockchain;

use error::*;
use persistence::prelude::*;
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

/// Usage: locksidian [options]
///
/// Options:
///
/// * -h, --help: display this help menu
/// * -v, --version: output version information and exit
/// * --verbose: activates verbose mode
/// * --trace: activates full log trace mode
/// * -d, --daemon LISTEN_ADDR: starts the Locksidian daemon service and HTTP REST API
/// * -p, --protected: starts the Locksidian daemon in protected mode. Only available when running with --daemon
/// * --local: starts the Locksidian daemon in local networking mode, thus deactivating the routable address gathering
/// * -i, --identity IDENTITY_HASH: switch the active node identity
/// * --identity-new BIT_SIZE: generate a new identity (defaults to 4096 bit RSA keypair)
/// * --identity-import PATH_TO_PEM_FILE: import the specified PEM-encoded RSA keypair as the new active identity
/// * --identity-export IDENTITY_HASH: export the specified identity keypair to stdout
/// * -e, --entrypoint ADDRESS: specify the IP address or hotsname of the network entrypoint
fn main() {
    match setup_registry() {
        Ok(()) => (),
        Err(err) => {
            error!("{}", err.description());
            exit(EXIT_FAILURE);
        }
    }

    match opts::init() {
        Ok(matches) => match cli::handle(matches) {
            Ok(success) => {
                println!("{}", success);
                exit(EXIT_SUCCESS);
            },
            Err(err) => {
                error!("{}", err.description());
                exit(EXIT_FAILURE);
            }
        },
        Err(err) => {
            println!("{}\n\n{}", err.description(), opts::usage());
            exit(EXIT_FAILURE);
        }
    }
}

/// Establish a connection to the registry and setup the database schemas.
fn setup_registry() -> LocksidianResult<()> {
    let connection = get_connection(database_path())?;
    setup_database(&connection)?;

    Ok(())
}