use axum_extra::extract::SignedCookieJar;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
    pub signed_jar: SignedCookieJar
}
