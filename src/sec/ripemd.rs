//! Ripemd algorithms, use to compute the `Identity` hashes.

use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;

/// Process th bytes slice and return its RIPEMD160 hash value.
///
/// Example usage:
///
/// ```rust
/// use sec::ripemd::ripemd160;
///
/// let hash = ripemd160(b"Hello World!");
/// assert_eq!(hash, "8476ee4631b9b30ac2754b0ee0c47e161d3f724c");
/// ```
pub fn ripemd160(data: &[u8]) -> String {
    let mut hasher = Ripemd160::new();
    hasher.input(data);

    hasher.result_str()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ripemd160_hello_world() {
        let raw = "Hello World";
        let raw_hash = "a830d7beb04eb7549ce990fb7dc962e499a27230";

        let hash = ripemd160(raw.as_bytes());
        assert_eq!(hash, raw_hash);
    }
}