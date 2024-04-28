use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CheckUserLogin {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
