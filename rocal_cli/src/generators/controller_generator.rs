use std::{
    fs::{self, File},
    io::Write,
};

use quote::quote;

pub fn create_controller_file() {
    let root_controller_content = quote! {
        use rocal::rocal_core::traits::{Controller, SharedRouter};
        use crate::views::root_view::RootView;

        pub struct RootController {
            router: SharedRouter,
            view: RootView,
        }

        impl Controller for RootController {
            type View = RootView;

            fn new(router: SharedRouter, view: Self::View) -> Self {
                RootController { router, view }
            }
        }

        impl RootController {
            #[rocal::action]
            pub fn index(&self) {
                self.view.index();
            }
        }
    };

    let controller_content = quote! {
        pub mod root_controller;
    };

    fs::create_dir_all("src/controllers").expect("Failed to create src/controllers");

    let mut root_controller_file = File::create("src/controllers/root_controller.rs")
        .expect("Failed to create src/controllers/root_controller.rs");

    root_controller_file
        .write_all(root_controller_content.to_string().as_bytes())
        .expect("Failed to create src/controllers/root_controller.rs");
    root_controller_file
        .flush()
        .expect("Failed to create src/controllers/root_controller.rs");

    let mut controller_file =
        File::create("src/controllers.rs").expect("Failed to create src/controllers.rs");

    controller_file
        .write_all(controller_content.to_string().as_bytes())
        .expect("Failed to create src/controllers.rs");
    controller_file
        .flush()
        .expect("Failed to create src/controllers.rs");
}
