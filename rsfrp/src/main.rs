use clap::Parser;
use rsfrp::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.subcmd {
        Some(Commands::Server { conf }) => {
            println!("Server: {:?}", conf);
        }
        Some(Commands::Client { conf }) => {
            println!("Client: {:?}", conf);
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
