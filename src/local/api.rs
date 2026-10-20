use crate::api;
use crate::local::AppState;
use axum::{
    Router,
    body::{Body, Bytes, to_bytes},
    extract::{OriginalUri, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::any,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/{*path}", any(handle))
}

async fn handle(state: State<AppState>, uri: OriginalUri, request: Request) -> impl IntoResponse {
    let file = state
        .path
        .join(uri.path().strip_prefix('/').unwrap_or(uri.path()))
        .with_extension("rhai");

    if file.exists() {
        let (parts, body) = request.into_parts();
        let body_bytes = to_bytes(body, usize::MAX).await.unwrap();

        println!("{} {} | {}", parts.method, uri.path(), file.display());

        return api::handle(parts, body_bytes, file).await;
    }

    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(Body::empty())
        .unwrap()
}
