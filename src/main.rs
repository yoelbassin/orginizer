mod actions;
mod cli;
mod filters;
mod organizer;
mod utils;
use clap::Parser;
use std::path::PathBuf;

fn main() {
    utils::cache::init_cache(PathBuf::from(".cache"));
    let cli = cli::args::Cli::parse();
    cli::runner::run_organizer(&cli);
}
