fn on_post(req, ctx) {
    let form_data = #{
        type: req.header("content-type"),
        email: req.form("email")
    };

    response.json(form_data);
}