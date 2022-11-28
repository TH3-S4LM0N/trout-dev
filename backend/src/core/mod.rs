use {
    crate::core::structs::{Config as aConfig, database as aDatabase},
    tokio::fs as tfs
};

pub mod database;
pub mod structs;
pub mod audio_file;

pub async fn load() -> (aConfig, aDatabase) {
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

    let database: aDatabase = serde_json::from_str(
        &tfs::read_to_string(&format!("{}/database.json", cfg.data_dir.display()))
            .await
            .expect("Failed to read databasee.json"),
    )
    .expect("Failed to convert database.json to JSON!");
    return (cfg, database);
}
pub async fn dbg(test_b: bool) {
    println!("sleeping");
    sleep(10);
    println!("done sleeping");
    return "hi".to_string()
}

fn sleep(time: u64) {
    std::thread::sleep(tokio::time::Duration::from_secs(time));
}