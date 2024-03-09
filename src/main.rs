mod database;
use axum::{response::IntoResponse, routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(database::connection().await);

    println!("Server started successfully.");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
