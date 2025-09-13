use crate::authenticator::{Authenticator, Platforms};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct UserPayload {
    username: String,
    code: String,
}

pub async fn root() -> &'static str {
    "praise the lord!"
}

pub async fn auth_leetcode(
    State(auth): State<Authenticator>,
    Json(payload): Json<UserPayload>,
) -> impl IntoResponse {
    info!("Verifying user '{}' on Leetcode", payload.username);

    match auth
        .verify(Platforms::Leetcode, &payload.username, &payload.code)
        .await
    {
        Ok(true) => {
            info!(
                "User '{}' verified successfully on Leetcode",
                payload.username
            );
            (
                StatusCode::OK,
                Json(
                    serde_json::json!({"status": "success", "message": "Authentication successful"}),
                ),
            )
        }
        Ok(false) => {
            info!(
                "Authentication failed for user '{}' on Leetcode",
                payload.username
            );
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"status": "error", "message": "Authentication failed"})),
            )
        }
        Err(e) => {
            error!(
                "Error verifying user '{}' on Leetcode: {:?}",
                payload.username, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"status": "error", "message": "Internal server error"})),
            )
        }
    }
}

pub async fn auth_codeforces(
    State(auth): State<Authenticator>,
    Json(payload): Json<UserPayload>,
) -> impl IntoResponse {
    info!("Verifying user '{}' on Codeforces", payload.username);

    match auth
        .verify(Platforms::Codeforces, &payload.username, &payload.code)
        .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(serde_json::json!({"status": "success", "message": "Authentication successful"})),
        ),
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"status": "error", "message": "Authentication failed"})),
        ),
        Err(e) => {
            error!(
                "Error verifying user '{}' on Codeforces: {:?}",
                payload.username, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"status": "error", "message": "Internal server error"})),
            )
        }
    }
}

pub async fn auth_codechef(
    State(auth): State<Authenticator>,
    Json(payload): Json<UserPayload>,
) -> impl IntoResponse {
    info!("Verifying user '{}' on Codechef", payload.username);

    match auth
        .verify(Platforms::Codechef, &payload.username, &payload.code)
        .await
    {
        Ok(true) => (
            StatusCode::OK,
            Json(serde_json::json!({"status": "success", "message": "Authentication successful"})),
        ),
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"status": "error", "message": "Authentication failed"})),
        ),
        Err(e) => {
            error!(
                "Error verifying user '{}' on Codechef: {:?}",
                payload.username, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"status": "error", "message": "Internal server error"})),
            )
        }
    }
}
