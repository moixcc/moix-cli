mod context;
mod request;
mod response;

use axum::{
    body::{Body, Bytes},
    http::{StatusCode, request::Parts},
    response::Response,
};
use context::ApiContext;
use request::ApiRequest;
use response::ApiResponse;
use rhai::{AST, Engine, EvalAltResult, Scope};
use std::path::PathBuf;

pub async fn handle(client_parts: Parts, client_body: Bytes, path_file: PathBuf) -> Response<Body> {
    let mut engine = Engine::new();

    engine.build_type::<ApiResponse>();
    engine.build_type::<ApiRequest>();
    engine.build_type::<ApiContext>();

    let mut scope = Scope::new();

    scope.push("response", ApiResponse::new());

    let ast: AST = engine
        .compile_file_with_scope(&mut scope, path_file)
        .unwrap();

    let method_str = client_parts.method.to_string().to_lowercase();
    let request = ApiRequest::new(method_str.clone(), client_body);
    let context = ApiContext::new();

    let target_fn = format!("on_{}", method_str);
    let result: Result<ApiResponse, Box<EvalAltResult>> = engine
        .call_fn(
            &mut scope,
            &ast,
            &target_fn,
            (request.clone(), context.clone()),
        )
        .or_else(|err| {
            if let EvalAltResult::ErrorFunctionNotFound(fn_name, _) = err.as_ref() {
                if fn_name.starts_with(&target_fn) {
                    return engine.call_fn(&mut scope, &ast, "on", (request, context));
                }
            }
            Err(err)
        });

    match result {
        Ok(api_response) => api_response.response(),
        Err(error) => Response::builder()
            .status(StatusCode::NOT_IMPLEMENTED)
            .body(Body::from(error.to_string()))
            .unwrap(),
    }
}
