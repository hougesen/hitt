use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run(RunCommandArguments),
    New(NewCommandArguments),
}

/// Send request
#[derive(Args, Debug)]
pub struct RunCommandArguments {
    /// Path to .http file, or directory if supplied with the `--recursive` argument
    #[arg()]
    pub path: std::path::PathBuf,

    /// Exit on error response status code
    #[arg(long, default_value_t = false)]
    pub fail_fast: bool,

    /// Whether or not to show response body
    #[arg(long, default_value_t = false)]
    pub hide_body: bool,

    /// Whether or not to show response headers
    #[arg(long, default_value_t = false)]
    pub hide_headers: bool,

    /// Disable pretty printing of response body
    #[arg(long, default_value_t = false)]
    pub disable_formatting: bool,

    /// Enable to run directory recursively
    #[arg(long, short, default_value_t = false)]
    pub recursive: bool,

    /// Request timeout in milliseconds
    #[arg(long)]
    pub timeout: Option<u64>,

    #[arg(long, default_value_t = false, hide = true)]
    pub vim: bool,
}

/// Create new http request
#[derive(Args, Debug)]
pub struct NewCommandArguments {
    #[arg()]
    pub path: std::path::PathBuf,
}
