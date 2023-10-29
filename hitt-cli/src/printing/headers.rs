#[inline]
pub(crate) fn print_headers(headers: &reqwest::header::HeaderMap) {
    for (key, value) in headers {
        if let Ok(value) = value.to_str() {
            println!("{key}: {value}",);
        } else {
            eprintln!("Error printing value for header: {key}");
        }
    }
}
