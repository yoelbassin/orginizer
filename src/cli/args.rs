use clap::{ArgAction, Parser};
use indicatif::ProgressBar;
use organizer::actions::{Action, copy::CopyAction, delete::DeleteAction, verbose::VerboseAction};
use organizer::filters::FilterKindType;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Comma separated ordered list of filters to match by (NAME,SIZE,DATE_MODIFIED,DATE_CREATED,IMAGE_CONTENT,SKIP_SELF)
    #[arg(long = "by", default_value = "NAME,SIZE,IMAGE_CONTENT,SKIP_SELF")]
    pub by: String,

    /// Comma separated ordered list of actions to perform (VERBOSE,DELETE,COPY=/path)
    #[arg(long = "action", default_value = "VERBOSE")]
    pub action: String,

    /// Enable verbose output (same as adding VERBOSE action)
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Dry run (same as only having the VERBOSE action)
    #[arg(long, action = ArgAction::SetTrue)]
    pub dry_run: bool,

    /// Check directories recursively
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub recursive: bool,

    /// Add skip-self filter
    #[arg(long, action = ArgAction::SetTrue)]
    pub skip_self: bool,

    /// Comma separated list of glob patterns to exclude
    #[arg(long = "exclude", default_value = "")]
    pub exclude: String,

    /// Target folders (one or more)
    #[arg(required = true, num_args = 1..)]
    pub targets: Vec<PathBuf>,

    /// Reference folder (last positional argument)
    #[arg(required = true)]
    pub reference: PathBuf,
}

pub fn parse_filters(by: &str, skip_self: bool) -> Vec<FilterKindType> {
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

pub fn parse_actions(
    action: &str,
    verbose: bool,
    dry_run: bool,
    progress: Option<Arc<ProgressBar>>,
) -> Vec<Box<dyn Action>> {
    let mut actions: Vec<Box<dyn Action>> = Vec::new();
    if dry_run {
        actions.push(Box::new(VerboseAction {
            progress: progress.clone(),
        }));
        return actions;
    }
    let mut has_verbose = false;
    for part in action.split(',') {
        let part = part.trim();
        if part.eq_ignore_ascii_case("VERBOSE") {
            actions.push(Box::new(VerboseAction {
                progress: progress.clone(),
            }));
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
        actions.push(Box::new(VerboseAction {
            progress: progress.clone(),
        }));
    }
    if actions.is_empty() {
        actions.push(Box::new(VerboseAction {
            progress: progress.clone(),
        }));
    }
    actions
}

pub fn build_exclude_patterns(exclude: &str, targets: &[PathBuf]) -> Vec<String> {
    if exclude.trim().is_empty() {
        return Vec::new();
    }

    // Helper to expand a bare name for each target
    let expand_for_targets = |name: &str| {
        targets
            .iter()
            .map(|target| {
                let prefix = if target.to_string_lossy() == "." {
                    ".".to_string()
                } else {
                    target.to_string_lossy().to_string()
                };
                format!("{}/{}{}", prefix, name, "/**")
            })
            .collect::<Vec<String>>()
    };

    exclude
        .split(',')
        .map(str::trim)
        .filter(|pat| !pat.is_empty())
        .flat_map(|pat| {
            if pat.contains('/') {
                // Prepend './' if not already present
                let pat = if pat.starts_with("./") || pat.starts_with('/') {
                    pat.to_string()
                } else {
                    format!("./{}", pat)
                };
                vec![pat]
            } else if !pat.contains('*') {
                expand_for_targets(pat)
            } else {
                vec![pat.to_string()]
            }
        })
        .collect()
}
