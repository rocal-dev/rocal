use std::fs::File;

use chrono::Utc;

use crate::commands::utils::project::find_project_root;

pub fn add(name: &str) {
    let now = Utc::now();
    let stamp = now.format("%Y%m%d%H%M%S").to_string();
    let file_name = &format!("{stamp}-{name}.sql");

    let root_path = find_project_root().expect("Failed to find the project root");

    File::create(root_path.join(&format!("db/migrations/{file_name}")))
        .expect(&format!("Failed to create db/migrations/{file_name}"));

    println!("{file_name}");
}
