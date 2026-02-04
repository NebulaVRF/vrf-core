use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};
use nebula_vrf::vrf::{generate_random, verify_proof};
use nebula_vrf::vrf::commit::{commit, verify_commit};
use nebula_vrf::SamplePayload;

use rand::rngs::OsRng;
use rand::RngCore;

/// GET /get-random?seed=<hex>&proof=true&commit=true
#[derive(Debug, Deserialize)]
pub struct RandomRequest {
    pub seed: Option<String>,
    pub proof: Option<bool>,
    pub commit: Option<bool>,
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
    // Use user-supplied seed if provided and valid, else generate random
    let seed = if let Some(seed_hex) = &params.seed {
        match hex::decode(seed_hex) {
            Ok(bytes) if bytes.len() == 32 => bytes,
            _ => {
                let mut s = [0u8; 32];
                OsRng.fill_bytes(&mut s);
                s.to_vec()
            }
        }
    } else {
        let mut s = [0u8; 32];
        OsRng.fill_bytes(&mut s);
        s.to_vec()
    };

    // Generate randomness using NebulaVRF
    let vrf = generate_random(&seed).expect("VRF generation failed");

    let response = RandomResponse {
        seed: hex::encode(&seed),
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
pub struct PayloadRequest {
    pub seed_len: Option<usize>,
    pub salt_len: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct PayloadGroup {
    pub seed: String,
    pub salt: String,
    pub commitment: String,
    pub pubkey: String,
    pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct PayloadResponse {
    pub hex: PayloadGroup,
    pub base64: PayloadGroup,
}

pub async fn payloads_handler(Query(params): Query<PayloadRequest>) -> Json<PayloadResponse> {
    let seed_len = params.seed_len.unwrap_or(8);
    let salt_len = params.salt_len.unwrap_or(8);

    let payload = SamplePayload::generate(seed_len, salt_len)
        .expect("Failed to generate sample payload");

    Json(PayloadResponse {
        hex: PayloadGroup {
            seed: payload.seed_hex(),
            salt: payload.salt_hex(),
            commitment: payload.commitment_hex(),
            pubkey: payload.pubkey_hex(),
            signature: payload.signature_hex(),
        },
        base64: PayloadGroup {
            seed: payload.seed_base64(),
            salt: payload.salt_base64(),
            commitment: payload.commitment_base64(),
            pubkey: payload.pubkey_base64(),
            signature: payload.signature_base64(),
        },
    })
}

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
