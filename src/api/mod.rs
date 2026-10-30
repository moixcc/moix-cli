pub mod context;
pub mod database;
mod request;
mod response;

use crate::api::database::ApiDatabase;
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use context::ApiContext;
use request::ApiRequest;
use response::ApiResponse;
use rhai::{AST, Engine, EvalAltResult, Scope};
use std::{collections::HashMap, path::PathBuf};

pub async fn handle(
    path_db: PathBuf,
    method: String,
    headers: HeaderMap,
    form_data: HashMap<String, String>,
    body: Bytes,
    path_file: PathBuf,
) -> Response<Body> {
    let mut engine = Engine::new();

    engine.build_type::<ApiResponse>();
    engine.build_type::<ApiRequest>();
    engine.build_type::<ApiContext>();
    engine.build_type::<ApiDatabase>();

    let mut scope = Scope::new();

    scope.push("response", ApiResponse::new());

    let ast: AST = engine
        .compile_file_with_scope(&mut scope, path_file)
        .unwrap();
    let request = ApiRequest::new(method.clone(), headers, body, form_data);
    let context = ApiContext::new();
    let database = ApiDatabase::new(path_db);

    let target_fn = format!("on_{}", method);
    let result: Result<ApiResponse, Box<EvalAltResult>> = engine
        .call_fn(
            &mut scope,
            &ast,
            &target_fn,
            (request.clone(), context.clone(), database.clone()),
        )
        .or_else(|err| {
            if let EvalAltResult::ErrorFunctionNotFound(fn_name, _) = err.as_ref() {
                if fn_name.starts_with(&target_fn) {
                    return engine.call_fn(&mut scope, &ast, "on", (request, context, database));
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
