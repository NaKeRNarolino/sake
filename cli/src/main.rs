use clap::{Parser, Subcommand};
use crate::init::init;

mod init;

#[derive(Parser, Debug)]
#[command(
name = "sakecli"
)]
pub struct CLI {
    #[command(subcommand)]
    pub subcommand: Command
}

#[derive(Subcommand, Debug)]
#[derive(Clone)]
pub enum Command {
    Init
}

#[tokio::main]
async fn main() {
    colog::init();

    let cli = CLI::parse();

    match cli.subcommand {
        Command::Init => {
            log::info!("Generating Sake project in this directory.");
            init().await;
        }
    }
}
