use std::fs::{self, File};

pub fn create_public_dir() {
    fs::create_dir("public").expect("Failed to create public/");
    File::create("public/.keep").expect("Failed to create public/.keep");
}
