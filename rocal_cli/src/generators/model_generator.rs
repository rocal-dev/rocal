use std::fs::{self, File};

pub fn create_model_file() {
    fs::create_dir_all("src/models").expect("Failed to create src/models");
    File::create("src/models/.keep").expect("Failed to create src/models/.keep");
    File::create("src/models.rs").expect("Failed to create src/models.rs");
}
