use crate::config::database;
use crate::routes;
use axum::Router;

pub async fn initialize_routes() -> Router {
    let app_state = database::connection().await;
    Router::new()
        .with_state(app_state)
        .merge(routes::healthchecker::create_route())
        .merge(routes::user::create_route())
}
