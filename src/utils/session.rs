use uuid::Uuid;

use crate::config::database;

pub async fn create(user_id: Uuid) -> Uuid{
    let db_pool = database::connection().await;

    let session = sqlx::query!(
        "INSERT INTO sessions (user_id) VALUES ($1) RETURNING *",
        user_id
    ).fetch_one(&db_pool.db).await;

    match session {
        Ok(session) => {
            return session.id;
        }
        Err(e) => {
            panic!("Failed to create session. {:?}", e);
        }
        
    }
}