use std::{
    fs,
    io::{Read, Write},
};

pub fn handle(path: std::path::PathBuf) {
    println!("moix dev {}\nhttp://localhost:8080", path.display());

    let path_index = path.join("index.bin");

    let listener = match std::net::TcpListener::bind("0.0.0.0:8080") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("listener {e}");

            return crate::exit();
        }
    };

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("stream {e}");

                continue;
            }
        };

        let mut buffer = [0u8; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => continue,
            Ok(_) => {}
            Err(e) => {
                eprintln!("read {e}");

                continue;
            }
        };

        let index_bytes = match fs::read(&path_index) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("index read {e}");

                continue;
            }
        };

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=UTF-8\r\n\
             Content-Encoding: br\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\r\n",
            index_bytes.len()
        );

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("write response {e}");

            continue;
        }

        if let Err(e) = stream.write_all(&index_bytes) {
            eprintln!("write index_bin {e}");

            continue;
        }

        let _ = stream.flush();
    }
}
