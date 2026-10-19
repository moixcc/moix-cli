use axum::{
    Router,
    extract::{OriginalUri, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::{fs, path::PathBuf};
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
struct AppState {
    path: PathBuf,
}

pub async fn handle(path_str: &str) {
    println!("http://localhost:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let _ = axum::serve(listener, router(PathBuf::from(path_str))).await;
}

pub fn router(path: PathBuf) -> Router {
    let state = AppState { path };
    Router::new()
        .route("/favicon.ico", get(not_found))
        .route("/", get(index_min))
        .route("/{*all}", get(index_min))
        .with_state(state)
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "")
}

async fn index_min(OriginalUri(uri): OriginalUri, state: State<AppState>) -> impl IntoResponse {
    println!("GET {}", uri.path());

    let bytes = fs::read(state.path.join("index.min.html")).unwrap();

    (
        StatusCode::OK,
        [("Content-Type", "text/html; charset=UTF-8")],
        bytes,
    )
}
