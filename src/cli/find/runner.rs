use crate::cli::find::utils::parse_filters;
use crate::cli::parsers::{parse_actions, parse_targets};
use crate::organizer::actions::actions_pipeline;
use crate::organizer::finder::finder;

pub fn run_find(args: &super::args::FindArgs) {
    let filters = parse_filters(&args.by);
    let actions = parse_actions(&args.action);

    let targets = parse_targets(&args.targets);

    for target in targets {
        let found_itr = finder(&target, args.recursive, &filters);
        for file in found_itr {
            actions_pipeline(&file, &actions);
        }
    }
}
