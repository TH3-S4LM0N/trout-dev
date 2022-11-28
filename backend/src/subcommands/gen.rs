use {
    crate::core::structs::{database, dbaseplaylist, dbasesong},
    serde_json::json,
    std::vec,
    tokio::fs::{create_dir as tcreate_dir, write as twrite, File as tFile},
};

pub async fn gen(data_dir: &str) -> String {
    // too lazy currently to check if stuff already exists
    // so ill just let panic cause the message is pretty obvious

    // create xdg compliant config file
    let xdg_dirs = xdg::BaseDirectories::with_prefix("trout").unwrap();
    let cfg_path = xdg_dirs
        .place_config_file("config.toml")
        .expect("Failed to create xdg config path!");
    tFile::create(&cfg_path)
        .await
        .expect("Failed to create config file!");
    twrite(
        &cfg_path,
        format!(
            r#"data_dir = {:?}
# can be flac, mp3, ogg, opus, or m4a
dl_type = "flac"
"#,
            data_dir
        ),
    )
    .await
    .expect("Failed to write config file!");

    // create data dir
    tcreate_dir(data_dir)
        .await
        .expect(&format!("Failed to create {}", data_dir));
    // create raw dir to store music
    tcreate_dir(&format!("{}/raw", data_dir))
        .await
        .expect(&format!("Failed to create {}/raw", data_dir));
    // create dir to store playlist jsons
    tcreate_dir(&format!("{}/playlists", data_dir))
        .await
        .expect(&format!("Failed to create {}/playlists", data_dir));
    // create dir to store song jsons
    tcreate_dir(&format!("{}/songs", data_dir))
        .await
        .expect(&format!("Failed to create {}/songs", data_dir));
    // create dir to store temp data
    tcreate_dir(&format!("{}/tmp", data_dir))
        .await
        .expect(&format!("Failed to create {}/tmp", data_dir));

    // create database
    let dbase = format!("{}/database.json", data_dir);
    tFile::create(&dbase)
        .await
        .expect("Failed to create database.json!");

    twrite(
        &dbase,
        serde_json::to_string_pretty(&json!(database {
            songs: vec![dbasesong {
                id: String::new(),
                name: String::new()
            }],
            playlists: vec![dbaseplaylist {
                name: String::new()
            }]
        }))
        .unwrap(),
    )
    .await
    .expect("Failed to write blank to database.json!");

    format!("Succesfully created {}", data_dir)
}
