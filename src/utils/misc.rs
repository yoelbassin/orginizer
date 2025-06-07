use globset::{Glob, GlobSet};

pub fn make_globset<I, S>(patterns: I) -> GlobSet
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut builder = globset::GlobSetBuilder::new();
    for pat in patterns {
        builder.add(Glob::new(pat.as_ref()).unwrap());
    }
    builder.build().unwrap()
}

pub fn path_matches_any_glob<P: AsRef<std::path::Path>>(path: P, globset: &GlobSet) -> bool {
    globset.is_match(path.as_ref())
}
