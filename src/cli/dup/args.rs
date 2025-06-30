use clap::Args;

#[derive(Args, Debug)]
pub struct DupArgs {
    /// Filters to match by (default: NAME,SIZE,IMAGE_CONTENT,SKIP_SELF)
    #[arg(long, default_value = "NAME,SIZE,IMAGE_CONTENT,SKIP_SELF")]
    pub by: String,

    /// Actions to perform: VERBOSE, DELETE, COPY=/path (default: VERBOSE)
    #[arg(long, default_value = "VERBOSE")]
    pub action: String,

    /// Mode of action operation: DUPLICATES, REFERENCE, UNIQUE_REFERENCE (default: DUPLICATES)
    #[arg(long, default_value = "DUPLICATES")]
    pub mode: String,

    /// Scan directories recursively
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,

    /// Target directories/files
    #[arg(required = true)]
    pub targets: Vec<String>,

    /// Reference directory/file
    #[arg(required = true)]
    pub reference: String,
}
