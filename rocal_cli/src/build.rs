use std::process::Command;

pub fn build() {
    let output = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .output()
        .expect("Confirm you run this command in a rocal project or you've installed wasm-pack");

    if !output.status.success() {
        eprintln!(
            "rocal build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
