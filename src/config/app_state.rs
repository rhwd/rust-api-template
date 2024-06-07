use axum_extra::extract::cookie::Key;

use crate::structs::app_state::AppState;
use crate::config::database; 

pub async fn app_state()->AppState{
  let state = AppState {db: database::connection().await, key: Key::generate() };
  state
}