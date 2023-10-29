use crate::printing::{STYLE_BOLD, STYLE_RESET, TEXT_GREEN, TEXT_RED, TEXT_RESET};

#[inline]
pub(crate) fn print_status(method: &str, url: &str, status_code: u16) {
    let text_color = if status_code < 400 {
        TEXT_GREEN
    } else {
        TEXT_RED
    };

    println!("{STYLE_BOLD}{text_color}{method} {url} {status_code}{TEXT_RESET}{STYLE_RESET}");
}
