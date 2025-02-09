use std::{fs, path::PathBuf};

macro_rules! copy_files {
    ( $( $filename:literal ),* $(,)? ) => {{
        $(
            let src_file = include_bytes!(concat!("../../js/", $filename));
            let dst_file = std::path::PathBuf::from(&format!("js/{}", $filename));
            std::fs::write(&dst_file, src_file).expect(&format!("Failed to copy {}", $filename));
        )*

    }};
}

pub fn create_js_files() {
    let src_sw_file = include_bytes!("../../js/sw.js");
    let dst_sw_file = PathBuf::from("sw.js");
    fs::write(&dst_sw_file, src_sw_file).expect("Failed to copy js/sw.js");

    fs::create_dir_all("js").expect("Failed to create js/");

    copy_files![
        "db_query_worker.js",
        "db_sync_worker.js",
        "global.js",
        "sqlite3-opfs-async-proxy.js",
        "sqlite3.mjs",
        "sqlite3.wasm",
    ];
}
