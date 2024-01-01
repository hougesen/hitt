#[inline]
pub fn print_status(
    term: &console::Term,
    http_version: http::version::Version,
    method: &str,
    url: &str,
    status_code: u16,
    duration: &core::time::Duration,
) -> Result<(), std::io::Error> {
    let text_color = if status_code < 400 {
        console::Style::new().green()
    } else {
        console::Style::new().red()
    };

    let line = text_color.bold().apply_to(format!(
        "{http_version:?} {method} {url} {status_code} {}ms",
        duration.as_millis()
    ));

    term.write_line(&line.to_string())
}

#[cfg(test)]
mod test_print_status {
    use super::print_status;

    #[test]
    fn should_print_green_if_success_code() {
        let term = console::Term::stdout();

        let url = "https://mhouge.dk";
        let http_version = http::version::Version::HTTP_11;
        let duration = core::time::Duration::from_millis(123);
        let method = "GET";

        for i in 0..=399 {
            // TODO: actually validate what is written to stdout
            print_status(&term, http_version, method, url, i, &duration)
                .expect("it to not raise an error");
        }
    }

    #[test]
    fn should_print_red_if_error_code() {
        let term = console::Term::stdout();

        let url = "https://mhouge.dk";
        let http_version = http::version::Version::HTTP_11;
        let duration = core::time::Duration::from_millis(123);
        let method = "GET";

        for i in 400..600 {
            // TODO: actually validate what is written to stdout
            print_status(&term, http_version, method, url, i, &duration)
                .expect("it to not raise an error");
        }
    }
}
