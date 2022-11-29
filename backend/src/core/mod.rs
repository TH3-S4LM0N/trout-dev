pub use {
    crate::core::{
        logger as clogger, // shut up its a 'logger' from the 'core' mod
        structs::{database as sDatabase, Config as sConfig},
    },
    std::fmt,
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


pub async fn load_dbase() -> sDatabase {
    let cfg = load_cfg().await;
    let database: sDatabase = serde_json::from_str(
        &tfs::read_to_string(&format!("{}/database.json", cfg.data_dir.display()))
            .await
            .expect("Failed to read databasee.json"),
    )
    .expect("Failed to convert database.json to JSON!");
    return database;
}

#[cfg(debug_assertions)]
pub async fn dbg(test_b: bool) {
    dbg!(test_b);
}

// trait to add a log method to replace `.expect` for Result<T, E>
pub trait ResultExt<T, E> {
    fn log(self, msg: &str, cfg: &sConfig) -> T
    where
        E: fmt::Debug;
}
impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn log(self, msg: &str, cfg: &sConfig) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => clogger::log_result(msg, &e, cfg),
        }
    }
}

// trait to add log method to replace `.expect` for Option<T>
pub trait OptionExt<T> {
    fn log(self, msg: &str, cfg: &sConfig) -> T;
}
impl<T> OptionExt<T> for Option<T> {
    fn log(self, msg: &str, cfg: &sConfig) -> T {
        match self {
            Some(val) => val,
            None => clogger::log_opt(msg, cfg)
        }
    }
}
