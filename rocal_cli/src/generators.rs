use std::{
    env::{self},
    path::{Path, PathBuf},
};

pub mod cargo_file_generator;
pub mod controller_generator;
pub mod entrypoint_generator;
pub mod js_generator;
pub mod lib_generator;
pub mod migration_generator;
pub mod model_generator;
pub mod template_generator;
pub mod view_generator;

fn absolute_path(file_path: &str) -> PathBuf {
    let base_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is not set; please use Cargo to build");
    let base_dir = Path::new(&base_dir);

    base_dir.join(file_path)
}
