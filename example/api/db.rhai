fn on(req, ctx, db) {
    // SPACE
    let db_test = db.space("test");

    // ENTRY

    // add [ Nature | Quantity | Context ] -> ID
    let id_one = db_test.add(0, 1, "context");
    
    //  put [ 
    //      Nature | Quantity | Context |
    //      Reference |
    //      Metadata | Raw ] -> ID
    let id_two = db_test.put(
        0, 1, "context", 
        id_one, 
        #{"content-type": "text/plain"}, "raw content".to_blob()
    );

    // set_ref [ ID | Reference ]
    db_test.set_ref(id_one, ());

    // get_ref [ ID ] -> Reference
    let my_ref = db_test.get_ref(id_two);

    // set_meta [ ID , Metadata]
    db_test.set_meta(id_one, #{});

    // get_meta [ ID ] -> Metadata
    let my_meta = db_test.get_meta(id_two);

    // set_raw [ ID, RAW]
    db_test.set_raw(id_one, blob());

    // get_raw [ ID ] -> Raw
    let my_raw = db_test.get_raw(id_two);

    // on_ctx [ Context ] -> Cursor<Entry>
    for entry in db_test.on_ctx("context") {}

    // on_ref [ Reference ] -> Cursor<Entry>
    for entry in db_test.on_ref(id_one) {}

    response.ok("ok")
}