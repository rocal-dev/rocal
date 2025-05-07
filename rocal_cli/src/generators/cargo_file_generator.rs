use std::{fs::File, io::Write};

pub fn create_cargo_file(project_name: &str) {
    let content = format!(
        r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
rocal = "0.3"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = {{ version = "0.3", features = [
  "Window",
  "History",
  "console",
  "Location",
  "Document",
  "DocumentFragment",
  "Element",
  "HtmlElement",
  "Node",
  "NodeList",
  "Event",
  "FormData",
  "HtmlFormElement",
  "Worker",
  "WorkerOptions",
  "WorkerType"
]}}
js-sys = "0.3"
serde = {{ version = "1.0", features = ["derive"] }}
serde-wasm-bindgen = "0.6"
"#,
        project_name
    );

    let mut file = File::create("Cargo.toml").expect("Failed to create Cargo.toml");
    file.write_all(content.to_string().as_bytes())
        .expect("Failed to create Cargo.toml");
    file.flush().expect("Failed to create Cargo.toml");
}
