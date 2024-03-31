use axum::{response::IntoResponse, routing::get, Json, Router};

use crate::utils::jwt_claim::Claims;

pub fn create_route() -> Router {
    Router::new().route("/status", get(health_checker_handler))
}

async fn health_checker_handler(claims: Claims) -> impl IntoResponse {
    const MESSAGE: &str = "API template built with Rust, SQLX, Postgres, and Axum.";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
        "claim": claims
    });

    Json(json_response)
}
