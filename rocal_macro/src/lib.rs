#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use rocal_core::{build_action, build_config, build_route, run_migration, start_app};

/// This attribute macro should be used when you create an entrypoint of a Rocal application.
///
/// ```rust
/// use rocal::config;
///
/// #[rocal::main]
/// fn app() {}
/// ```
///
#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    start_app(item.into()).into()
}

/// This attribute macro should be used when you create an action of a controller.
///
/// ```rust
/// use crate::views::root_view::RootView;
/// use rocal::rocal_core::traits::{Controller, SharedRouter};
///
/// pub struct RootController {
///     router: SharedRouter,
///     view: RootView,
/// }
///
/// impl Controller for RootController {
///     type View = RootView;
///     fn new(router: SharedRouter, view: Self::View) -> Self {
///         RootController { router, view }
///     }
/// }
///
/// impl RootController {
///     #[rocal::action]
///     pub fn index(&self) {
///         self.view.index();
///     }
/// }
/// ```
///
#[proc_macro_attribute]
pub fn action(_: TokenStream, item: TokenStream) -> TokenStream {
    build_action(item.into()).into()
}

/// This function-like macro sets up application routing.
///
/// ```rust
/// route! {
///     get "/" => { controller: RootController , action: index , view: RootView },
///     post "/users" => { controller: UsersController, action: create, view: UserView}
/// }
///
/// ```
#[proc_macro]
pub fn route(item: TokenStream) -> TokenStream {
    build_route(item.into()).into()
}

/// This function-like macro makes `static CONFIG` which contains app_id, a connection of an embedded database, and sync server endpoint URL.
///
/// ```rust
/// config! {
///     app_id: "a917e367-3484-424d-9302-f09bdaf647ae" ,
///     sync_server_endpoint: "http://127.0.0.1:3000/presigned-url" ,
///     database_directory_name: "local" ,
///     database_file_name: "local.sqlite3"
/// }
///
#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    build_config(item.into()).into()
}

/// This function-like macro allows users to set a path where migration files are supposed to be.
///
/// ```rust
/// migrate!("db/migrations");
/// ```
#[proc_macro]
pub fn migrate(item: TokenStream) -> TokenStream {
    run_migration(item.into()).into()
}
