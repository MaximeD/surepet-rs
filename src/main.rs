use clap::{Parser, Subcommand};

use crate::api::devices::devices;
use crate::api::pets::pets;
use crate::entities::shared::Information;

mod api;
mod entities;
mod utils;

#[derive(Debug, Parser)]
#[command(name = "surepet")]
#[command(about = "Surepet unofficial CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List devices
    Devices {},
    /// List pets
    Pets {},
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Devices {} => {
            for device in devices().await {
                println!("{}", device.information());
            }
        }
        Commands::Pets {} => {
            for pet in pets().await {
                println!("{}", pet.information());
            }
        }
    }
}
