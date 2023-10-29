use crate::printing::{TEXT_RESET, TEXT_YELLOW};

#[inline]
fn format_json(input: &str) -> String {
    jsonformat::format(input, jsonformat::Indentation::TwoSpace)
}

#[inline]
fn __print_body(body: &str) {
    println!("{TEXT_YELLOW}{body}{TEXT_RESET}")
}

#[inline]
pub(crate) fn print_body(body: &str, content_type: Option<&str>, disable_pretty_printing: bool) {
    if disable_pretty_printing {
        __print_body(body);
        return;
    }

    match content_type {
        Some(content_type) => {
            if content_type.starts_with("application/json") {
                __print_body(&format_json(body));
            } else {
                __print_body(body);
            }
        }
        None => __print_body(body),
    }
}
