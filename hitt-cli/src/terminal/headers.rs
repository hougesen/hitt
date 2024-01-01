#[inline]
pub fn print_headers(
    term: &console::Term,
    headers: &reqwest::header::HeaderMap,
) -> Result<(), std::io::Error> {
    let header_name_style = console::Style::new().yellow();

    for (key, value) in headers {
        if let Ok(value_str) = value.to_str() {
            term.write_line(&format!("{}: {value_str}", header_name_style.apply_to(key)))?;
        } else {
            term.write_line(
                &console::style(format!(
                    "hitt: error printing value for header - {key} '{value:?}'"
                ))
                .red()
                .to_string(),
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test_print_headers {
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    use super::print_headers;

    #[test]
    fn it_should_print_without_errors() {
        let term = console::Term::stdout();

        let mut headers = HeaderMap::new();

        headers.insert(
            HeaderName::from_static("mads"),
            HeaderValue::from_static("hougesen"),
        );

        // TODO: validate what is written to stdout
        print_headers(&term, &headers).expect("it to not error");
    }
}
