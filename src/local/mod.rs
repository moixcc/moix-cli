pub mod api;
pub mod cdn;
pub mod db;
pub mod server;

use axum::{http::StatusCode, response::IntoResponse};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppState {
    path: PathBuf,
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "")
}
