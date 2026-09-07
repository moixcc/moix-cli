pub fn handle(path: std::path::PathBuf, args: Vec<String>) {
    let uri = args.get(3).expect("uri error");
    let mimetype = args.get(4).expect("mimetype error");

    println!("moix cdn {} /cdn/{} {}", path.display(), uri, mimetype);

    let file_bytes = std::fs::read(path.join("cdn").join(uri)).expect("file error");

    let (url, token) = super::get_env_deploy(&path);

    let response = minreq::put(format!("{url}/cdn/{uri}"))
        .with_header("Authorization", &format!("Bearer {token}"))
        .with_header("Content-Type", mimetype)
        .with_body::<Vec<u8>>(file_bytes)
        .send()
        .expect("response error");

    if response.status_code == 200 {
        println!("Updated!");
    } else {
        let text = response.as_str().expect("response body error");
        println!("remote error: {text}");
    }
}
