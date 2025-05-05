use std::io::prelude::*;
use std::{env, fs};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use models::content_type::ContentType;
use utils::color::Color;

mod models;
mod utils;

pub fn run(port: Option<&str>) {
    let port = if let Some(port) = port { port } else { "3000" };

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", &port)).expect(&format!(
        "{}",
        Color::Red.text(&format!("Failed to listen on 127.0.0.1:{}.", &port))
    ));

    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!(
                "{}",
                Color::Red.text(&format!(
                    "[ERROR] the current directory could not be found: {}",
                    e
                ))
            );
            return;
        }
    };
    println!(
        "Serving path {}",
        Color::Cyan.text(&current_dir.to_string_lossy())
    );
    println!("Available at:");
    println!(
        "    {}",
        Color::Green.text(&format!("http://127.0.0.1:{}", &port))
    );
    println!("\nQuit by pressing CTRL-C");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    Color::Red.text(&format!("[ERROR] Connection failed: {}", e))
                );
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);

            if request.is_empty() {
                let res = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(res.as_bytes()).expect(&format!(
                    "{}",
                    Color::Red.text("[ERROR] Failed to return 400 Bad Request")
                ));
                return;
            }

            let request: Vec<&str> = request.lines().collect();
            let request = if let Some(request) = request.first() {
                request
            } else {
                let res = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(res.as_bytes()).expect(&format!(
                    "{}",
                    Color::Red.text("[ERROR] Failed to return 400 Bad Request")
                ));
                return;
            };
            let request: Vec<&str> = request.split(' ').collect();
            let resource = if let Some(resource) = request.get(1) {
                resource
            } else {
                let res = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(res.as_bytes()).expect(&format!(
                    "{}",
                    Color::Red.text("[ERROR] Failed to return 400 Bad Request")
                ));
                return;
            };
            let resource: Vec<&str> = resource.split('?').collect();
            let resource = if let Some(resource) = resource.get(0) {
                resource
            } else {
                let res = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(res.as_bytes()).expect(&format!(
                    "{}",
                    Color::Red.text("[ERROR] Failed to return 400 Bad Request")
                ));
                return;
            };

            let file_path = if 1 < resource.len() {
                let resource = &resource[1..];
                resource
            } else {
                "index.html"
            };

            let contents = if let Ok(contents) = fs::read(&format!("./{}", file_path)) {
                println!("[INFO] {} could be found", resource);
                contents
            } else {
                eprintln!(
                    "{}",
                    Color::Red.text(&format!("[ERROR] {} could not be found", resource))
                );
                let res = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(res.as_bytes()).expect(&format!(
                    "{}",
                    Color::Red.text("[ERROR] Failed to return 400 Bad Request")
                ));
                return;
            };

            let content_type = ContentType::new(file_path);

            let response_header = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nCross-Origin-Opener-Policy: same-origin\r\nCross-Origin-Embedder-Policy: require-corp\r\n\r\n",
                contents.len(),
                content_type.get_content_type()
            );

            stream
                .write_all(response_header.as_bytes())
                .expect(&format!(
                    "{}",
                    Color::Red.text("[ERROR] Failed to send header")
                ));

            stream.write_all(&contents).expect(&format!(
                "{}",
                Color::Red.text("[ERROR] Failed to send file contents")
            ));
        }
        Err(e) => {
            eprintln!(
                "{}",
                Color::Red.text(&format!("[ERROR] Failed to read from connection: {}", e))
            );
        }
    }
}
