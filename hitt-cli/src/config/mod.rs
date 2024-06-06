use clap::{Args, Parser, Subcommand};

pub mod variables;

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

    Completions(CompletionsCommandArguments),
}

/// Send request
#[derive(Args, Debug)]
pub struct RunCommandArguments {
    /// Path to .http file, or directory if supplied with the `--recursive` argument
    #[arg()]
    pub path: std::path::PathBuf,

    /// Request timeout in milliseconds
    #[arg(long, value_name = "TIMEOUT_MS")]
    pub timeout: Option<u64>,

    /// Variables to pass to request
    #[arg(long, value_name = "KEY>=<VALUE")]
    pub var: Option<Vec<String>>,

    /// Enable to run directory recursively
    #[arg(long, short, default_value_t = false)]
    pub recursive: bool,

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

    #[arg(long, default_value_t = false, hide = true)]
    pub vim: bool,
}

/// Generate shell completions
#[derive(Args, Debug)]
pub struct CompletionsCommandArguments {
    pub shell: clap_complete::Shell,
}
