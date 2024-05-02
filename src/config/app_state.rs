use std::sync::Arc;
use crate::structs::app_state::AppState;
use crate::config::{database, signed_cookie_jar}; 

pub async fn app_state()->Arc<AppState>{
  let state = Arc::new(AppState {db: database::connection().await, signed_jar: signed_cookie_jar::signed_jar() });
  state
}