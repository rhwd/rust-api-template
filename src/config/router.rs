use crate::{config::database, routes};
use axum::Router;

pub async fn initialize_routes() -> Router {
    let app_state = database::connection().await;
    Router::new()
        .merge(routes::healthchecker::create_route())
        .merge(routes::user::create_routes(app_state))
}
