use {
    crate::core::structs::{database, Config},
    serde_json::{json, Value},
    std::path::PathBuf,
    tokio::{fs as tfs, process::Command as tCommand},
};

pub async fn new_song(link: &str) -> String {
    let (cfg, dbase): (Config, database) = crate::core::load().await;

    // &cfg.data_dir as a srting
    
    let data_dir_string = format!("{}", &cfg.data_dir.display());
    // path to tmpfile
    let tmpfile = format!("{}/tmp.spotdl", &data_dir_string);

    // command to run spotdl
    // downloads audio song to data_dir/raw
    let mut cmd = tCommand::new("spotdl")
        .args([
            "download",
            link,
            "--save-file",
            &tmpfile,
            "--output",
            &format!("{}/raw/{{track-id}}.{{output-ext}}", &data_dir_string),
            "--format",
            &cfg.dl_type,
            "--preload",
        ])
        .spawn()
        .expect("Failed to create download command!");
    cmd.wait().await.expect("Failed to run download command!");

    // contents of the savefile spotdl makes, containing json song values
    let song_json: Vec<Value> = serde_json::from_str(
        &tfs::read_to_string(&tmpfile)
            .await
            .expect("Failed to create read tmpfile!"),
    )
    .expect("Failed to read tmpfile into JSON!");

    // json data that will be written to data_dir/songs/id.json
    let new_song_json = json!({
        "FancyName": song_json[0]["name"],
        "Link": link,
        "Artist": song_json[0]["artist"],
        "Album": song_json[0]["album_name"],
        "Path": &format!("{}/raw/{}.{}", &data_dir_string, &song_json[0]["song_id"], &cfg.dl_type)
    });

    // path to save new_song_json at
    let song_json_path: PathBuf = {
        let mut song_path = PathBuf::new();
        song_path = song_path.join(&cfg.data_dir);
        song_path.push(&song_json[0]["song_id"].as_str().unwrap().trim_matches('"'));
        song_path.set_extension("json");
        song_path
    };

    tfs::File::create(&song_json_path)
        .await
        .expect("Failed to create song file!");

    tfs::write(
        &song_json_path,
        serde_json::to_string_pretty(&new_song_json).unwrap(),
    )
    .await
    .expect("Failed to write json file!");

    tfs::remove_file(&tmpfile)
        .await
        .expect("Failed to remove temp file!");

    crate::core::database::add(
        &song_json[0]["song_id"],
        &song_json[0]["name"],
        &cfg.data_dir,
    )
    .await;

    format!(
        "Created {} at {}!",
        song_json[0]["name"],
        &song_json_path.into_os_string().into_string().expect("bruh")
    )
}

pub async fn new_playlist(_link: &str) -> String {
    String::new()
}
