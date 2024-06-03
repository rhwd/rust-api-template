use uuid::Uuid;
use crate::{config::database, models::user::User};

pub async fn create(user_id: Uuid) -> Uuid{
    let db_pool = database::connection().await;

    let query_result = sqlx::query!(
        "INSERT INTO sessions (user_id) VALUES ($1) RETURNING *",
        user_id
    ).fetch_one(&db_pool).await;

    match query_result {
        Ok(session) => {
            return session.id;
        }
        Err(e) => {
            panic!("Failed to create session. {:?}", e);
        }
        
    }
}

pub async fn authenticate(session_id: Uuid) -> User{
    let db_pool = database::connection().await;

    let query_result = sqlx::query_as!(
        User,
        "SELECT name, email, id FROM users WHERE users.id = (SELECT user_id FROM sessions WHERE id = ($1))",
        session_id
    )
    .fetch_one(&db_pool)
    .await;

    match query_result {
        Ok(user) => {
            return user;
        }
        Err(e) => {
            panic!("Failed to authenticate session. {:?}", e);
        }
    }
}