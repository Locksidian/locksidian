//! Block computation algorithms.
//!
//! `ProofOfWork` trait definition and various `Block` algorithms.

pub trait ProofOfWork<T> {
    fn difficulty(block: &T) -> Result<usize, String>;
    fn compute(block: &T) -> Result<(String, u32), String>;
}