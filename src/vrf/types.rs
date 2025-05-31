/// VRFProof includes the random output and the public key for verification.
#[derive(Debug)]
pub struct VRFProof {
    /// The randomness (signature)
    pub output: Vec<u8>,
    /// The proof (public key)
    pub public_key: Vec<u8>,
}

/// Errors that can occur during VRF operations.
#[derive(Debug)]
pub enum VRFError {
    /// The signature is invalid or malformed.
    InvalidSignature,
    /// The public key is invalid or malformed.
    InvalidPublicKey,
    /// The commitment is invalid or does not match.
    InvalidCommitment,
    /// Deserialization of key or signature failed.
    DeserializationError,
    /// Verification of the proof failed.
    VerificationFailed,
}

impl std::fmt::Display for VRFError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VRFError::InvalidSignature => write!(f, "Invalid signature"),
            VRFError::InvalidPublicKey => write!(f, "Invalid public key"),
            VRFError::InvalidCommitment => write!(f, "Invalid commitment"),
            VRFError::DeserializationError => write!(f, "Deserialization error"),
            VRFError::VerificationFailed => write!(f, "Verification failed"),
        }
    }
}

impl std::error::Error for VRFError {}
