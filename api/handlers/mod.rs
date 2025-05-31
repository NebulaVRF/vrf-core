use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};
use nebula_vrf::vrf::{generate_random, verify_proof};
use nebula_vrf::vrf::commit::{commit, verify_commit};

use rand::rngs::OsRng;
use rand::RngCore;
use hex;

/// GET /get-random?proof=true&commit=true
#[derive(Debug, Deserialize)]
pub struct RandomRequest {
    proof: Option<bool>,
    commit: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RandomResponse {
    seed: String,
    randomness: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<String>,
}

pub async fn get_random_handler(Query(params): Query<RandomRequest>) -> Json<RandomResponse> {
    // Generate secure 32-byte seed
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);

    // Generate randomness using NebulaVRF
    let vrf = generate_random(&seed).expect("VRF generation failed");

    let response = RandomResponse {
        seed: hex::encode(seed),
        randomness: hex::encode(vrf.output),
        public_key: if params.proof.unwrap_or(false) {
            Some(hex::encode(vrf.public_key))
        } else {
            None
        },
        commitment: if params.commit.unwrap_or(false) {
            Some(hex::encode(commit(&seed)))
        } else {
            None
        },
    };

    Json(response)
}

// --- New Handlers ---

#[derive(Debug, Deserialize)]
pub struct VerifyRandomRequest {
    pub seed: String,
    pub output: String,
    pub public_key: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyRandomResponse {
    pub valid: bool,
}

pub async fn verify_random_handler(Json(req): Json<VerifyRandomRequest>) -> Json<VerifyRandomResponse> {
    let seed = hex::decode(&req.seed).unwrap_or_default();
    let output = hex::decode(&req.output).unwrap_or_default();
    let public_key = hex::decode(&req.public_key).unwrap_or_default();
    let valid = verify_proof(&seed, &output, &public_key).is_ok();
    Json(VerifyRandomResponse { valid })
}

#[derive(Debug, Deserialize)]
pub struct CommitRequest {
    pub seed: String,
}

#[derive(Debug, Serialize)]
pub struct CommitResponse {
    pub commitment: String,
}

pub async fn commit_handler(Json(req): Json<CommitRequest>) -> Json<CommitResponse> {
    let seed = hex::decode(&req.seed).unwrap_or_default();
    let commitment = commit(&seed);
    Json(CommitResponse { commitment: hex::encode(commitment) })
}

#[derive(Debug, Deserialize)]
pub struct VerifyCommitRequest {
    pub seed: String,
    pub commitment: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyCommitResponse {
    pub valid: bool,
}

pub async fn verify_commit_handler(Json(req): Json<VerifyCommitRequest>) -> Json<VerifyCommitResponse> {
    let seed = hex::decode(&req.seed).unwrap_or_default();
    let mut commitment_bytes = [0u8; 32];
    if let Ok(bytes) = hex::decode(&req.commitment) {
        if bytes.len() == 32 {
            commitment_bytes.copy_from_slice(&bytes);
        }
    }
    let valid = verify_commit(&seed, &commitment_bytes);
    Json(VerifyCommitResponse { valid })
}
