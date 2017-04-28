//! Block management module.

mod algorithms;

mod block_domain;
mod block_repository;
mod block_dto;

pub use self::algorithms::ProofOfWork;

pub use self::block_domain::Block;
pub use self::block_repository::{BlockEntity, BlockRepository};