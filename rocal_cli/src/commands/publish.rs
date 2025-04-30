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

use super::utils::{
    get_user_input,
    project::{find_project_root, get_app_name},
    refresh_user_token::refresh_user_token,
};

pub async fn publish() {
    refresh_user_token().await;

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

    if fs::exists("release").expect("Failed to check existence of release/") {
        fs::remove_dir_all("release").expect("Failed to reset release/");
    }

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

        if let Err(err) = upload(&root_path.join("release.tar.gz"), &app_name, &subdomain).await {
            eprintln!("{}", Color::Red.text(&err));
            return;
        }

        extract(&subdomain).await;
        println!("Uploaded. Go to https://{}.rocal.app", &subdomain);
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

    create_public_dir_in_release_dir();
    if let Err(err) =
        compress_public_dir(&root_path.join("public"), &root_path.join("release/public"))
    {
        panic!("{}", &err.to_string());
    }

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

async fn upload(app_path: &PathBuf, app_name: &str, subdomain: &str) -> Result<(), String> {
    let client = RocalAPIClient::new();

    if let Err(err) = client
        .upload_app(
            CreateApp::new(app_name, subdomain),
            app_path.to_str().unwrap(),
        )
        .await
    {
        return Err(err);
    }

    Ok(())
}

async fn extract(subdomain: &str) {
    let client = RocalAPIClient::new();

    if let Err(err) = client.extract_app(subdomain).await {
        eprintln!("{}", &err);
    }
}

async fn get_subdomain() -> Result<Option<String>, String> {
    let root_path = find_project_root().expect(
        "Failed to find a project root. Please run the command in a project built by Cargo",
    );

    let app_name = get_app_name(&root_path);

    let client = RocalAPIClient::new();

    match client.get_subdomain(&app_name).await {
        Ok(Some(subdomain)) => Ok(Some(subdomain.get_subdomain().to_string())),
        Err(err) => Err(err),
        _ => Ok(None),
    }
}

fn create_public_dir_in_release_dir() {
    fs::create_dir_all("release/public").expect("Failed to create release/public");
}

fn compress_public_dir(lookup_path: &PathBuf, dst_path: &PathBuf) -> std::io::Result<()> {
    for entry in fs::read_dir(&lookup_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let to = &dst_path.join(&format!(
                "{}.br",
                path.file_name().unwrap().to_str().unwrap()
            ));
            compress(&path, to).expect(&format!("Failed to compress {}", path.to_str().unwrap()));
        } else if path.is_dir() {
            let dst_path = dst_path.join(path.file_name().unwrap().to_str().unwrap());
            fs::create_dir(&dst_path).expect(&format!(
                "Failed to create {} directory",
                &dst_path.to_str().unwrap()
            ));
            compress_public_dir(&path, &dst_path)?;
        }
    }

    Ok(())
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
