use {
    crate::core::{structs::{database, Config, Song}, audio_file::lengthof_flac},
    rodio::{source::Source, Decoder, OutputStream},
    rand::{thread_rng, Rng},
    std::{
        fs::File,
        io::BufReader,
        thread::sleep
    },
    tokio::{fs as tfs},
};

pub async fn pre_play(to_play: String) -> String {
    let (cfg, database): (Config, database) = crate::core::load().await;    

    let mut is_match: bool = false;
    let mut play: Vec<String> = vec![String::new(), String::new()];
    
    // check if search term matches any songs
    for item in &database.songs {
        if item.name.contains(&to_play) {
            is_match = true;
            play[0] = item.id.to_string();
            play[1] = item.name.to_string();
            break;
        } else {
            continue;
        }
    }
    if !is_match {
        return format!("Could not find {} in any song name!", &to_play);
    }
    // if we get here we know there is a song we will play
    let song_to_play: Song = serde_json::from_str(
        &tfs::read_to_string(&format!(
            "{}/songs/{}.json",
            &cfg.data_dir.display(),
            &play[0]
        ))
        .await
        .expect("Failed to read song.json!"),
    )
    .expect("Failed to read into JSON!");

    // init play of passed song
    crate::subcommands::play::play(song_to_play).await;


    loop {
        let next_dbase = &database.songs[thread_rng().gen_range(
            0..*&database.songs.len() as i32
        ) as usize];
        let next: Song = serde_json::from_str(
            &tfs::read_to_string(&format!(
                "{}/songs/{}.json", &cfg.data_dir.display(), &next_dbase.id
            )).await.expect("Failed to read song.json")
        ).expect("Failed to read into JSON!");
        crate::subcommands::play::play(next).await;
    }
}

async fn play(to_play: Song) {
    let (_, stream_handle) = OutputStream::try_default().expect("Failed to get default audio I/O stream!");
    let file = BufReader::new(File::open(&to_play.Path).expect("Failed to open FLAC!"));
    let source = Decoder::new(file).expect("Failed to create decoder!");
    stream_handle.play_raw(source.convert_samples()).expect("Failed to play stream!");
    // wait for length of file while seperate thread plays
    sleep(lengthof_flac(&to_play.Path).await);
}
