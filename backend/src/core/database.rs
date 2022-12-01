use {
    crate::core::{structs::{database, dbasesong}, logger::ResultExt},
    serde_json::{json, Value},
    std::path::PathBuf,
    tokio::fs as tfs,
};

// function to add song database.json
pub async fn add(id: &Value, name: &Value, data_dir: &PathBuf) {
    // path to database.json
    let dbasepath = format!("{}/database.json", data_dir.display());

    // database.json as struct
    let mut databasest: database = serde_json::from_str(
        &tfs::read_to_string(&dbasepath)
            .await
            .log("Failed to read database!", data_dir),
    )
    .log("Failed to convert database to JSON!", data_dir);

    // add new song
    databasest.songs.push(dbasesong {
        id: id.to_string(),
        name: name.to_string(),
    });

    // write back to database.json
    tfs::write(&dbasepath, json!(&databasest).to_string())
        .await
        .log("Failed to write database!", data_dir);
}
