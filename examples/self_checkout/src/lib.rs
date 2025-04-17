use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use models::flash_memory::FlashMemory;
use rocal::{config, migrate, route};

mod controllers;
mod models;
mod repositories;
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

static FLASH_MEMORY: LazyLock<Mutex<FlashMemory>> =
    LazyLock::new(|| Mutex::new(FlashMemory::new(Arc::new(Mutex::new(HashMap::new())))));

#[rocal::main]
fn app() {
    migrate!("db/migrations");

    route! {
        get "/" => { controller: RootController, action: index, view: RootView },
        post "/carts/<product_id>" => { controller: CartsController, action: add, view: EmptyView },
        delete "/carts/<product_id>" => { controller: CartsController, action: delete, view: EmptyView },
        get "/sales" => { controller: SalesController, action: index, view: SalesView },
        get "/sales/<id>" => { controller: SalesController, action: show, view: SalesView },
        post "/sales/checkout" => { controller: SalesController, action: checkout, view: SalesView }
    }
}
