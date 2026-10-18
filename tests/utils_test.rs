use moix_cli::utils::{Config, b64, build, init};
use std::{fs, path::PathBuf};

#[test]
fn b64_encode_bytes() {
    let base_str = "Hello,  Moix!";
    let b64_str = b64::encode_bytes(base_str.as_bytes());

    assert_eq!(b64_str, "SGVsbG8sICBNb2l4IQ==");
}

#[test]
fn b64_encode_path_str() {
    let path_str = "./example/cdn/data.json";
    let mut path = PathBuf::from(path_str);
    let contents_str = fs::read_to_string(path.clone()).unwrap();
    let contents_b64 = b64::encode_bytes(contents_str.as_bytes());

    b64::encode_path(path_str);

    path.add_extension("txt");

    assert!(path.exists());

    let contents_txt = fs::read_to_string(path.clone()).unwrap();

    assert_eq!(contents_b64, contents_txt);
}

#[test]
fn init_exists_files() {
    let path_str = "./example";
    let path = PathBuf::from(path_str);

    init::handle(path_str);

    for file_name in init::FILES {
        let file = path.join(file_name);

        assert!(file.exists());
    }
}

#[test]
fn build_index_bin() {
    let path_str = "./example";
    let index_bin = PathBuf::from(path_str).join("index.bin");

    build::handle(path_str);

    assert!(index_bin.exists());
}

#[test]
fn config_mimetype_json() {
    let path = PathBuf::from("./example");
    let config = Config::new(&path);
    let mimetype = config.get_mimetype(&path.join("cdn/data.json"));

    assert_eq!(mimetype, "application/json".to_string());
}
