use clap::{ArgAction, Parser};
use organizer::organizer::find_all_duplicates_in_folder;
use organizer::utils::misc::make_globset;
use organizer::{
    actions::{Action, copy::CopyAction, delete::DeleteAction, verbose::VerboseAction},
    filters::FilterKindType,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Comma separated ordered list of filters to match by (NAME,SIZE,DATE_MODIFIED,DATE_CREATED,IMAGE_CONTENT,SKIP_SELF)
    #[arg(long = "by", default_value = "NAME,SIZE,IMAGE_CONTENT,SKIP_SELF")]
    by: String,

    /// Comma separated ordered list of actions to perform (VERBOSE,DELETE,COPY=/path)
    #[arg(long = "action", default_value = "VERBOSE")]
    action: String,

    /// Enable verbose output (same as adding VERBOSE action)
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,

    /// Dry run (same as only having the VERBOSE action)
    #[arg(long, action = ArgAction::SetTrue)]
    dry_run: bool,

    /// Check directories recursively
    #[arg(short, long, action = ArgAction::SetTrue)]
    recursive: bool,

    /// Add skip-self filter
    #[arg(long, action = ArgAction::SetTrue)]
    skip_self: bool,

    /// Comma separated list of glob patterns to exclude
    #[arg(long = "exclude", default_value = "")]
    exclude: String,

    /// Target folders (one or more)
    #[arg(required = true, num_args = 1..)]
    targets: Vec<PathBuf>,

    /// Reference folder (last positional argument)
    #[arg(required = true)]
    reference: PathBuf,
}

fn parse_filters(by: &str, skip_self: bool) -> Vec<FilterKindType> {
    let mut filters = Vec::new();
    for part in by.split(',') {
        match part.trim().to_uppercase().as_str() {
            "NAME" => filters.push(FilterKindType::FileName),
            "SIZE" => filters.push(FilterKindType::FileSize),
            "DATE_MODIFIED" => filters.push(FilterKindType::DateModified),
            "DATE_CREATED" => filters.push(FilterKindType::DateCreated),
            "IMAGE_CONTENT" => filters.push(FilterKindType::ImageContent),
            "SKIP_SELF" => filters.push(FilterKindType::SkipSelf),
            _ => {} // ignore unknown
        }
    }
    if skip_self && !filters.contains(&FilterKindType::SkipSelf) {
        filters.push(FilterKindType::SkipSelf);
    }
    filters
}

fn parse_actions(action: &str, verbose: bool, dry_run: bool) -> Vec<Box<dyn Action>> {
    let mut actions: Vec<Box<dyn Action>> = Vec::new();
    if dry_run {
        actions.push(Box::new(VerboseAction {}));
        return actions;
    }
    let mut has_verbose = false;
    for part in action.split(',') {
        let part = part.trim();
        if part.eq_ignore_ascii_case("VERBOSE") {
            actions.push(Box::new(VerboseAction {}));
            has_verbose = true;
        } else if part.eq_ignore_ascii_case("DELETE") {
            actions.push(Box::new(DeleteAction {}));
        } else if let Some(rest) = part.strip_prefix("COPY=") {
            actions.push(Box::new(CopyAction {
                destination: PathBuf::from(rest),
            }));
        }
    }
    if verbose && !has_verbose {
        actions.push(Box::new(VerboseAction {}));
    }
    if actions.is_empty() {
        actions.push(Box::new(VerboseAction {}));
    }
    actions
}

fn build_exclude_patterns(exclude: &str, targets: &[PathBuf]) -> Vec<String> {
    if exclude.trim().is_empty() {
        return Vec::new();
    }
    let mut patterns = Vec::new();
    for pat in exclude
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        if !pat.contains('*') && !pat.contains('/') {
            for target in targets {
                let prefix = if target.to_string_lossy() == "." {
                    ".".to_string()
                } else {
                    target.to_string_lossy().to_string()
                };
                patterns.push(format!("{}/{}{}", prefix, pat, "/**"));
            }
        } else {
            patterns.push(pat.to_string());
        }
    }
    patterns
}

fn run_organizer(cli: &Cli) {
    if cli.targets.is_empty() {
        eprintln!("At least one target folder must be specified.");
        std::process::exit(1);
    }
    let filters = parse_filters(&cli.by, cli.skip_self);
    let actions = parse_actions(&cli.action, cli.verbose, cli.dry_run);
    let exclude_patterns = build_exclude_patterns(&cli.exclude, &cli.targets);
    let globset = make_globset(exclude_patterns);
    let recursive = Some(cli.recursive);
    for target in &cli.targets {
        find_all_duplicates_in_folder(
            &cli.reference,
            target,
            &filters,
            &actions,
            &globset,
            recursive,
        );
    }
}

fn main() {
    let cli = Cli::parse();
    run_organizer(&cli);
}
