use hitt_parser::http;

use crate::terminal::{STYLE_BOLD, STYLE_RESET, TEXT_GREEN, TEXT_RED, TEXT_RESET};

#[inline]
pub(crate) fn print_status(
    http_version: &http::version::Version,
    method: &str,
    url: &str,
    status_code: u16,
    duration: &std::time::Duration,
) {
    let text_color = if status_code < 400 {
        TEXT_GREEN
    } else {
        TEXT_RED
    };

    println!("{STYLE_BOLD}{text_color}{http_version:?} {method} {url} {status_code} {}ms{TEXT_RESET}{STYLE_RESET}", duration.as_millis());
}
