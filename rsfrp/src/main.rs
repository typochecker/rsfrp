use clap::{Error, Parser};
use rsfrp::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.subcmd {
        Some(Commands::Server { conf }) => {
            println!("Server: {:?}", conf);
            run_server().await.unwrap();
        }
        Some(Commands::Client { conf }) => {
            println!("Client: {:?}", conf);
            run_client().await.unwrap();
        }
        None => {
            println!("No subcommand was used");
        }
    }
}

async fn run_server() -> Result<(), Error> {
    todo!()
}
async fn run_client() -> Result<(), Error> {
    todo!()
}
