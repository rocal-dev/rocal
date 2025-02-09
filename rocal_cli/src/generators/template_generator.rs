use std::{
    fs::{self, File},
    io::Write,
};

use quote::quote;

pub fn create_template_file() {
    let root_template_content = quote! {
        use rocal::rocal_core::traits::{SharedRouter, Template};

        pub struct RootTemplate {
            router: SharedRouter,
        }

        impl Template for RootTemplate {
            type Data = String;

            fn new(router: SharedRouter) -> Self {
                RootTemplate { router }
            }

            fn body(&self, data: Self::Data) -> String {
                let mut html = String::from("<h1>Welcome to rocal world!</h1>");

                html += &format!("<p>{}</p>", &data);

                html
            }

            fn router(&self) -> SharedRouter {
                self.router.clone()
            }
        }
    };

    let template_content = quote! {
        pub mod root_template;
    };

    fs::create_dir_all("src/templates").expect("Failed to create src/templates");

    let mut root_template_file = File::create("src/templates/root_template.rs")
        .expect("Failed to create src/templates/root_template.rs");

    root_template_file
        .write_all(root_template_content.to_string().as_bytes())
        .expect("Failed to create src/templates/root_template.rs");
    root_template_file
        .flush()
        .expect("Failed to create src/templates/root_template.rs");

    let mut template_file =
        File::create("src/templates.rs").expect("Failed to create src/templates.rs");

    template_file
        .write_all(template_content.to_string().as_bytes())
        .expect("Failed to create src/templates.rs");
    template_file
        .flush()
        .expect("Failed to create src/templates.rs");
}
