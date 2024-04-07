use crate::{
    models::user::{CheckUserLogin, CreateUser, LoginUser, User},
    structs::app_state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn get_one(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(User, "SELECT name, email, id FROM users WHERE users.id = $1", id)
        .fetch_one(&data.db)
        .await;
    match query_result {
        Ok(user) => {
            let user_response = json!(user);
            return Ok((StatusCode::OK, Json(user_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}
pub async fn authorize_user(
    State(data): State<Arc<AppState>>,
    Json(body):Json<LoginUser>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(CheckUserLogin, "SELECT email, password_hash FROM users WHERE users.email = $1", body.email)
        .fetch_one(&data.db)
        .await;
    match user {
        Ok(user) => {
            let is_valid = bcrypt::verify(body.password, &user.password_hash).unwrap();
            if is_valid {
                return Ok((StatusCode::OK, Json(json!({"status": "success", "message": "User is authorized"}))));
            } else {
                return Ok((StatusCode::UNAUTHORIZED, Json(json!({"status": "error", "message": "User is not authorized"}))));
            }
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
        
    }
}

pub async fn create(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let password_hash = bcrypt::hash(body.password.to_string(), bcrypt::DEFAULT_COST).unwrap();
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name,email,password_hash) VALUES ($1, $2, $3) RETURNING name, email, id",
        body.name.to_string(),
        body.email.to_string(),
        password_hash
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = json!(user);
            return Ok((StatusCode::CREATED, Json(user_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}
