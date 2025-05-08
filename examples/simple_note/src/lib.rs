use rocal::{config, migrate, route};

mod controllers;
mod models;
mod templates;
mod view_models;
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
        get "/" => { controller: RootController, action: index, view: RootView },
        post "/notes" => { controller: NotesController, action: create, view: NotesView },
        patch "/notes/<note_id>" => { controller: NotesController, action: update, view: NotesView },
        delete "/notes/<note_id>" => { controller: NotesController, action: delete, view: NotesView }
    }
}
