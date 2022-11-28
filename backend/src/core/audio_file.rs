use std::path::PathBuf;

use {metaflac::Tag, tokio::time::Duration};

// obtain precise lenght of flac so we know how
// long to wait for the audio thead
// shamelessly ripped from https://framagit.org/Bromind/playlist-duration/-/blob/master/src/main.rs
pub async fn lengthof_flac(path: &PathBuf) -> Duration {
    let tag = match Tag::read_from_path(path) {
        Ok(t) => t,
        Err(_e) => {
            panic!("Unable to open {}", path.display());
        }
    };
    let stream_info = tag.get_streaminfo().expect("Failed to get streaminfo");
    let nb_sec = stream_info.total_samples / stream_info.sample_rate as u64;
    return Duration::from_secs(nb_sec);
}
