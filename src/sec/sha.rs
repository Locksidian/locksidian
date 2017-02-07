//! SHA algorithms, used to compute the hash of various binary data.

use crypto::digest::Digest;
use crypto::sha2::Sha512;

/// Process the bytes slice (`&[u8]`) and return its SHA512 hash value.
///
/// Example usage :
///
/// ```rust
/// use sec::sha::sha512;
///
/// let hash = sha512("Hello World!".as_bytes());
/// assert_eq!(hash, concat!(
///     "861844d6704e8573fec34d967e20bcfe",
///     "f3d424cf48be04e6dc08f2bd58c72974",
///     "3371015ead891cc3cf1c9d34b49264b5",
///     "10751b1ff9e537937bc46b5d6ff4ecc8"
/// ));
/// ```
pub fn sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.input(data);

    hasher.result_str()
}

#[cfg(test)]
mod test {
    use sec::sha;

    #[test]
    fn sha512_hello_world() {
        let raw = "Hello World!";
        let raw_hash = "861844d6704e8573fec34d967e20bcfef3d424cf48be04e6dc08f2bd58c729743371015ead891cc3cf1c9d34b49264b510751b1ff9e537937bc46b5d6ff4ecc8";

        let hash = sha::sha512(raw.as_bytes());
        assert_eq!(hash, raw_hash);
    }

    #[test]
    fn sha512_locksidian() {
        let raw = "Locksidian";
        let raw_hash = "00fff8865c6708f9635540a3340e6f80eec4fcdac8814a39e17cbd839983988dc4df836f7e102be1cf45f8c3b4f7c75a1e33d3a3bbfb5131d9a1347abf6b3c08";

        let hash = sha::sha512(raw.as_bytes());
        assert_eq!(hash, raw_hash);
    }
}