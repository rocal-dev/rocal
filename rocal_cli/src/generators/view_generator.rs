use std::{
    fs::{self, File},
    io::Write,
};

use quote::quote;

pub fn create_view_file() {
    let root_view_content = quote! {
        use rocal::rocal_core::traits::{SharedRouter, Template, View};

        use crate::templates::root_template::RootTemplate;

        pub struct RootView {
            router: SharedRouter,
        }

        impl View for RootView {
            fn new(router: SharedRouter) -> Self {
                RootView { router }
            }
        }

        impl RootView {
            pub fn index(&self) {
                let template = RootTemplate::new(self.router.clone());
                template.render(String::new());
            }
        }
    };

    let view_content = quote! {
        pub mod root_view;
    };

    fs::create_dir_all("src/views").expect("Failed to create src/views");

    let mut root_view_file =
        File::create("src/views/root_view.rs").expect("Failed to create src/views/root_view.rs");

    root_view_file
        .write_all(root_view_content.to_string().as_bytes())
        .expect("Failed to create src/views/root_view.rs");
    root_view_file
        .flush()
        .expect("Failed to create src/views/root_view.rs");

    let mut view_file = File::create("src/views.rs").expect("Failed to create src/views.rs");

    view_file
        .write_all(view_content.to_string().as_bytes())
        .expect("Failed to create src/views.rs");
    view_file.flush().expect("Failed to create src/views.rs");
}
