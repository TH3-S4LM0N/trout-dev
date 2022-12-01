pub use {
    crate::core::{
        logger::ResultExt,
        structs::{database as sDatabase, Config as sConfig},
    },
    std::{fmt, path::PathBuf},
    tokio::fs as tfs,
};

pub mod audio_file;
pub mod database;
pub mod logger;
pub mod structs;

pub async fn load_cfg() -> sConfig {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("trout").unwrap();
    let cfg_path = xdg_dirs
        .find_config_file("config.toml")
        .expect("Config file not present in xdg config dir!");
    let cfg: structs::Config = toml::from_str(
        &tfs::read_to_string(cfg_path)
            .await
            .expect("Failed to read config file!")
            .trim(),
    )
    .expect("Failed to convert config to struct");
    return cfg;
}


pub async fn load_dbase(data_dir: &PathBuf) -> sDatabase {
    let database: sDatabase = serde_json::from_str(
        &tfs::read_to_string(&format!("{}/database.json", data_dir.display()))
            .await
            .log("Failed to read databasee.json", data_dir),
    )
    .log("Failed to convert database.json to JSON!", data_dir);
    return database;
}

#[cfg(debug_assertions)]
pub async fn dbg(test_b: bool) {
    dbg!(test_b);
}

