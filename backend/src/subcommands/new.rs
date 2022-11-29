//use crate::{sites, core::structs::{Config,database}};

use {
    crate::sites::spotify,
};

pub async fn new(playlist: String, site: String, link: String) -> String {
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
            return spotify::new_song(&link).await;
        },
        _ => {
            // default if nothing is passed
            if site.is_empty() {
                return spotify::new_song(&link).await;
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