use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    process::{Command, Output},
};

use flate2::{write::GzEncoder, Compression};
use tar::Builder;

use crate::{
    commands::utils::{
        color::Color,
        indicator::{IndicatorLauncher, Kind},
    },
    rocal_api_client::{create_app::CreateApp, RocalAPIClient},
};

use super::{
    unsubscribe::get_subscription_status,
    utils::{get_user_input, refresh_user_token::refresh_user_token},
};

pub async fn publish() {
    refresh_user_token().await;

    if let Err(_) = get_subscription_status().await {
        println!("Need to subscribe a plan to publish your app. (`rocal subscribe` first.)");
        return;
    }

    let subdomain = match get_subdomain().await {
        Ok(Some(subdomain)) => subdomain,
        Ok(None) => {
            let mut subdomain = get_user_input("subdomain where you host this app");

            while {
                match check_subdomain_existence(&subdomain).await {
                    Ok(exists) => exists,
                    Err(err) => {
                        eprintln!("{}", err);
                        return;
                    }
                }
            } {
                println!("{} has already been taken.", &subdomain);
                subdomain = get_user_input("subdomain where you host this app");
            }

            subdomain
        }
        Err(err) => {
            eprintln!("{}", &err);
            return;
        }
    };

    let mut indicator = IndicatorLauncher::new()
        .kind(Kind::Dots)
        .interval(100)
        .text("Building...")
        .color(Color::White)
        .start();

    let root_path = find_project_root().expect(
        "Failed to find a project root. Please run the command in a project built by Cargo",
    );

    env::set_current_dir(&root_path).expect(&format!(
        "Failed to change directory to {}",
        root_path.to_str().unwrap()
    ));

    let output = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .arg("--release")
        .output()
        .expect("Confirm you run this command in a rocal project or you've installed wasm-pack");

    let _ = indicator.stop();

    if !output.status.success() {
        eprintln!(
            "rocal build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }

    create_release_artifact(&root_path).await;

    if let Some(app_name) = root_path.file_name() {
        let app_name = app_name.to_string_lossy();
        upload(&root_path.join("release.tar.gz"), &app_name, &subdomain).await;
        println!("Uploaded. Go to https://{}.rocal.dev", &subdomain);
    } else {
        eprintln!("Failed to upload your app (Reason: could not find your app name)");
    }
}

async fn create_release_artifact(root_path: &PathBuf) {
    let mut indicator = IndicatorLauncher::new()
        .kind(Kind::Dots)
        .interval(100)
        .text("Compressing...")
        .color(Color::White)
        .start();

    fs::create_dir_all("release/pkg").expect("Failed to create release/pkg");
    fs::create_dir_all("release/js").expect("Failed to create release/js");

    let pkg_files = get_all_files_in(&root_path.join("pkg")).expect("Failed to access pkg/");
    let js_files = get_all_files_in(&root_path.join("js")).expect("Failed to access js/");

    compress(
        &root_path.join("index.html"),
        &root_path.join("release/index.html.br"),
    )
    .expect("Failed to compress index.html");

    compress(
        &root_path.join("sw.js"),
        &root_path.join("release/sw.js.br"),
    )
    .expect("Failed to compress sw.js");

    pkg_files.iter().for_each(|file| {
        let to = &root_path.join("release/pkg").join(&format!(
            "{}.br",
            file.file_name().unwrap().to_str().unwrap()
        ));
        compress(&file, to).expect(&format!("Failed to compress {}", file.to_str().unwrap()));
    });

    js_files.iter().for_each(|file| {
        let to = &root_path.join("release/js").join(&format!(
            "{}.br",
            file.file_name().unwrap().to_str().unwrap()
        ));
        compress(&file, to).expect(&format!("Failed to compress {}", file.to_str().unwrap()));
    });

    let tar_gz = File::create("release.tar.gz").expect("Failed to create release.tar.gz");

    let enc = GzEncoder::new(tar_gz, Compression::default());

    let mut tar = Builder::new(enc);

    tar.append_dir_all("release", "release")
        .expect("Failed to create release.tar.gz");

    tar.finish()
        .expect("Failed to finish creating release.tar.gz");

    let _ = indicator.stop();

    println!("Generated release.tar.gz");
}

async fn upload(app_path: &PathBuf, app_name: &str, subdomain: &str) {
    let client = RocalAPIClient::new();

    if let Err(err) = client
        .upload_app(
            CreateApp::new(app_name, subdomain),
            app_path.to_str().unwrap(),
        )
        .await
    {
        eprintln!("{}", &err);
    }
}

async fn get_subdomain() -> Result<Option<String>, String> {
    let root_path = find_project_root().expect(
        "Failed to find a project root. Please run the command in a project built by Cargo",
    );

    let app_name = root_path
        .file_name()
        .expect("Failed to find your app name")
        .to_string_lossy();

    let client = RocalAPIClient::new();

    match client.get_subdomain(&app_name).await {
        Ok(Some(subdomain)) => Ok(Some(subdomain.get_subdomain().to_string())),
        Err(err) => Err(err),
        _ => Ok(None),
    }
}

async fn check_subdomain_existence(subdomain: &str) -> Result<bool, String> {
    let client = RocalAPIClient::new();

    match client.check_subdomain_existence(subdomain).await {
        Ok(exists) => Ok(exists),
        Err(err) => Err(format!("{}", err.to_string())),
    }
}

fn compress(from: &PathBuf, to: &PathBuf) -> std::io::Result<Output> {
    let output = Command::new("brotli")
        .arg("--best")
        .arg(from)
        .arg("-o")
        .arg(to)
        .output()?;

    Ok(output)
}

fn get_all_files_in(path: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

fn find_project_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return Some(current_dir);
        }
        if !current_dir.pop() {
            break;
        }
    }
    None
}
