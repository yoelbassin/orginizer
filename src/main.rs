use clap::Parser;
mod cli;

fn main() {
    let cli = cli::args::Cli::parse();
    cli::runner::run_organizer(&cli);
}
