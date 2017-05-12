//! `ProofOfWork` trait definition.

use num_bigint::BigUint;

pub trait ProofOfWork {
    fn difficulty(&self) -> Result<usize, String>;
    fn target(&self, difficulty: usize) -> Result<BigUint, String>;
    fn compute(&mut self) -> Result<(String, u32), String>;
    fn validate(&self, target: &BigUint) -> Result<Option<(String, u32)>, String>;
}