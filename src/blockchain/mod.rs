//! Blockchain root module.

mod algorithm;

pub mod version;
pub mod network;
pub mod identity;
pub mod block;
pub mod peer;
pub mod metric;

/// Return the current timestamp as an `u64`.
pub fn get_current_timestamp() -> u64 {
    let current_time = ::time::get_time();
    let milliseconds = current_time.sec as u64;
    
    milliseconds
}