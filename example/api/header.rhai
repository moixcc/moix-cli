fn on_get(req, ctx) {
    response
        .header("content-type", req.header("content-type"))
        .header("host", req.header("host"))
        .header("x-token", req.header("x-token"))
        .ok("ok");
}