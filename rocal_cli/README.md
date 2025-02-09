```bash
$ rocal init
```
- Create directories, files, and Cargo.toml that include necessary crates
- `cargo init --lib`

```
├── Cargo.toml
├── db
│   └── migrations
│       └── .keep
└── src
    ├── controllers
    │   └── root_controller.rs
    ├── controllers.rs
    ├── lib.rs
    ├── models
    │   └── .keep
    ├── models.rs
    ├── templates
    │   └── root_template.rs
    ├── templates.rs
    ├── views
    │   └── root_view.rs
    └── views.rs
```

- lib.rs
```rust
use rocal::{config, migrate, route};

mod controllers;
mod models;
mod templates;
mod views;

config! {
    app_id: "85abd3d8-4db2-4487-96f1-e763300c543a",
    sync_server_endpoint: "http://127.0.0.1:3000/presigned-url",
    database_directory_name: "local",
    database_file_name: "local.sqlite3"
}

#[rocal::main]
fn run() {
    route! {
        get "/" => { controller: RootController, action: index, view: RootView },
    }

    migrate!("db/migrations");
}
```

```bash
$ rocal build
```
- Run `wasm-pack build --target web`
