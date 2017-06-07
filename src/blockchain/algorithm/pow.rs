//! `ProofOfWork` trait definition.

use error::*;
use num_bigint::BigUint;

pub trait ProofOfWork {
    fn difficulty(&self) -> LocksidianResult<usize>;
    fn target(&self, difficulty: usize) -> LocksidianResult<BigUint>;
    fn compute(&mut self) -> LocksidianResult<(String, u32)>;
    fn validate(&self) -> LocksidianResult<Option<(String, u32)>>;
}