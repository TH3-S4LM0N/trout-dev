use {
    std::path::PathBuf,
    tokio::time::Duration,
    crate::core::{ResultExt, OptionExt, structs::Config},
    metaflac::Tag,
};

// obtain precise lenght of flac so we know how
// long to wait for the audio thead
// shamelessly ripped from https://framagit.org/Bromind/playlist-duration/-/blob/master/src/main.rs
pub async fn lengthof_flac(path: &PathBuf, cfg: &Config) -> Duration {
    let tag = Tag::read_from_path(path).log("Failed to get tag", cfg);
    let stream_info = tag.get_streaminfo().log("Failed to get streaminfo", cfg);
    let nb_sec = stream_info.total_samples / stream_info.sample_rate as u64;
    return Duration::from_secs(nb_sec);
}
