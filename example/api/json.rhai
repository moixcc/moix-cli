// https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON

fn on(req, ctx) {
    if req.text == "" {
        return response.error("", 400);
    }

    response.ok(req.text)
}

fn on_get(req, ctx) {
    let data = #{
        username: "moix",
        email: "dev@moix.cc",
        urls: ["https://moix.cc", "https://api.moix.cc"]
    };

    response.json(data)
}

fn on_post(req, ctx) {
    let data = req.json;

    if data == () {
        return response.error("", 400);
    }

    response.json(data)
}
