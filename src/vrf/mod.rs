//! VRF module: BLS-based VRF, commit-reveal, and error types.

pub mod bls;
pub mod commit;
pub mod types;

pub use bls::{generate_random, verify_proof};
pub use types::{VRFProof, VRFError};
