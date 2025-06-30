mod actions;
mod cli;
mod filters;
mod organizer;
mod utils;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Dup(cli::dup::args::DupArgs),
    Find(cli::find::args::FindArgs),
}

fn main() {
    utils::cache::init_cache(PathBuf::from(".cache"));
    let cli = Cli::parse();
    match cli.command {
        Commands::Dup(args) => cli::dup::runner::run_dup(&args),
        Commands::Find(args) => cli::find::runner::run_find(&args),
    }
}
