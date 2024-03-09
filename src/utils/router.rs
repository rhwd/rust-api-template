use crate::database;
use crate::routes;
use axum::Router;

pub async fn initialize_routes() -> Router {
    Router::new()
        .merge(routes::healthchecker::create_route())
        .with_state(database::connection().await)
}
