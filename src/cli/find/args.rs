use clap::Args;

#[derive(Args, Debug)]
pub struct FindArgs {
    /// Filters to match by, e.g. NAME="hello.txt",SIZE=12345
    #[arg(long)]
    pub by: String,

    /// Actions to perform: VERBOSE, DELETE, COPY=/path (default: VERBOSE)
    #[arg(long, default_value = "VERBOSE")]
    pub action: String,

    /// Scan directories recursively
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,

    /// Target directories/files
    #[arg(required = true)]
    pub targets: Vec<String>,
}
