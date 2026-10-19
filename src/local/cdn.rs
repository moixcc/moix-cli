use std::fs;

use crate::local::{AppState, not_found};
use axum::{
    Router,
    extract::{OriginalUri, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{any, get},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{*path}", get(get_contents))
        .route("/{*path}", any(not_found))
}

async fn get_contents(OriginalUri(uri): OriginalUri, state: State<AppState>) -> impl IntoResponse {
    println!("GET {}", uri);

    let file_path_str = uri.path().trim_start_matches('/');
    let file_path = state.path.join(file_path_str);

    if file_path.exists() {
        let mimetype = state.config.get_mimetype(&file_path);

        if let Ok(bytes) = fs::read(file_path) {
            return (StatusCode::OK, [("Content-Type", mimetype)], bytes).into_response();
        }
    }

    (StatusCode::NOT_FOUND).into_response()
}
