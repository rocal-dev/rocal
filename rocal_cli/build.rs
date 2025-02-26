use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());

    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
}
