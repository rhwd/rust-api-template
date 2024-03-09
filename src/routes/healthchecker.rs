use axum::{response::IntoResponse, routing::get, Json, Router};

pub fn create_route() -> Router {
    Router::new().route("/status", get(health_checker_handler))
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API template built with Rust, SQLX, Postgres, and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
