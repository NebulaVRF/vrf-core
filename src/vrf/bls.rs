use blst::min_sig::{SecretKey, PublicKey, Signature};
use blst::BLST_ERROR;
use crate::utils::hash::sha256;
use super::types::VRFError;

/// VRFProof includes the random output and the public key for verification.
#[derive(Debug)]
pub struct VRFProof {
    pub output: Vec<u8>,     // The randomness (signature)
    pub public_key: Vec<u8>, // The proof
}

pub fn generate_random(seed: &[u8]) -> Result<VRFProof, VRFError> {
    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";

    let ikm = sha256(seed);
    let sk = SecretKey::key_gen(&ikm, &[]).map_err(|_| VRFError::DeserializationError)?;
    let signature = sk.sign(seed, dst, &[]);
    let pk = sk.sk_to_pk();

    Ok(VRFProof {
        output: signature.to_bytes().to_vec(),
        public_key: pk.to_bytes().to_vec(),
    })
}

pub fn verify_proof(
    seed: &[u8],
    signature_bytes: &[u8],
    public_key_bytes: &[u8],
) -> Result<(), VRFError> {
    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";

    let pk = PublicKey::from_bytes(public_key_bytes)
        .map_err(|_| VRFError::InvalidPublicKey)?;

    let sig = Signature::from_bytes(signature_bytes)
        .map_err(|_| VRFError::InvalidSignature)?;

    let result = sig.verify(true, seed, dst, &[], &pk, true);
    if result == BLST_ERROR::BLST_SUCCESS {
        Ok(())
    } else {
        Err(VRFError::VerificationFailed)
    }
}
