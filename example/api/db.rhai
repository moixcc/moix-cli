fn on(req, ctx, db) {
    // SPACE
    let db_test = db.space("test");

    // ENTRY

    // set_entry [ Nature | Quantity | Context ] -> ID
    let id = db_test.set_entry(0, 1, "context");

    // get_entry [ ID ]
    let my_entry = db_test.get_entry(id);

    // set_ref [ ID | Reference ]
    db_test.set_ref(id, ());
    
    // get_ref [ ID ] -> Reference
    let my_ref = db_test.get_ref(id);
    
    // set_meta [ ID , Metadata]
    db_test.set_meta(id, #{});

    // get_meta [ ID ] -> Metadata
    let meta = db_test.get_meta(id);

    // set_raw [ ID, RAW]
    db_test.set_raw(id, blob());

    // get_raw [ ID ] -> Raw
    let raw = db_test.get_raw(id);

    // on_ref [ Reference ] -> Cursor<Entry>
    for entry in db_test.on_ref(id) {}

    // on_ctx [ Context ] -> Cursor<Entry>
    for entry in db_test.on_ctx("context") {}

    response.json(my_entry)
}