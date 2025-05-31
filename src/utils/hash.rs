
use sha2::{Digest, Sha256};

/// Hashes arbitrary byte input into a 32-byte array using SHA256.
/// Used for commitments, key derivation, etc.
pub fn sha256(input: &[u8]) -> [u8; 32] {
    Sha256::digest(input).into()
}
