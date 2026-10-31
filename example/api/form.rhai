fn on_post(req, ctx, db) {
    let form_data = #{
        type: req.header("content-type"),
        email: req.form("email")
    };

    response.json(form_data);
}