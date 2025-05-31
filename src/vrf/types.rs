#[derive(Debug)]
pub enum VRFError {
    InvalidSignature,
    InvalidPublicKey,
    InvalidCommitment,
    DeserializationError,
    VerificationFailed,
}
