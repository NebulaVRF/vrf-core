use sha2::{Digest, Sha256};

/// Generate a commit hash from the seed.
///
/// # Arguments
/// * `seed` - The input seed as a byte slice.
///
/// # Returns
/// * `[u8; 32]` - The SHA256 hash of the seed.
pub fn commit(seed: &[u8]) -> [u8; 32] {
    Sha256::digest(seed).into()
}

/// Verify that the given seed matches the commit hash.
///
/// # Arguments
/// * `seed` - The input seed as a byte slice.
/// * `commitment` - The expected commitment hash.
///
/// # Returns
/// * `bool` - True if the seed hashes to the commitment, false otherwise.
pub fn verify_commit(seed: &[u8], commitment: &[u8; 32]) -> bool {
    &commit(seed) == commitment
}
