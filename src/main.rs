mod actions;
mod cli;
mod filters;
mod organizer;
mod utils;
use clap::Parser;

fn main() {
    let cli = cli::args::Cli::parse();
    cli::runner::run_organizer(&cli);
}
