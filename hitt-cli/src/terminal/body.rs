use hitt_formatter::ContentType;

use crate::terminal::{TEXT_RESET, TEXT_YELLOW};

#[inline]
fn __print_body(term: &console::Term, body: &str) -> Result<(), std::io::Error> {
    term.write_line(&format!("\n{TEXT_YELLOW}{body}{TEXT_RESET}"))
}

#[inline]
pub fn print_body(
    term: &console::Term,
    body: &str,
    content_type: ContentType,
    disable_pretty_printing: bool,
) -> Result<(), std::io::Error> {
    if disable_pretty_printing {
        return __print_body(term, body);
    }

    if let Some(formatted) = hitt_formatter::format(body, content_type) {
        return __print_body(term, &formatted);
    }

    __print_body(term, body)
}
