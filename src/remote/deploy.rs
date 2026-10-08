pub fn handle(path: std::path::PathBuf) -> anyhow::Result<()> {
    let (url, token) = super::get_env_deploy(&path);
    let index_bytes = std::fs::read(path.join("index.bin"))?;

    let response = minreq::put(format!("{url}/app/index"))
        .with_header("Authorization", &format!("Bearer {token}"))
        .with_header("Content-Type", "application/octet-stream")
        .with_header("Content-Encoding", "deflate")
        .with_body::<Vec<u8>>(index_bytes)
        .send()?;

    println!(
        "{}/index.bin | {} {}",
        path.display(),
        response.status_code,
        response.as_str()?
    );

    Ok(())
}
