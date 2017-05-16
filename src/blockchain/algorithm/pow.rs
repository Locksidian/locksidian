//! `ProofOfWork` trait definition.

use error::*;
use num_bigint::BigUint;

pub trait ProofOfWork {
    fn difficulty(&self) -> LocksidianResult<usize>;
    fn target(&self, difficulty: usize) -> LocksidianResult<BigUint>;
    fn compute(&self) -> LocksidianResult<(String, u32)>;
}