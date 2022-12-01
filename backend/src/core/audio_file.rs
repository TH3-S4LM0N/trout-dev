use {
    std::path::PathBuf,
    tokio::time::Duration,
    crate::core::{logger::{ResultExt, OptionExt}},
    metaflac::Tag,
};

// obtain precise lenght of flac so we know how
// long to wait for the audio thead
// shamelessly ripped from https://framagit.org/Bromind/playlist-duration/-/blob/master/src/main.rs
pub async fn lengthof_flac(path: &PathBuf, data_dir: &PathBuf) -> Duration {
    let tag = Tag::read_from_path(path).log("Failed to get tag", data_dir);
    let stream_info = tag.get_streaminfo().log("Failed to get streaminfo", data_dir);
    let nb_sec = stream_info.total_samples / stream_info.sample_rate as u64;
    return Duration::from_secs(nb_sec);
}
