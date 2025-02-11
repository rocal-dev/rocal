use std::{env, process::Command};

use crate::generators::{
    cargo_file_generator::create_cargo_file, controller_generator::create_controller_file,
    entrypoint_generator::create_entrypoint, gitignore_generator::create_gitignore,
    js_generator::create_js_files, lib_generator::create_lib_file,
    migration_generator::create_migration_dir, model_generator::create_model_file,
    template_generator::create_template_file, view_generator::create_view_file,
};

pub fn init(project_name: &str) {
    println!("Initializing...");

    let output = Command::new("cargo")
        .arg("init")
        .arg("--lib")
        .arg(project_name)
        .output()
        .expect("Failed to execute cargo init");

    if output.status.success() {
        env::set_current_dir(project_name).expect(&format!(
            "Failed to change a current directory: {}",
            &project_name
        ));

        create_cargo_file(project_name);
        println!("Created Cargo.toml");

        create_lib_file();
        println!("Created lib.rs");

        create_template_file();
        println!("Created (a) template file(s)");

        create_view_file();
        println!("Created (a) view file(s)");

        create_controller_file();
        println!("Created (a) controller file(s)");

        create_model_file();
        println!("Created models/ directory");

        create_migration_dir();
        println!("Created db/migration directory");

        create_js_files();
        println!("Created js files");

        create_entrypoint(project_name);
        println!("Created index.html");

        create_gitignore();
        println!("Created .gitignore");

        Command::new("cargo")
            .arg("fmt")
            .arg("--all")
            .output()
            .expect("Failed to format Rust code");

        println!("Done.");
    } else {
        eprintln!(
            "cargo init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
