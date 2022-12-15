use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "Trout",
    author = "TH3 S4LM0N",
    version = env!("CARGO_PKG_VERSION"),
    about = "egui alternative Spotify client in Rust"
)]
struct args {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[clap(alias = "p")]
    Play {
        
    }
}