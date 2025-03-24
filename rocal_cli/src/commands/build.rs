use std::process::Command;

use super::utils::{
    color::Color,
    indicator::{IndicatorLauncher, Kind},
};

pub fn build() {
    let mut indicator = IndicatorLauncher::new()
        .kind(Kind::Dots)
        .interval(100)
        .text("Building...")
        .color(Color::White)
        .start();

    let output = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .arg("--dev")
        .output()
        .expect("Confirm you run this command in a rocal project or you've installed wasm-pack");

    let _ = indicator.stop();

    if !output.status.success() {
        eprintln!(
            "rocal build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        println!("Done.");
    }
}
