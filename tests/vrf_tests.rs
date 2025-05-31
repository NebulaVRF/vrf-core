use nebula_vrf::vrf::{generate_random, verify_proof};
use nebula_vrf::vrf::commit::{commit, verify_commit};
use nebula_vrf::vrf::types::VRFError;

#[test]
fn test_generate_and_verify_vrf() {
    let seed = b"secure-seed-xyz";
    let vrf = generate_random(seed).expect("generation failed");

    assert_eq!(vrf.output.len(), 48);      // BLS sig (G1)
    assert_eq!(vrf.public_key.len(), 96);  // BLS pk (G2)

    // Should verify
    assert!(verify_proof(seed, &vrf.output, &vrf.public_key).is_ok());

    // Tamper check
    let bad = verify_proof(b"wrong-seed", &vrf.output, &vrf.public_key);
    assert!(matches!(bad, Err(VRFError::VerificationFailed)));
}

#[test]
fn test_commit_reveal() {
    let seed = b"fairness-proof";
    let commitment = commit(seed);

    assert!(verify_commit(seed, &commitment));
    assert!(!verify_commit(b"tampered", &commitment));
}

#[test]
fn test_vrf_determinism() {
    let seed = b"repeatable-seed-999";

    let vrf1 = generate_random(seed).expect("generation 1 failed");
    let vrf2 = generate_random(seed).expect("generation 2 failed");

    assert_eq!(vrf1.output, vrf2.output, "VRF output must be deterministic");
    assert_eq!(vrf1.public_key, vrf2.public_key, "VRF proof must be deterministic");
}

#[test]
fn test_vrf_uniqueness() {
    let seed1 = b"unique-seed-1";
    let seed2 = b"unique-seed-2";

    let vrf1 = generate_random(seed1).expect("vrf1 failed");
    let vrf2 = generate_random(seed2).expect("vrf2 failed");

    assert_ne!(vrf1.output, vrf2.output, "Different seeds should produce different VRF outputs");
}

#[test]
fn test_corrupted_signature_fails() {
    let seed = b"test-seed";
    let vrf = generate_random(seed).unwrap();

    let mut corrupted = vrf.output.clone();
    corrupted[0] ^= 0xff;

    let result = verify_proof(seed, &corrupted, &vrf.public_key);
    assert!(
        matches!(result, Err(VRFError::VerificationFailed) | Err(VRFError::InvalidSignature)),
        "Corrupt signature must not verify"
    );
}

#[test]
fn test_corrupted_public_key_fails() {
    let seed = b"test-seed";
    let vrf = generate_random(seed).unwrap();

    let mut corrupted = vrf.public_key.clone();
    corrupted[1] ^= 0xff;

    let result = verify_proof(seed, &vrf.output, &corrupted);
    assert!(
        matches!(result, Err(VRFError::VerificationFailed) | Err(VRFError::InvalidPublicKey)),
        "Corrupt public key must not verify"
    );
}


#[test]
fn test_commit_collision() {
    let seed1 = b"collide-me-1";
    let seed2 = b"collide-me-2";

    let hash1 = commit(seed1);
    let hash2 = commit(seed2);

    assert_ne!(hash1, hash2, "Different seeds should not hash to the same commitment");
}

#[test]
fn test_empty_seed_input() {
    let empty = b"";

    let vrf = generate_random(empty);
    assert!(vrf.is_ok(), "VRF generation should handle empty seed");

    let proof = vrf.unwrap();
    assert!(verify_proof(empty, &proof.output, &proof.public_key).is_ok());
}
