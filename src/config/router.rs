use crate::routes;
use axum::Router;

use super::app_state::app_state;

pub async fn initialize_routes() -> Router {
    let app_state = app_state().await;
    Router::new()
        .merge(routes::healthchecker::create_route())
        .merge(routes::user::create_routes(app_state))
}
