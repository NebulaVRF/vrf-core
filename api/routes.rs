// Route definitions for the NebulaVRF API.

use super::handlers::{
    get_random_handler,
    verify_random_handler,
    commit_handler,
    verify_commit_handler,
};
use axum::{Router, routing::{get, post}};

/// Creates all API routes for NebulaVRF.
pub fn create_routes() -> Router {
    Router::new()
        .route("/get-random", get(get_random_handler))
        .route("/verify-random", post(verify_random_handler))
        .route("/commit", post(commit_handler))
        .route("/verify-commit", post(verify_commit_handler))
}
