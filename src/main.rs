mod config;
pub mod handlers;
pub mod models;
mod routes;
pub mod utils;

#[tokio::main]
async fn main() {
    let router = config::router::initialize_routes().await;
    println!("Server started successfully.");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
