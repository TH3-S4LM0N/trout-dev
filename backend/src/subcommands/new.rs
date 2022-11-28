use crate::{sites, core::structs::{Config,database}};

pub async fn new(playlist: String, site: String, link: String) -> String {
    if playlist.is_empty() {} else {
        match site.as_str() {
            "spotify" => {
                return sites::spotify::new_playlist(&link).await;

            },
            _ => {

            }
        }
    }
    
    match site.as_str() {
        "spotify" => {
            return sites::spotify::new_song(&link).await;
        },
        _ => {
            // default if nothing is passed
            if site.is_empty() {
                return sites::spotify::new_song(&link).await;
            } else {
                panic!("Failed to match `site` to any valid options")
            }
        }
    }
}


async fn new_playlist(_name: String) -> String {
    let (cfg, database): (Config, database) = crate::core::load().await;
    

    String::new()
}