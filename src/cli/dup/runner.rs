use crate::cli::dup::utils::{parse_filters, parse_mode, perform_action_by_mode};
use crate::cli::parsers::{parse_actions, parse_targets};

pub fn run_dup(args: &super::args::DupArgs) {
    let filters = parse_filters(&args.by);
    let actions = parse_actions(&args.action, None);

    let reference = std::path::PathBuf::from(&args.reference);
    let targets = parse_targets(&args.targets);

    let mode = parse_mode(&args.mode);

    for target in targets {
        let duplicates_iter = crate::organizer::finder::duplicates_finder(
            &target,
            &reference,
            args.recursive,
            &filters,
        );
        for (reference_file, duplicates) in duplicates_iter {
            perform_action_by_mode(&mode, &reference_file, &duplicates, &actions);
        }
    }
}
