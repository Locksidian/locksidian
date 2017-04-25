//! Block management module.

mod block_domain;
mod block_repository;

pub use self::block_domain::Block;
pub use self::block_repository::{BlockEntity, BlockRepository};