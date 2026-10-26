use moix_cli::utils::{Config, b64, build, init};
use std::{fs, path::PathBuf};

#[test]
fn init_example_files() {
    init::handle("./example".into());

    for file_name in init::FILES {
        assert!(PathBuf::from("./example/").join(file_name).exists());
    }
}

#[test]
fn build_index_min() {
    build::handle("./example".into());

    assert!(PathBuf::from("./example/index.min.html").exists());
}

#[test]
fn b64_encode_bytes() {
    let text = "Hello,  Moix!";
    let b64_str = b64::encode_bytes(text.as_bytes());

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

    let contents_str = fs::read_to_string(path.clone()).unwrap();

    assert_eq!(contents_b64, contents_str);
}

#[test]
fn config_mimetype_json() {
    let path = PathBuf::from("./example");
    let config = Config::new(&path);
    let mimetype_str = config.get_mimetype(&path.join("cdn/data.json"));

    assert_eq!(mimetype_str, "application/json".to_string());
}
