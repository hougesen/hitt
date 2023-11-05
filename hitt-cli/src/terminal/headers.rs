use crate::terminal::{TEXT_RED, TEXT_RESET, TEXT_YELLOW};

#[inline]
pub(crate) fn print_headers(headers: &reqwest::header::HeaderMap) {
    for (key, value) in headers {
        if let Ok(value) = value.to_str() {
            println!("{TEXT_YELLOW}{key}{TEXT_RESET}: {value}{TEXT_RESET}");
        } else {
            eprintln!("{TEXT_RED}hitt: error printing value for header - {key}{TEXT_RESET}")
        }
    }
}
