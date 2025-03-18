use std::{env, process::Command};

use crate::generators::{
    cargo_file_generator::create_cargo_file, controller_generator::create_controller_file,
    entrypoint_generator::create_entrypoint, gitignore_generator::create_gitignore,
    js_generator::create_js_files, lib_generator::create_lib_file,
    migration_generator::create_migration_dir, model_generator::create_model_file,
    template_generator::create_template_file, view_generator::create_view_file,
};

use crate::commands::utils::{
    color::Color,
    indicator::{IndicatorLauncher, Kind},
};

pub fn init(project_name: &str) {
    let mut indicator = IndicatorLauncher::new()
        .kind(Kind::Dots)
        .interval(100)
        .text("Initializing...")
        .color(Color::White)
        .start();

    let output = Command::new("cargo")
        .arg("init")
        .arg("--lib")
        .arg(project_name)
        .output()
        .expect("Failed to execute cargo init");

    if output.status.success() {
        let _ = indicator.stop();

        env::set_current_dir(project_name).expect(&format!(
            "Failed to change a current directory: {}",
            &project_name
        ));

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating Cargo.toml...")
            .color(Color::White)
            .start();

        create_cargo_file(project_name);

        let _ = indicator.stop();
        println!("Created Cargo.toml");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating lib.rs...")
            .color(Color::White)
            .start();

        create_lib_file();

        let _ = indicator.stop();

        println!("Created lib.rs");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating (a) template file(s)...")
            .color(Color::White)
            .start();

        create_template_file();

        let _ = indicator.stop();

        println!("Created (a) template file(s)");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating (a) view file(s)...")
            .color(Color::White)
            .start();

        create_view_file();

        let _ = indicator.stop();

        println!("Created (a) view file(s)");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating (a) controller file(s)...")
            .color(Color::White)
            .start();

        create_controller_file();

        let _ = indicator.stop();

        println!("Created (a) controller file(s)");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating models/ directory...")
            .color(Color::White)
            .start();

        create_model_file();

        let _ = indicator.stop();

        println!("Created models/ directory");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating db/migration directory...")
            .color(Color::White)
            .start();

        create_migration_dir();

        let _ = indicator.stop();

        println!("Created db/migration directory");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating js files...")
            .color(Color::White)
            .start();

        create_js_files();

        let _ = indicator.stop();

        println!("Created js files");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating index.html...")
            .color(Color::White)
            .start();

        create_entrypoint(project_name);

        let _ = indicator.stop();

        println!("Created index.html");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Creating .gitignore...")
            .color(Color::White)
            .start();

        create_gitignore();

        let _ = indicator.stop();

        println!("Created .gitignore");

        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Formatting...")
            .color(Color::White)
            .start();

        Command::new("cargo")
            .arg("fmt")
            .arg("--all")
            .output()
            .expect("Failed to format Rust code");

        let _ = indicator.stop();

        println!("Done.");
    } else {
        let _ = indicator.stop();

        eprintln!(
            "cargo init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
