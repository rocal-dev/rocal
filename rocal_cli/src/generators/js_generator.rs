use std::{fs, path::PathBuf};

use super::absolute_path;

pub fn create_js_files() {
    let src_sw_file = absolute_path("js/sw.js");
    let dst_sw_file = PathBuf::from("sw.js");
    fs::copy(&src_sw_file, &dst_sw_file).expect("Failed to copy js/sw.js");

    fs::create_dir_all("js").expect("Failed to create js/");
    let src_files = vec![
        "db_query_worker.js",
        "db_sync_worker.js",
        "global.js",
        "sqlite3-opfs-async-proxy.js",
        "sqlite3.mjs",
        "sqlite3.wasm",
    ];

    src_files.iter().for_each(|src| {
        let src_file = absolute_path(&format!("js/{}", src));
        let dst_file = PathBuf::from(&format!("js/{}", src));
        fs::copy(&src_file, &dst_file).expect(&format!("Failed to copy {}", src));
    });
}
