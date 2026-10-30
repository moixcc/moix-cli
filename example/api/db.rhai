fn on(req, ctx, db) {
    // SPACE
    let db = db.space("test");

    // ENTRY

    // set_entry [ Nature | Quantity | Context ] -> ID
    let id = db.set_entry(0, 1, "context");

    // set_ref [ ID | Reference ]
    db.set_ref(id, ());
    
    // get_ref [ ID ] -> Reference
    let ref = db_test.get_ref(id);
    
    // set_meta [ ID , Metadata]
    db.set_meta(id, #{});

    // get_meta [ ID ] -> Metadata
    let meta = db.get_meta(id);

    // set_raw [ ID, RAW]
    db.set_raw(id, blob());

    // get_raw [ ID ] -> Raw
    let raw = db.get_raw(id);

    // on_ref [ Reference ] -> Cursor<Entry>
    for entry in db.on_ref(id) {}

    // on_ctx [ Context ] -> Cursor<Entry>
    for entry in db.on_ctx("context") {}

    // get_entry [ ID ]
    let entry = db.get_entry(id);

    response.json(entry)
}