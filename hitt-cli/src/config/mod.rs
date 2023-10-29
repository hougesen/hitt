#[derive(clap::Parser, Debug)]
#[clap(
    author = "Mads Hougesen, mhouge.dk",
    version,
    about = "hitt is a command line HTTP testing tool focused on speed and simplicity."
)]
pub(crate) struct CliArguments {
    /// Path to .http file, or directory if supplied with the `--recursive` argument
    #[arg()]
    pub(crate) path: String,

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
}
