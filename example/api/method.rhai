// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods
// https://www.iana.org/assignments/http-methods
fn on(req, ctx, db) {
    response.error(req.method, 500);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/HEAD
fn on_head(req, ctx, db) {
    response.error("", 501);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/GET
fn on_get(req, ctx, db) {
    response.error(req.method, 502);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/POST
fn on_post(req, ctx, db) {
    response.error(req.method, 503);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PUT
fn on_put(req, ctx, db) {
    response.error(req.method, 504);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PATCH
fn on_patch(req, ctx, db) {
    response.error(req.method, 505);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/DELETE
fn on_delete(req, ctx, db) {
    response.error(req.method, 506);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/OPTIONS
fn on_options(req, ctx, db) {
    response.error(req.method, 507);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/CONNECT
fn on_connect(req, ctx, db) {
    response.error(req.method, 508);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/TRACE
fn on_trace(req, ctx, db) {
    response.error(req.method, 509);
}

// https://www.rfc-editor.org/info/rfc3253/#section-3.6
fn on_report(req, ctx, db) {
    response.error(req.method, 510);
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/QUERY
fn on_query(req, ctx, db) {
    response.error(req.method, 511);
}
