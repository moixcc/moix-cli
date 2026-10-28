use crate::local::{AppState, not_found};
use axum::{Router, routing::any};

pub fn router() -> Router<AppState> {
    Router::new().route("/{*path}", any(not_found))
}
