use crate::terminal::{STYLE_BOLD, STYLE_RESET, TEXT_GREEN, TEXT_RED, TEXT_RESET};

#[inline]
pub fn print_status(
    term: &console::Term,
    http_version: http::version::Version,
    method: &str,
    url: &str,
    status_code: u16,
    duration: &core::time::Duration,
) -> Result<(), std::io::Error> {
    let text_color = if status_code < 400 {
        TEXT_GREEN
    } else {
        TEXT_RED
    };

    let line = format!("{STYLE_BOLD}{text_color}{http_version:?} {method} {url} {status_code} {}ms{TEXT_RESET}{STYLE_RESET}", duration.as_millis());

    term.write_line(&line)
}
