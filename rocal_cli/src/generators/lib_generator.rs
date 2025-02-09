use std::{fs::File, io::Write};

use quote::quote;
use uuid::Uuid;

pub fn create_lib_file() {
    let app_id = Uuid::new_v4().to_string();

    let content = quote! {
        use rocal::{config, migrate, route};

        mod controllers;
        mod models;
        mod templates;
        mod views;

        config! {
            app_id: #app_id,
            sync_server_endpoint: "http://127.0.0.1:3000/presigned-url",
            database_directory_name: "local",
            database_file_name: "local.sqlite3"
        }

        #[rocal::main]
        fn app() {
            route! {
                get "/" => { controller: RootController, action: index, view: RootView }
            }

            migrate!("db/migrations");
        }
    };

    let mut file = File::create("src/lib.rs").expect("Failed to create src/lib.rs");

    file.write_all(content.to_string().as_bytes())
        .expect("Failed to create src/lib.rs");
    file.flush().expect("Failed to create src/lib.rs");
}
