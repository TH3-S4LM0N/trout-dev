use {
    crate::sites::spotify,
    std::path::PathBuf
};

pub async fn new(playlist: String, site: String, link: String, data_dir: &PathBuf) -> String {
    if playlist.is_empty() {} else {
        match site.as_str() {
            "spotify" => {
                return spotify::new_playlist(&link).await;

            },
            _ => {

            }
        }
    }
    
    match site.as_str() {
        "spotify" => {
            return spotify::new_song(&link, data_dir).await;
        },
        _ => {
            // default if nothing is passed
            if site.is_empty() {
                return spotify::new_song(&link, data_dir).await;
            } else {
                panic!("Failed to match `site` to any valid options")
            }
        }
    }
}

/*
async fn new_playlist(name: String) -> String {
    let (cfg, database): (Config, database) = ccore::load().await;
    

    String::new()
} */