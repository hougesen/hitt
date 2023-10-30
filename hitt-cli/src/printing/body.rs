use hitt_formatter::ContentType;

use crate::printing::{TEXT_RESET, TEXT_YELLOW};

#[inline]
fn __print_body(body: &str) {
    println!("\n{TEXT_YELLOW}{body}{TEXT_RESET}")
}

#[inline]
pub(crate) fn print_body(body: &str, content_type: Option<&str>, disable_pretty_printing: bool) {
    if disable_pretty_printing {
        __print_body(body);
        return;
    }

    match content_type {
        Some(content_type) => match ContentType::from(content_type) {
            ContentType::Unknown => __print_body(body),
            i => {
                if let Some(formatted) = hitt_formatter::format(body, i) {
                    __print_body(&formatted)
                } else {
                    __print_body(body)
                }
            }
        },
        None => __print_body(body),
    }
}
