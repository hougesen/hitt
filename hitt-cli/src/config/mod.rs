#[derive(clap::Parser, Debug)]
#[clap(
    author = "Mads Hougesen, mhouge.dk",
    version,
    about = "hitt is a HTTP testing tool focused on speed and simplicity."
)]
pub(crate) struct CliArguments {
    /// Path to .http file
    #[arg()]
    pub(crate) path: String,

    /// Exit on error status code
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
}
