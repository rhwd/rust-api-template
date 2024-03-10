use crate::models::user::User;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

pub async fn get_one(
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("I'm being called!!");

    let user = User {
        id,
        name: "Joe".to_string(),
        email: "Joemail.com".to_string(),
    };
    let json_response = serde_json::json!(user);
    Ok(Json(json_response))
}
