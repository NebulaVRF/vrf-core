//! Unit tests for NebulaVRF core logic: generation, verification, commit-reveal, and edge cases.
//!
//! These tests cover:
//! - VRF output validity and proof verification
//! - Commit-reveal integrity
//! - Determinism and uniqueness
//! - Tamper/corruption detection
//! - Edge cases (empty seeds, collisions)

use nebula_vrf::vrf::{generate_random, verify_proof};
use nebula_vrf::vrf::commit::{commit, verify_commit};
use nebula_vrf::vrf::types::VRFError;

/// Test that VRF output is valid, verifiable, and fails on tampering.
#[test]
fn test_generate_and_verify_vrf() {
    let seed = b"secure-seed-xyz";
    let vrf = generate_random(seed).expect("generation failed");

    // Output sizes: BLS signature (G1) and public key (G2)
    assert_eq!(vrf.output.len(), 48);      // 48 bytes for BLS signature
    assert_eq!(vrf.public_key.len(), 96);  // 96 bytes for BLS public key

    // Proof should verify for correct seed
    assert!(verify_proof(seed, &vrf.output, &vrf.public_key).is_ok());

    // Verification should fail for wrong seed
    let bad = verify_proof(b"wrong-seed", &vrf.output, &vrf.public_key);
    assert!(matches!(bad, Err(VRFError::VerificationFailed)));
}

/// Test commit-reveal: commit is reproducible and detects tampering.
#[test]
fn test_commit_reveal() {
    let seed = b"fairness-proof";
    let commitment = commit(seed);

    // Should verify for correct seed
    assert!(verify_commit(seed, &commitment));
    // Should fail for tampered seed
    assert!(!verify_commit(b"tampered", &commitment));
}

/// Test that VRF is deterministic: same seed always yields same output/proof.
#[test]
fn test_vrf_determinism() {
    let seed = b"repeatable-seed-999";

    let vrf1 = generate_random(seed).expect("generation 1 failed");
    let vrf2 = generate_random(seed).expect("generation 2 failed");

    assert_eq!(vrf1.output, vrf2.output, "VRF output must be deterministic");
    assert_eq!(vrf1.public_key, vrf2.public_key, "VRF proof must be deterministic");
}

/// Test that different seeds yield unique outputs (no collisions).
#[test]
fn test_vrf_uniqueness() {
    let seed1 = b"unique-seed-1";
    let seed2 = b"unique-seed-2";

    let vrf1 = generate_random(seed1).expect("vrf1 failed");
    let vrf2 = generate_random(seed2).expect("vrf2 failed");

    assert_ne!(vrf1.output, vrf2.output, "Different seeds should produce different VRF outputs");
}

/// Test that a corrupted signature does not verify.
#[test]
fn test_corrupted_signature_fails() {
    let seed = b"test-seed";
    let vrf = generate_random(seed).unwrap();

    let mut corrupted = vrf.output.clone();
    corrupted[0] ^= 0xff; // Flip a bit

    let result = verify_proof(seed, &corrupted, &vrf.public_key);
    assert!(
        matches!(result, Err(VRFError::VerificationFailed) | Err(VRFError::InvalidSignature)),
        "Corrupt signature must not verify"
    );
}

/// Test that a corrupted public key does not verify.
#[test]
fn test_corrupted_public_key_fails() {
    let seed = b"test-seed";
    let vrf = generate_random(seed).unwrap();

    let mut corrupted = vrf.public_key.clone();
    corrupted[1] ^= 0xff; // Flip a bit

    let result = verify_proof(seed, &vrf.output, &corrupted);
    assert!(
        matches!(result, Err(VRFError::VerificationFailed) | Err(VRFError::InvalidPublicKey)),
        "Corrupt public key must not verify"
    );
}

/// Test that different seeds do not collide in commit-reveal.
#[test]
fn test_commit_collision() {
    let seed1 = b"collide-me-1";
    let seed2 = b"collide-me-2";

    let hash1 = commit(seed1);
    let hash2 = commit(seed2);

    assert_ne!(hash1, hash2, "Different seeds should not hash to the same commitment");
}

/// Test that empty seed input is handled gracefully.
#[test]
fn test_empty_seed_input() {
    let empty = b"";

    let vrf = generate_random(empty);
    assert!(vrf.is_ok(), "VRF generation should handle empty seed");

    let proof = vrf.unwrap();
    assert!(verify_proof(empty, &proof.output, &proof.public_key).is_ok());
}
