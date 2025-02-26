use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    process::{Command, Output},
};

use flate2::{write::GzEncoder, Compression};
use tar::Builder;

pub fn publish() {
    println!("Building...");

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

    if !output.status.success() {
        eprintln!(
            "rocal build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }

    println!("Compressing...");
    create_release_artifact(&root_path);
    println!("Compressed.");

    println!("Done.");
}

fn create_release_artifact(root_path: &PathBuf) {
    fs::create_dir_all("release/pkg").expect("Failed to create release/pkg");
    fs::create_dir_all("release/js").expect("Failed to create release/js");

    let pkg_files = get_all_files_in(&root_path.join("pkg")).expect("Failed to access pkg/");
    let js_files = get_all_files_in(&root_path.join("js")).expect("Failed to access js/");

    compress(
        &root_path.join("index.html"),
        &root_path.join("release/index.html.br"),
    )
    .expect("Failed to compress index.html");

    println!("Compressed index.html");

    compress(
        &root_path.join("sw.js"),
        &root_path.join("release/sw.js.br"),
    )
    .expect("Failed to compress sw.js");

    println!("Compressed sw.js");

    pkg_files.iter().for_each(|file| {
        let to = &root_path.join("release/pkg").join(&format!(
            "{}.br",
            file.file_name().unwrap().to_str().unwrap()
        ));
        compress(&file, to).expect(&format!("Failed to compress {}", file.to_str().unwrap()));
    });

    println!("Compressed pkg/");

    js_files.iter().for_each(|file| {
        let to = &root_path.join("release/js").join(&format!(
            "{}.br",
            file.file_name().unwrap().to_str().unwrap()
        ));
        compress(&file, to).expect(&format!("Failed to compress {}", file.to_str().unwrap()));
    });

    println!("Compressed js/");

    println!("Generating release file...");

    let tar_gz = File::create("release.tar.gz").expect("Failed to create release.tar.gz");

    let enc = GzEncoder::new(tar_gz, Compression::default());

    let mut tar = Builder::new(enc);

    tar.append_dir_all("release", "release")
        .expect("Failed to create release.tar.gz");

    tar.finish()
        .expect("Failed to finish creating release.tar.gz");

    println!("Generated release.tar.gz");
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
