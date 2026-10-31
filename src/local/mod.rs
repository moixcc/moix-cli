pub mod api;
pub mod cdn;
pub mod server;

use crate::utils::Config;
use axum::{http::StatusCode, response::IntoResponse};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppState {
    path: PathBuf,
    config: Config,
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND).into_response()
}
