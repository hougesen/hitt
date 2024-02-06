use crossterm::{
    queue,
    style::{Print, Stylize},
};

#[inline]
pub fn print_status<W: std::io::Write>(
    term: &mut W,
    http_version: http::version::Version,
    method: &str,
    url: &str,
    status_code: u16,
    duration: &core::time::Duration,
) -> Result<(), std::io::Error> {
    let line = format!(
        "{http_version:?} {method} {url} {status_code} {}ms\n",
        duration.as_millis()
    )
    .bold();

    queue!(
        term,
        Print(if status_code < 400 {
            line.green()
        } else {
            line.red()
        }),
    )
}

#[cfg(test)]
mod test_print_status {
    use std::io::Write;

    use super::print_status;

    #[test]
    fn should_print_green_if_success_code() {
        let mut term = Vec::new();

        let url = "https://mhouge.dk";
        let http_version = http::version::Version::HTTP_11;
        let duration = core::time::Duration::from_millis(123);
        let method = "GET";

        for status_code in 0..=399 {
            print_status(&mut term, http_version, method, url, status_code, &duration)
                .expect("it to not raise an error");

            term.flush().expect("it to flush");

            assert_eq!(
                format!(
                    "\x1b[38;5;10m\x1B[1mHTTP/1.1 {method} {url} {status_code} {}ms\n\x1B[0m",
                    duration.as_millis()
                ),
                String::from_utf8_lossy(&term)
            );

            term.clear();
        }
    }

    #[test]
    fn should_print_red_if_error_code() {
        let mut term = Vec::new();

        let url = "https://mhouge.dk";
        let http_version = http::version::Version::HTTP_11;
        let duration = core::time::Duration::from_millis(123);
        let method = "GET";

        for status_code in 400..600 {
            print_status(&mut term, http_version, method, url, status_code, &duration)
                .expect("it to not raise an error");

            term.flush().expect("it to flush");

            assert_eq!(
                format!(
                    "\x1b[38;5;9m\x1B[1mHTTP/1.1 {method} {url} {status_code} {}ms\n\x1B[0m",
                    duration.as_millis()
                ),
                String::from_utf8_lossy(&term)
            );

            term.clear();
        }
    }
}
