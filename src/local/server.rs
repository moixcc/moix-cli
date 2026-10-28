use crate::{
    local::{AppState, api, cdn, db, not_found},
    utils::Config,
};
use axum::{
    Router,
    extract::{OriginalUri, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::{fs, path::PathBuf};
use tokio::net::TcpListener;

/// moix dev
pub async fn handle(path: PathBuf) {
    println!("http://localhost:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let _ = axum::serve(listener, router(path)).await;
}

pub fn router(path: PathBuf) -> Router {
    let config = Config::new(&path);
    let state = AppState { path, config };

    Router::new()
        // Services
        .nest("/cdn", cdn::router())
        .nest("/api", api::router())
        .nest("/db", db::router())
        // View
        .route("/favicon.ico", get(not_found))
        .route("/", get(index_min))
        .route("/{*all}", get(index_min))
        // State
        .with_state(state)
}

async fn index_min(state: State<AppState>, uri: OriginalUri) -> impl IntoResponse {
    println!("GET {}", uri.path());

    let bytes = fs::read(state.path.join("index.min.html")).unwrap();

    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=UTF-8")],
        bytes,
    )
}
