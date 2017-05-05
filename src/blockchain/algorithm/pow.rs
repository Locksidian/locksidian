//! `ProofOfWork` trait definition.

use num_bigint::BigUint;

pub trait ProofOfWork {
    fn difficulty(&self) -> Result<usize, String>;
    fn target(&self, difficulty: usize) -> Result<BigUint, String>;
    fn compute(&self) -> Result<(String, u32), String>;
}