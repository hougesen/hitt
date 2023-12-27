use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Run(RunCommandArguments),
    New(NewCommandArguments),
}

/// Send request
#[derive(Args, Debug)]
pub(crate) struct RunCommandArguments {
    /// Path to .http file, or directory if supplied with the `--recursive` argument
    #[arg()]
    pub(crate) path: std::path::PathBuf,

    /// Exit on error response status code
    #[arg(long, default_value_t = false)]
    pub(crate) fail_fast: bool,

    /// Whether or not to show response body
    #[arg(long, default_value_t = false)]
    pub(crate) hide_body: bool,

    /// Whether or not to show response headers
    #[arg(long, default_value_t = false)]
    pub(crate) hide_headers: bool,

    /// Disable pretty printing of response body
    #[arg(long, default_value_t = false)]
    pub(crate) disable_formatting: bool,

    /// Enable to run directory recursively
    #[arg(long, short, default_value_t = false)]
    pub(crate) recursive: bool,

    /// Request timeout in milliseconds
    #[arg(long)]
    pub(crate) timeout: Option<u64>,

    #[arg(long, default_value_t = false, hide = true)]
    pub(crate) vim: bool,
}

/// Create new http request
#[derive(Args, Debug)]
pub(crate) struct NewCommandArguments {
    #[arg()]
    pub(crate) path: std::path::PathBuf,
}
