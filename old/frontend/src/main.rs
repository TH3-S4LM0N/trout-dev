// Rust version of trout-frontend

use core::panic;
use std::env;

use clap::{Parser, Subcommand};
use zbus::{dbus_proxy, Connection, Result};

//
// CLAP
//
#[derive(Parser)]
#[clap(name = "Trout")]
#[clap(author = "TH3-S4LM0N")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "CLI music player downloading from Spotify")]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Play a song or playlist. Alias "p"
    #[clap(alias = "p")]
    Play {
        /// Song to play
        #[clap(value_name = "SONG")]
        #[clap(help = "Song to play")]
        song: String,

        /// Play a playlist
        #[clap(short, long)]
        playlist: bool,

        /// Use regex to search
        #[clap(short, long)]
        regex: bool
    },
    /// Generate a directory to store songs and playlists etc.
    Gen {
        /// Dir to put data in, defaults to ~/trout
        #[clap(short, long, value_name = "DIR")]
        data_dir: Option<String>,
    },
    /// Create new songs and playlists. Alias "n"
    #[clap(alias = "n")]
    New {
        /// Make a playlist
        #[clap(short, long, value_name = "NAME")]
        playlist: Option<String>,

        /// Site to download from
        #[clap(short, long, value_name = "SITE")]
        #[clap(help = "Site to download from. Current Values: `spotify`")]
        site: Option<String>,

        /// Share link of song to download
        #[clap(value_name = "URL")]
        #[clap(help = "Share link of song to download")]
        link: String,
    },
    #[cfg(debug_assertions)]
    Dbg {

    }
}

//
// ZBUS
//
#[dbus_proxy(
    interface = "org.trout.BackendI",
    default_service = "org.trout.Backend",
    default_path = "/org/trout/Backend"
)]
trait Backend {
    async fn Play(&self, playlist: bool, regex: bool, to_play: String) -> Result<String>;
    async fn Gen(&self, data_dir: String) -> Result<String>;
    async fn New(&self, playlist: String, site: String, link: String) -> Result<String>;
    #[cfg(debug_assertions)]
    async fn dbg(&self, test: bool) -> Result<String>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::session().await?;
    let proxy = BackendProxy::new(&conn).await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Play { song, playlist, regex } => {
            let reply = proxy.Play(playlist, regex, song).await?;
            println!("{reply}");
        }
        Commands::Gen { data_dir } => {
            let reply = proxy
                .Gen({
                    if data_dir.is_some() {
                        format!("{}", data_dir.unwrap())
                    } else {
                        format!("{}/trout/", {
                            let res = env::var("HOME").expect("Failed to read $HOME env var!");
                            format!("{}", res)
                        })
                    }
                })
                .await?;
            println!("{reply}");
        }
        Commands::New {
            playlist,
            site,
            link,
        } => {
            let reply = proxy.New(
                if playlist.is_some() {
                    playlist.unwrap()
                } else if playlist.is_none() {
                    String::new()
                } else {
                    panic!("Playlist is neither `none` or `some`!")
                },
                if site.is_some() {
                    site.unwrap()
                } else if site.is_none() {
                    String::new()
                } else {
                    panic!("Site is neither `none` or `some`!")
                },
                link,
            ).await?;
            println!("{reply}");
        },
        #[cfg(debug_assertions)]
        Commands::Dbg {  } => {
            let reply = proxy.dbg(true).await?;
            println!("{reply}");
        }
    };

    Ok(())
}
