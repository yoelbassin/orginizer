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
