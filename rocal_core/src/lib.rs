#![doc = include_str!("../README.md")]

use configuration::{build_config_struct, parse_config};
use database::build_database_struct;
use enums::request_method::RequestMethod;
use migrator::get_migrations;
use parsed_action::parse_action;
use parsed_route::parse_routes;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_str, Ident, ItemFn, Path};
use utils::to_snake_case;
use workers::db_sync_worker::build_db_sync_worker_struct;

mod configuration;
mod database;
pub mod enums;
mod migrator;
mod parsed_action;
mod parsed_route;
pub mod route_handler;
pub mod router;
pub mod traits;
mod utils;
pub mod workers;

pub fn start_app(item: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(item.into()).unwrap();
    let stmts = &ast.block.stmts;

    let database_struct = build_database_struct();
    let db_sync_worker_struct = build_db_sync_worker_struct();

    quote! {
        use wasm_bindgen::prelude::*;
        use rocal::rocal_core::traits::{Controller, View};

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = execSQL)]
            fn exec_sql(db: &str, quer: &str) -> JsValue;
        }

        #[wasm_bindgen(start)]
        pub async fn run() {
            #(#stmts)*

            let db_sync_worker = crate::DbSyncWorker::new(
                "./js/db_sync_worker.js",
                crate::ForceType::None
            );
            db_sync_worker.run();
        }

        #database_struct
        #db_sync_worker_struct
    }
}

pub fn build_route(item: TokenStream) -> TokenStream {
    let routes = match parse_routes(item) {
        Ok(routes) => routes,
        Err(err) => return err.to_compile_error().into(),
    };

    let routes = routes.into_iter().map(|route| {
        let controller = route
            .get_controller()
            .clone()
            .expect("Controller should be here");
        let controller_mod_name =
            Ident::new(&to_snake_case(&controller.to_string()), Span::call_site());
        let view = route.get_view().clone().expect("View should be here");
        let view_mod_name = Ident::new(&to_snake_case(&view.to_string()), Span::call_site());
        let method: Path = match route.get_method() {
            Some(RequestMethod::Get) => {
                parse_str("rocal::rocal_core::enums::request_method::RequestMethod::Get")
                    .expect("Failed to parse the enum")
            }
            Some(RequestMethod::Post) => {
                parse_str("rocal::rocal_core::enums::request_method::RequestMethod::Post")
                    .expect("Failed to parse the enum")
            }
            _ => panic!("Method should be get or post"),
        };
        let path = route.get_path().clone().expect("Path should be here");
        let action = route.get_action().clone().expect("Action shuold be here");

        let ctrl = Ident::new(
            &format!("{}_{}", "ctrl_", controller.to_string()),
            Span::call_site(),
        );

        quote! {
            let #ctrl = std::rc::Rc::new(crate::controllers::#controller_mod_name::#controller::new(
                router.clone(),
                crate::views::#view_mod_name::#view::new(router.clone()),
            ));

            router
                .clone()
                .borrow_mut()
                .register(#method, #path, {
                    let #ctrl = std::rc::Rc::clone(&#ctrl);
                    Box::new(move |args| {
                        Box::pin({
                            let #ctrl = std::rc::Rc::clone(&#ctrl);
                            async move { #ctrl.#action(args).await }
                        })
                    })
                });
        }
    });

    quote! {
        let router = std::rc::Rc::new(std::cell::RefCell::new(rocal::rocal_core::router::Router::new()));

        #(#routes)*

        let route_handler = rocal::rocal_core::route_handler::RouteHandler::new(router, None);
        let route_handler = std::rc::Rc::new(route_handler);

        route_handler.handle_route().await;

        let handle_route_closure = {
            let route_handler = std::rc::Rc::clone(&route_handler);
            Closure::wrap(Box::new(move || {
                let route_handler = std::rc::Rc::clone(&route_handler);
                wasm_bindgen_futures::spawn_local(async move {
                    route_handler.handle_route().await;
                });
            }) as Box<dyn Fn()>)
        };

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("popstate", handle_route_closure.as_ref().unchecked_ref())
            .unwrap();
        handle_route_closure.forget();
    }
}

pub fn build_config(item: TokenStream) -> TokenStream {
    let config_struct = build_config_struct();
    let config = match parse_config(item) {
        Ok(config) => config,
        Err(err) => return err.to_compile_error().into(),
    };

    let app_id = config.get_app_id().clone().unwrap_or(String::new());
    let sync_server_endpoint = config
        .get_sync_server_endpoint()
        .clone()
        .unwrap_or(String::new());
    let database_directory_name = config
        .get_database_directory_name()
        .clone()
        .unwrap_or(String::new());
    let database_file_name = config
        .get_database_file_name()
        .clone()
        .unwrap_or(String::new());

    quote! {
        #config_struct

        static CONFIG: std::sync::LazyLock<crate::Configuration> = std::sync::LazyLock::new(|| {
            crate::Configuration::new(
                #app_id.to_string(),
                #sync_server_endpoint.to_string(),
                std::sync::Arc::new(Database::new(
                    #database_directory_name.to_string(),
                    #database_file_name.to_string(),
                )),
            )
        });
    }
}

pub fn build_action(item: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse2(item).unwrap();

    let parsed_action = match parse_action(&ast) {
        Ok(action) => action,
        Err(err) => return err.to_compile_error().into(),
    };

    let fn_name = parsed_action.get_name();

    let build_args = parsed_action.get_args().iter().map(|arg| {
        let name = arg.get_name();
        let name_str = name.to_string();

        let is_optional = arg.get_is_optional();

        let mut result = if *is_optional {
            quote! {
                let #name = if let Some(#name) = args.get(#name_str) {
                    Some(#name.clone())
                } else {
                    None
                };
            }
        } else {
            quote! {
                let #name = args.get(#name_str).expect(&format!("{} is required", #name_str));
            }
        };

        let ty = arg.get_ty();
        let ty_str = ty.to_string();

        result = if ty == "String" || ty == "str" {
            quote!(#result)
        } else {
            if *is_optional {
                quote! {
                    #result

                    let #name = if let Some(#name) = #name {
                        if let Ok(#name) = #name.parse::<#ty>() {
                            Some(#name)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            } else {
                quote! {
                    #result

                    let #name = #name.parse::<#ty>().expect(&format!("{} cannot be parsed as {}", #name_str, #ty_str));
                }
            }
        };

        result
    });

    let stmts = &ast.block.stmts;

    quote! {
        pub async fn #fn_name(&self, args: std::collections::HashMap<String, String>) {
            #(#build_args)*

            #(#stmts)*
        }
    }
}

pub fn run_migration(item: TokenStream) -> TokenStream {
    let query = match get_migrations(&item) {
        Ok(query) => query,
        Err(err) => return err.to_compile_error().into(),
    };

    if !query.is_empty() {
        quote! {
            match CONFIG.get_database().exec(#query).await {
                Ok(_) => (),
                Err(err) => web_sys::console::error_1(&err),
            }
        }
    } else {
        quote!()
    }
}
