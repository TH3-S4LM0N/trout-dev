use {
    crate::{
        core as ccore,
        core::{
            structs::{database, Config},
            logger::log
        },
    },
    rodio::{source::Source, Decoder, OutputStream},
    std::{fs::File, io::BufReader, thread::sleep, path::PathBuf},
    tokio::fs as tfs,
    regex::Regex
};

pub async fn pre_play(to_play: String, playlist: bool, regex: bool, data_dir: &PathBuf) -> String {
    let dbase = ccore::load_dbase(data_dir).await;
    // check if were playing a playlist an redirect
    if playlist {
        play_playlist(&to_play, data_dir, &dbase, regex).await
    } else {
        play_song(&to_play, data_dir, &dbase, regex).await
    }
    String::new()
}
/*
    let mut is_match: bool = false;
    let mut play: Vec<String> = vec![String::new(), String::new()];


    // check if search term matches any songs
    for item in &dbase.songs {
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
    crate::subcommands::play::play_song(song_to_play, cfg).await;


    loop {
        let next_dbase = &dbase.songs[thread_rng().gen_range(
            0..*&dbase.songs.len() as i32
        ) as usize];
        let next: Song = serde_json::from_str(
            &tfs::read_to_string(&format!(
                "{}/songs/{}.json", &cfg.data_dir.display(), &next_dbase.id
            )).await.expect("Failed to read song.json")
        ).expect("Failed to read into JSON!");
        self::play_song(next, cfg).await;
    }
}
*/
async fn play_song(to_play: &str, data_dir: &PathBuf, dbase: &database, regex: bool) {
    // check if search term matches any songs
    let mut is_match: bool = false;
    let mut play: Vec<&str> = vec!["", ""];
    if regex {
        let re = Regex::new(to_play).unwrap();
        for item in &dbase.songs {
            if re.is_match(&item.name) {
                is_match = true;
                play[0] = &item.id;
                play[1] = &item.name;
                break;
            }
        }
        if !is_match {
            log("Error: Search did not match any song! Consider using regex (--regex)", data_dir).await;
        }
    } else {
        for item in &dbase.songs {
            if item.name.contains(&to_play) {
                is_match = true;
                play[0] = &item.id;
                play[1] = &item.name;
                break;
            }
        }
    }
    

    /*
    let (_, stream_handle) = OutputStream::try_default().expect("Failed to get default audio I/O stream!");
    let file = BufReader::new(File::open(&to_play.Path).expect("Failed to open FLAC!"));
    let source = Decoder::new(file).expect("Failed to create decoder!");
    stream_handle.play_raw(source.convert_samples()).expect("Failed to play stream!");
    // wait for length of file while seperate thread plays
    sleep(lengthof_flac(&to_play.Path, cfg).await); */
}

async fn play_playlist(to_play: &str, data_dir: &PathBuf, dbase: &database, regex: bool) {}
