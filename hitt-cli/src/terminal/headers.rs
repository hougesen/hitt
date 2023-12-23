use crate::terminal::{TEXT_RED, TEXT_RESET, TEXT_YELLOW};

#[inline]
pub(crate) fn print_headers(
    term: &console::Term,
    headers: &reqwest::header::HeaderMap,
) -> Result<(), std::io::Error> {
    for (key, value) in headers {
        if let Ok(value) = value.to_str() {
            term.write_line(&format!(
                "{TEXT_YELLOW}{key}{TEXT_RESET}: {value}{TEXT_RESET}"
            ))?;
        } else {
            term.write_line(&format!(
                "{TEXT_RED}hitt: error printing value for header - {key}{TEXT_RESET}"
            ))?;
        }
    }

    Ok(())
}
