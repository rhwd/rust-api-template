use crate::{
    models::user::{CreateUser, User},
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
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = User {
        id,
        name: "Joe".to_string(),
        email: "Joemail.com".to_string(),
    };
    let json_response = serde_json::json!(user);
    Ok(Json(json_response))
}

pub async fn create(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name,email) VALUES ($1, $2) RETURNING *",
        body.name.to_string(),
        body.email.to_string()
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
