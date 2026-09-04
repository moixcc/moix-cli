use std::env;

pub fn handle(path: std::path::PathBuf) {
    println!("moix deploy {}", path.display());

    dotenvy::from_path(&path.join(".env")).expect(".env error");

    let url = env::var("DEPLOY_URL").expect("deploy_url error");
    let token = env::var("DEPLOY_TOKEN").expect("deploy_token error");

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
        let text = response.as_str().expect("reponse body error");
        println!("error {text}");
    }
}
