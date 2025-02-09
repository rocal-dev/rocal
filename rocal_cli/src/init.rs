use std::{env, process::Command};

use crate::generators::{
    cargo_file_generator::create_cargo_file, controller_generator::create_controller_file,
    entrypoint_generator::create_entrypoint, js_generator::create_js_files,
    lib_generator::create_lib_file, migration_generator::create_migration_dir,
    model_generator::create_model_file, template_generator::create_template_file,
    view_generator::create_view_file,
};

pub fn init(project_name: &str) {
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
        create_lib_file();
        create_template_file();
        create_view_file();
        create_controller_file();
        create_model_file();
        create_migration_dir();
        create_js_files();
        create_entrypoint(project_name);
    } else {
        eprintln!(
            "cargo init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
