use crate::printing::{TEXT_RESET, TEXT_YELLOW};

use super::print_error;

#[inline]
pub(crate) fn print_headers(headers: &reqwest::header::HeaderMap) {
    for (key, value) in headers {
        if let Ok(value) = value.to_str() {
            println!("{TEXT_YELLOW}{key}{TEXT_RESET}: {value}{TEXT_RESET}");
        } else {
            print_error(format!("error printing value for header: {key}"));
        }
    }
}
