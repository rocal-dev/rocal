use std::{fs::File, io::Write};

pub fn create_gitignore() {
    let content = r#"
target
pkg
release
release.tar.gz
Cargo.lock
"#;

    let mut file = File::create(".gitignore").expect("Failed to create .gitignore");

    file.write_all(content.to_string().as_bytes())
        .expect("Failed to create .gitignore");
    file.flush().expect("Failed to create .gitignore");
}
