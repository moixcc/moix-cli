pub fn handle(path: std::path::PathBuf) {
    let (url, token) = super::get_env_deploy(&path);

    println!("moix deploy {}\n", path.display());

    let index_bytes = std::fs::read(path.join("index.bin")).expect("index.bin error");

    let response = minreq::put(format!("{url}/app/deploy/index"))
        .with_header("Authorization", &format!("Bearer {token}"))
        .with_header("Content-Type", "application/octet-stream")
        .with_header("Content-Encoding", "deflate")
        .with_body::<Vec<u8>>(index_bytes)
        .send()
        .expect("response error");

    if response.status_code == 200 {
        println!("Deployed!");
    } else {
        let text = response.as_str().expect("response body error");
        eprintln!("remote error: {text}");
    }
}
