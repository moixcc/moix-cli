// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods
// https://www.iana.org/assignments/http-methods
fn on(req, ctx) {
    response.error(req.method, 500)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/HEAD
fn on_head(req, ctx) {
    response.error("on_" + req.method, 400)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/GET
fn on_get(req, ctx) {
    response.error("on_" + req.method, 401)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/POST
fn on_post(req, ctx) {
    response.error("on_" + req.method, 402)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PUT
fn on_put(req, ctx) {
    response.error("on_" + req.method, 403)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PATCH
fn on_patch(req, ctx) {
    response.error("on_" + req.method, 404)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/DELETE
fn on_delete(req, ctx) {
    response.error("on_" + req.method, 405)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/OPTIONS
fn on_options(req, ctx) {
    response.error("on_" + req.method, 406)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/CONNECT
fn on_connect(req, ctx) {
    response.error("on_" + req.method, 407)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/TRACE
fn on_trace(req, ctx) {
    response.error("on_" + req.method, 408)
}

// https://www.rfc-editor.org/info/rfc3253/#section-3.6
fn on_report(req, ctx) {
    response.error("on_" + req.method, 409)
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/QUERY
fn on_query(req, ctx) {
    response.error("on_" + req.method, 410)
}
