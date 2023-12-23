use hitt_formatter::ContentType;

use crate::terminal::{TEXT_RESET, TEXT_YELLOW};

#[inline]
fn __print_body(term: &console::Term, body: &str) -> Result<(), std::io::Error> {
    term.write_line(&format!("\n{TEXT_YELLOW}{body}{TEXT_RESET}"))
}

#[inline]
pub(crate) fn print_body(
    term: &console::Term,
    body: &str,
    content_type: Option<&str>,
    disable_pretty_printing: bool,
) -> Result<(), std::io::Error> {
    if disable_pretty_printing {
        return __print_body(term, body);
    }

    match content_type {
        Some(content_type) => match ContentType::from(content_type) {
            ContentType::Unknown => __print_body(term, body),
            i => {
                if let Some(formatted) = hitt_formatter::format(body, i) {
                    __print_body(term, &formatted)
                } else {
                    __print_body(term, body)
                }
            }
        },
        None => __print_body(term, body),
    }
}
