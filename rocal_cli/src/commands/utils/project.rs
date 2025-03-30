use std::{borrow::Cow, env, path::PathBuf};

pub fn find_project_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return Some(current_dir);
        }
        if !current_dir.pop() {
            break;
        }
    }
    None
}

pub fn get_app_name(root_path: &PathBuf) -> Cow<str> {
    root_path
        .file_name()
        .expect("Failed to find your app name")
        .to_string_lossy()
}
