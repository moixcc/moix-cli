pub fn handle(path: std::path::PathBuf, args: Vec<String>) -> anyhow::Result<()> {
    let (url, token) = super::get_env_deploy(&path);
    let uri = match args.get(3) {
        Some(u) => u,
        _ => {
            println!("moix cdn {} [CDN_PATH_FILE]", path.display());
            return Ok(());
        }
    };
    let config = crate::utils::Config::new(&path);
    let path_file = path.join("cdn").join(uri);
    let mimetype = config.get_mimetype(&path_file);

    let file_bytes = std::fs::read(path_file)?;
    let response = minreq::put(format!("{url}/cdn/{uri}"))
        .with_header("Authorization", &format!("Bearer {token}"))
        .with_header("Content-Type", &mimetype)
        .with_body::<Vec<u8>>(file_bytes)
        .send()?;

    println!(
        "{}/cdn/{} | '{}' | {} {}",
        path.display(),
        uri,
        mimetype,
        response.status_code,
        response.as_str()?
    );

    Ok(())
}
