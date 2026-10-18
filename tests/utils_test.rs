use moix_cli::utils::b64;
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

    b64::encode_path(path_str);

    let mut path = PathBuf::from(path_str);

    let content = fs::read_to_string(path.clone()).unwrap();

    path.add_extension("txt");

    assert!(path.exists());

    let content_b64 = b64::encode_bytes(content.as_bytes());
    let content_txt = fs::read_to_string(path.clone()).unwrap();

    assert_eq!(content_b64, content_txt);
}
