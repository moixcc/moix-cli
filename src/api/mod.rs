mod context;
mod request;
mod response;
use context::Context;
use request::Request;
use response::Response;
use rhai::{AST, Engine, EvalAltResult, Scope};

pub fn handle(
    client: tiny_http::Request,
    path: std::path::PathBuf,
) -> anyhow::Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.build_type::<Response>();
    engine.build_type::<Request>();
    engine.build_type::<Context>();

    let mut scope = Scope::new();

    scope.push("response", Response::new());

    let ast: AST = engine.compile_file_with_scope(&mut scope, path)?;

    let request = Request::new();
    let context = Context::new();

    let result: Response = engine.call_fn(&mut scope, &ast, "on", (request, context))?;

    client.respond(result.response()).unwrap();

    Ok(())
}
