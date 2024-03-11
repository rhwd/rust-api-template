mod config;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod structs;
pub mod utils;

#[tokio::main]
async fn main() {
    let router = config::router::initialize_routes().await;
    match tokio::net::TcpListener::bind("0.0.0.0:8000").await {
        Ok(listener) => {
            println!("Server listening at: {}", listener.local_addr().unwrap());
            axum::serve(listener, router).await.unwrap();
        }
        Err(err) => {
            println!("Failed to start server. {:?}", err);
            std::process::exit(1);
        }
    };
}
