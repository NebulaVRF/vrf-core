use sha2::{Digest, Sha256};

/// Generate a commit hash from the seed
pub fn commit(seed: &[u8]) -> [u8; 32] {
    Sha256::digest(seed).into()
}

/// Verify that the given seed matches the commit hash
pub fn verify_commit(seed: &[u8], commitment: &[u8; 32]) -> bool {
    &commit(seed) == commitment
}
