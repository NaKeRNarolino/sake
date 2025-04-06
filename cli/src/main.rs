use clap::{Parser, Subcommand};
use crate::init::init;

mod init;

#[derive(Parser, Debug)]
pub struct CLI {
    pub subcommand: Command
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init
}

fn main() {
    colog::init();

    let cli = CLI::parse();

    match cli.subcommand {
        Command::Init => {
            log::info!("Generating Sake project in this directory.");
            init()
        }
    }
}
