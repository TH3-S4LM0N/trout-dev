use {
    crate::core::structs::Config,
    chrono::Local,
    futures::executor::block_on,
    std::{fmt, path::PathBuf},
    tokio::{fs as tfs, fs::File as tFile},
};

pub async fn log(msg: &str, data_dir: &PathBuf) {
    let data_dir_s = format!("{}", data_dir.display());
    let logfile = tfs::read_to_string(&data_dir_s)
        .await
        .expect("Failed to read logfile");
    tfs::write(
        &data_dir_s,
        format!("{}\n{} :: {}", logfile, Local::now().format("%H:%M:%S"), msg),
    )
    .await
    .expect("Failed to write logfile");
}

pub async fn log_err(msg: &str, err: &dyn fmt::Debug, data_dir: &PathBuf) {
    let data_dir_s = format!("{}", data_dir.display());
    let logfile = tfs::read_to_string(&data_dir_s)
        .await
        .expect("Failed to read logfile");
    tfs::write(
        &data_dir_s,
        format!(
            "{}\n{} :: {}: {:?}",
            logfile,
            Local::now().format("%H:%M:%S"),
            msg,
            err
        ),
    )
    .await
    .expect("Failed to write logfile");
}

pub fn log_result(msg: &str, err: &dyn fmt::Debug, data_dir: &PathBuf) -> ! {
    block_on(log_err(msg, err, data_dir));
    panic!("{msg}: {err:?}");
}
pub fn log_opt(msg: &str, data_dir: &PathBuf) -> ! {
    block_on(log(msg, data_dir));
    panic!("{msg}");
}


pub async fn init_logger(cfg: &Config) {
    let data_dir_s = format!("{}", &cfg.data_dir.display());
    tFile::create(&format!("{}/log", &data_dir_s))
        .await
        .expect("Failed to create logfile!");

    tfs::write(
        &data_dir_s,
        &format!(
            "{} :: Created trout logfile",
            Local::now().format("%H:%M:%S")
        ),
    )
    .await
    .expect("Failed to write logfile!");
}

// trait to add a log method to replace `.expect` for Result<T, E>
pub trait ResultExt<T, E> {
    fn log(self, msg: &str, data_dir: &PathBuf) -> T
    where
        E: fmt::Debug;
}
impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn log(self, msg: &str, data_dir: &PathBuf) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => log_result(msg, &e, data_dir),
        }
    }
}

// trait to add log method to replace `.expect` for Option<T>
pub trait OptionExt<T> {
    fn log(self, msg: &str, data_dir: &PathBuf) -> T;
}
impl<T> OptionExt<T> for Option<T> {
    fn log(self, msg: &str, data_dir: &PathBuf) -> T {
        match self {
            Some(val) => val,
            None => log_opt(msg, data_dir)
        }
    }
}
