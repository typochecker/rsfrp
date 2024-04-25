/*!
command line parse module
*/
#![allow(unused)]

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
#[command(about = "rsfrp is a reverse proxy tool")]
pub struct Cli {
    #[command(subcommand)]
    pub subcmd: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// run rsfrp as server
    Server {
        /// server config path
        #[arg(short, long)]
        conf: Option<PathBuf>,
    },

    /// run rsfrp as server
    Client {
        /// client config path
        #[arg(short, long)]
        conf: Option<PathBuf>,
    },
}
