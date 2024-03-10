use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::user, structs::app_state::AppState};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/users/:id", get(user::get_one))
        .route("/users", post(user::create))
        .with_state(app_state)
}
