use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use quote::quote;

pub fn create_template_file() {
    let template_content = quote! {
        pub mod root_template;
    };

    fs::create_dir_all("src/templates").expect("Failed to create src/templates");

    let src_file = include_bytes!("../../seeds/root_template.rs");
    let dst_file = PathBuf::from("src/templates/root_template.rs");
    fs::write(&dst_file, src_file).expect("Failed to copy root_template.rs");

    let mut template_file =
        File::create("src/templates.rs").expect("Failed to create src/templates.rs");

    template_file
        .write_all(template_content.to_string().as_bytes())
        .expect("Failed to create src/templates.rs");
    template_file
        .flush()
        .expect("Failed to create src/templates.rs");
}
