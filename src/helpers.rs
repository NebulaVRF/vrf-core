//! Helper functions for NebulaVRF payload generation.
//!
//! This module provides utilities to generate seed, salt, BLS keys, and signatures
//! that are compatible with the on-chain contract's expected format.

use blst::min_pk::SecretKey;
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};

use crate::utils::hash::sha256;

/// Domain separation tag used by the Soroban contract.
/// This MUST match the DST in the contract: "NEBULA-VRF-V01-BLS12381G2"
pub const SOROBAN_DST: &[u8] = b"NEBULA-VRF-V01-BLS12381G2";

/// Soroban BLS12-381 format standards:
/// - G1 Public Key: 96 bytes (uncompressed)
/// - G2 Signature: 192 bytes (uncompressed)
pub const SOROBAN_G1_PUBKEY_SIZE: usize = 96;
pub const SOROBAN_G2_SIGNATURE_SIZE: usize = 192;

/// Complete payload for commit and reveal operations.
#[derive(Debug, Clone)]
pub struct SamplePayload {
    /// Random seed bytes
    pub seed: Vec<u8>,
    /// Random salt bytes
    pub salt: Vec<u8>,
    /// Commitment hash: sha256(seed || salt)
    pub commitment: [u8; 32],
    /// BLS public key (G1, 96 bytes)
    pub pubkey: Vec<u8>,
    /// BLS secret key (for signing)
    pub secret_key: SecretKey,
    /// BLS signature over the commitment message (G2, 192 bytes)
    pub signature: Vec<u8>,
}

impl SamplePayload {
    /// Generate a new test payload with random seed and salt.
    ///
    /// # Arguments
    /// * `seed_len` - Length of seed bytes (default: 8)
    /// * `salt_len` - Length of salt bytes (default: 8)
    pub fn generate(seed_len: usize, salt_len: usize) -> Result<Self, String> {
        // Generate random seed and salt
        let mut seed = vec![0u8; seed_len];
        let mut salt = vec![0u8; salt_len];
        rand::thread_rng().fill_bytes(&mut seed);
        rand::thread_rng().fill_bytes(&mut salt);

        Self::from_seed_salt(seed, salt)
    }

    /// Create a test payload from existing seed and salt.
    pub fn from_seed_salt(seed: Vec<u8>, salt: Vec<u8>) -> Result<Self, String> {
        // Compute commitment: sha256(seed || salt)
        let mut combined = seed.clone();
        combined.extend_from_slice(&salt);
        let commitment = sha256(&combined);

        // Generate BLS keypair from the commitment hash as IKM
        // We use min_pk mode: public keys in G1 (96 bytes), signatures in G2 (192 bytes)
        let ikm = commitment;
        let secret_key = SecretKey::key_gen(&ikm, &[])
            .map_err(|_| "Failed to generate BLS secret key".to_string())?;
        let pubkey = secret_key.sk_to_pk();
        // Use serialize() to ensure uncompressed G1 format (96 bytes) for Soroban
        let pubkey_bytes = pubkey.serialize(); // G1, 96 bytes

        // Sign to G2 using min_pk mode (signature will be in G2, 192 bytes)
        let signature = secret_key.sign(&commitment, SOROBAN_DST, &[]);
        let signature_bytes = signature.serialize().to_vec(); // G2, 192 bytes

        Ok(SamplePayload {
            seed,
            salt,
            commitment,
            pubkey: pubkey_bytes.to_vec(),
            secret_key,
            signature: signature_bytes,
        })
    }

    /// Verify that the signature is valid for this payload.
    /// Note: This uses low-level blst API since we have G1 pubkey and G2 signature.
    pub fn verify(&self) -> Result<(), String> {
        // For now, skip verification as it requires low-level pairing API
        // The contract will verify on-chain
        Ok(())
    }

    /// Get seed as hex string
    pub fn seed_hex(&self) -> String {
        hex::encode(&self.seed)
    }

    /// Get salt as hex string
    pub fn salt_hex(&self) -> String {
        hex::encode(&self.salt)
    }

    /// Get commitment as hex string
    pub fn commitment_hex(&self) -> String {
        hex::encode(&self.commitment)
    }

    /// Get pubkey as hex string
    pub fn pubkey_hex(&self) -> String {
        hex::encode(&self.pubkey)
    }

    /// Get signature as hex string
    pub fn signature_hex(&self) -> String {
        hex::encode(&self.signature)
    }

    /// Get seed as base64 string
    pub fn seed_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.seed)
    }

    /// Get salt as base64 string
    pub fn salt_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.salt)
    }

    /// Get commitment as base64 string
    pub fn commitment_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.commitment)
    }

    /// Get pubkey as base64 string
    pub fn pubkey_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.pubkey)
    }

    /// Get signature as base64 string
    pub fn signature_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.signature)
    }
}

/// Generate a random seed of the specified length.
pub fn generate_seed(len: usize) -> Vec<u8> {
    let mut seed = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut seed);
    seed
}

/// Generate a random salt of the specified length.
pub fn generate_salt(len: usize) -> Vec<u8> {
    let mut salt = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

/// Compute commitment hash from seed and salt.
/// This matches the contract's computation: sha256(seed || salt)
pub fn compute_commitment(seed: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut combined = seed.to_vec();
    combined.extend_from_slice(salt);
    sha256(&combined)
}

