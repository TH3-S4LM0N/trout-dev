use {
    std::path::PathBuf,
    serde::{Deserialize, Serialize}
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,
    pub dl_type: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Song {
    // Display Name
    pub FancyName: String,
    // share link for download
    pub Link: String,
    pub Artist: String,
    pub Album: String,
    pub Path: PathBuf
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub struct dbasesong {
    pub id: String,
    pub name: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub struct dbaseplaylist {
    pub name: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub struct database {
    pub songs: Vec<dbasesong>,
    pub playlists: Vec<dbaseplaylist>
}
// json:
/*
{
    "songs": [
        {
            "id": "aaaaaaaaaaaaaaa",
            "name": "aaaaaaaaaaaaaaa"
        }
    ],
    "playlists": [
        {
            "name": "aaaaaaaaaaaaaa"
        }
    ]
}
*/
// access with `json["data"][0]["id"]`
// or as struct `dataabse.songs[0].id`