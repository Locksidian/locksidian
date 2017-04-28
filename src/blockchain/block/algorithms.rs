//! Block computation algorithms.
//!
//! `ProofOfWork` trait definition and various `Block` algorithms.

pub trait ProofOfWork {
    fn difficulty(&self) -> Result<usize, String>;
    fn compute(&self) -> Result<(String, u32), String>;
}