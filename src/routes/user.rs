use axum::{routing::get, Router};

use crate::handlers::user;

pub fn create_route() -> Router {
    Router::new().route("/users/:id", get(user::get_one))
}
