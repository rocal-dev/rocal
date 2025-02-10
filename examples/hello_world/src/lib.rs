use rocal::{config, migrate, route};
mod controllers;
mod models;
mod repositories;
mod templates;
mod views;

config! {
    app_id: "a917e367-3484-424d-9302-f09bdaf647ae" ,
    sync_server_endpoint: "http://127.0.0.1:3000/presigned-url" ,
    database_directory_name: "local" ,
    database_file_name: "local.sqlite3"
}

#[rocal::main]
fn app() {
    route! {
        get "/" => { controller: RootController , action: index , view: RootView },
        get "/sync-connections" => { controller: SyncConnectionsController, action: edit, view: SyncConnectionView },
        post "/sync-connections" => { controller: SyncConnectionsController, action: connect, view: SyncConnectionView }
    }
    migrate!("db/migrations");
}
