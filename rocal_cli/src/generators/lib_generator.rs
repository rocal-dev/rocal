use std::{fs::File, io::Write};

pub fn create_lib_file() {
    let content = r#"
use rocal::{config, migrate, route};

mod controllers;
mod models;
mod templates;
mod views;

// app_id and sync_server_endpoint should be set to utilize a sync server.
// You can get them by $ rocal sync-servers list
config! {
    app_id: "",
    sync_server_endpoint: "",
    database_directory_name: "local",
    database_file_name: "local.sqlite3"
}

#[rocal::main]
fn app() {
    migrate!("db/migrations");

    route! {
        get "/" => { controller: RootController, action: index, view: RootView }
    }
}
"#;

    let mut file = File::create("src/lib.rs").expect("Failed to create src/lib.rs");

    file.write_all(content.as_bytes())
        .expect("Failed to create src/lib.rs");
    file.flush().expect("Failed to create src/lib.rs");
}
