use sha2::{Digest, Sha256};

/// Hashes arbitrary byte input into a 32-byte array using SHA256.
///
/// # Arguments
/// * `input` - The input bytes to hash.
///
/// # Returns
/// * `[u8; 32]` - The SHA256 hash of the input.
pub fn sha256(input: &[u8]) -> [u8; 32] {
    Sha256::digest(input).into()
}
