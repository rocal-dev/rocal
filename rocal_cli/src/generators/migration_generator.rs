use std::fs::{self, File};

pub fn create_migration_dir() {
    fs::create_dir_all("db/migrations").expect("Failed to create db/migrations");
    File::create("db/migrations/.keep").expect("Failed to create db/migrations/.keep");
}
