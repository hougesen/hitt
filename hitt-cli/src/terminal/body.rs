use hitt_formatter::ContentType;

#[inline]
fn __print_body(term: &console::Term, body: &str) -> Result<(), std::io::Error> {
    term.write_line(&format!("\n{}", console::style(body).yellow()))
}

#[cfg(test)]
mod __test_print_body {
    use super::__print_body;

    #[test]
    fn it_should_print_without_errors() {
        let term = console::Term::stdout();

        // TODO: validate what is written to stdout
        __print_body(&term, "body").expect("it not to return an error");
    }
}

#[inline]
pub fn print_body(
    term: &console::Term,
    body: &str,
    content_type: ContentType,
    disable_pretty_printing: bool,
) -> Result<(), std::io::Error> {
    if disable_pretty_printing {
        return __print_body(term, body);
    }

    if let Some(formatted) = hitt_formatter::format(body, content_type) {
        return __print_body(term, &formatted);
    }

    __print_body(term, body)
}

#[cfg(test)]
mod test_print_body {
    use hitt_formatter::ContentType;

    use super::print_body;

    #[test]
    fn it_should_print_without_errors() {
        let term = console::Term::stdout();

        print_body(&term, "body", ContentType::Unknown, false).expect("it not to return an error");

        print_body(&term, "body", ContentType::Unknown, true).expect("it not to return an error");
    }

    #[test]
    fn it_should_format_json() {
        let term = console::Term::stdout();

        let input = "{\"key\":\"value\"}";

        // TODO: test stdout is formatted
        print_body(&term, input, ContentType::Json, false).expect("it not to return an error");

        // TODO: test stdout is not formatted
        print_body(&term, input, ContentType::Json, true).expect("it not to return an error");
    }

    #[test]
    fn it_should_format_json_if_pretty_printing_disable() {
        let term = console::Term::stdout();

        let input = "{\"key\":\"value\"}";

        // TODO: test stdout is not formatted
        print_body(&term, input, ContentType::Json, true).expect("it not to return an error");
    }
}
