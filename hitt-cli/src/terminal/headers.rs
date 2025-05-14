use crossterm::{
    queue,
    style::{Print, Stylize},
};

#[inline]
pub fn print_headers<W: std::io::Write + Send>(
    term: &mut W,
    headers: &reqwest::header::HeaderMap,
) -> std::io::Result<()> {
    for (key, value) in headers {
        let value_str = String::from_utf8_lossy(value.as_ref());

        let line = format!("{}: {value_str}\n", key.to_string().dark_yellow());

        queue!(term, Print(line))?;
    }

    Ok(())
}

#[cfg(test)]
mod test_print_headers {
    use std::io::Write;

    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    use super::print_headers;

    #[test]
    fn it_should_print_without_errors() {
        let mut term = Vec::new();

        let mut headers = HeaderMap::new();

        let n = "mads";
        let v = "hougesen";

        headers.insert(HeaderName::from_static(n), HeaderValue::from_static(v));

        print_headers(&mut term, &headers).expect("it to not error");

        term.flush().expect("it to flush");

        assert_eq!(
            format!("\x1B[38;5;3m{n}\x1B[39m: {v}\n"),
            String::from_utf8_lossy(&term)
        );

        term.clear();
    }
}
