use {
    crate::core::structs::Config,
    chrono::Local,
    futures::executor::block_on,
    std::fmt,
    tokio::{fs as tfs, fs::File as tFile},
};

pub async fn log(msg: &str, cfg: &Config) {
    let data_dir_s = format!("{}", cfg.data_dir.display());
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

pub async fn log_err(msg: &str, err: &dyn fmt::Debug, cfg: &Config) {
    let data_dir_s = format!("{}", cfg.data_dir.display());
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

pub fn log_result(msg: &str, err: &dyn fmt::Debug, cfg: &Config) -> ! {
    block_on(log_err(msg, err, cfg));
    panic!("{msg}: {err:?}");
}
pub fn log_opt(msg: &str, cfg: &Config) -> ! {
    block_on(log(msg, cfg));
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
