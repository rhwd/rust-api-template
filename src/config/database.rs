use crate::structs::app_state::AppState;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub async fn connection() -> Arc<AppState> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful.");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    Arc::new(AppState { db: pool.clone() })
}
