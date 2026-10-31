use crate::api;
use crate::local::AppState;
use axum::{
    Form, Router,
    body::{Body, Bytes},
    extract::{FromRequest, Multipart, OriginalUri, Request, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
};
use std::collections::HashMap;

const DATABASE: &str = "data/kipu.sqlite";

#[derive(Debug)]
pub enum Payload {
    UrlEncoded(HashMap<String, String>),
    Multipart(HashMap<String, String>),
    Bytes(Bytes),
}

pub fn router() -> Router<AppState> {
    Router::new().route("/{*path}", any(handle))
}

async fn handle(
    state: State<AppState>,
    uri: OriginalUri,
    method: Method,
    headers: HeaderMap,
    payload: Payload,
) -> impl IntoResponse {
    let file = state
        .path
        .join(uri.path().strip_prefix('/').unwrap_or(uri.path()))
        .with_extension("rhai");

    if file.exists() {
        println!("{} {} | {}", method, uri.path(), file.display());

        let mut form_data = HashMap::new();
        let mut body_bytes = Bytes::new();

        match payload {
            Payload::UrlEncoded(params) => form_data = params,
            Payload::Multipart(params) => form_data = params,
            Payload::Bytes(bytes) => body_bytes = bytes,
        }

        return api::handle(
            state.path.join(DATABASE),
            method.to_string().to_lowercase(),
            headers,
            form_data,
            body_bytes,
            file,
        )
        .await;
    }

    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(Body::empty())
        .unwrap()
}

impl<S> FromRequest<S> for Payload
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = request
            .headers()
            .get("Content-Type")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        if content_type.starts_with("application/x-www-form-urlencoded") {
            let Form(params) = Form::<HashMap<String, String>>::from_request(request, state)
                .await
                .map_err(|error| error.into_response())?;
            Ok(Payload::UrlEncoded(params))
        } else if content_type.starts_with("multipart/form-data") {
            let mut multipart = Multipart::from_request(request, state)
                .await
                .map_err(|error| error.into_response())?;

            let mut fields = HashMap::new();
            while let Some(field) = multipart.next_field().await.unwrap_or(None) {
                let name = field.name().unwrap_or("").to_string();
                let text = field.text().await.unwrap_or_default();
                fields.insert(name, text);
            }
            Ok(Payload::Multipart(fields))
        } else {
            let bytes = Bytes::from_request(request, state)
                .await
                .map_err(|error| error.into_response())?;

            Ok(Payload::Bytes(bytes))
        }
    }
}
