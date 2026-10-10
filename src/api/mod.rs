mod context;
mod fetch;
mod form;
mod hasher;
mod kipu;
mod request;
mod response;

use anyhow::Result;
use context::Context;
use request::Request;
use response::Response;
use rhai::{AST, Engine, EvalAltResult, Scope};
use std::path::PathBuf;
use tiny_http::Request as ClientRequest;

pub fn handle(mut client: ClientRequest, path: PathBuf) -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.build_type::<Response>();
    engine.build_type::<Request>();
    engine.build_type::<Context>();

    let mut scope = Scope::new();

    scope.push("response", Response::new());

    let ast: AST = engine.compile_file_with_scope(&mut scope, path)?;

    let mut body = vec![];
    let mut buffer = [0u8; 1024];

    match client.as_reader().read(&mut buffer) {
        Ok(bytes_size) => body = buffer[..bytes_size].to_vec(),
        _ => (),
    };

    let method = client.method().to_string().to_lowercase();
    let request = Request::new(method.clone(), body);
    let context = Context::new();

    let target_fn = format!("on_{}", method);
    let result: Result<Response, Box<EvalAltResult>> = engine
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

    client.respond(result?.response()).unwrap();

    Ok(())
}
