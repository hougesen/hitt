use crossterm::{
    queue,
    style::{Print, Stylize},
};
use hitt_formatter::ContentType;

#[inline]
fn __print_body<W: std::io::Write + Send>(term: &mut W, body: &str) -> std::io::Result<()> {
    queue!(term, Print('\n'), Print(body.dark_yellow()), Print("\n\n"))
}

#[cfg(test)]
mod __test_print_body {
    use std::io::Write;

    use super::__print_body;

    #[test]
    fn it_should_print_without_errors() {
        let mut term = Vec::new();

        __print_body(&mut term, "body").expect("it not to return an error");

        term.flush().expect("it to flush");

        assert_eq!(
            "\n\x1B[38;5;3mbody\x1B[39m\n\n",
            String::from_utf8_lossy(&term)
        );

        term.clear();
    }
}

#[inline]
pub fn print_body<W: std::io::Write + Send>(
    term: &mut W,
    body: &str,
    content_type: ContentType,
    disable_pretty_printing: bool,
) -> std::io::Result<()> {
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
    use std::io::Write;

    use hitt_formatter::ContentType;

    use super::print_body;

    #[test]
    fn it_should_print_without_errors() {
        let mut term = Vec::new();

        print_body(&mut term, "body", ContentType::Unknown, false)
            .expect("it not to return an error");

        term.flush().expect("it to flush");
        assert_eq!(
            "\n\x1B[38;5;3mbody\x1B[39m\n\n",
            String::from_utf8_lossy(&term)
        );
        term.clear();

        print_body(&mut term, "body", ContentType::Unknown, true)
            .expect("it not to return an error");

        term.flush().expect("it to flush");
        assert_eq!(
            "\n\x1B[38;5;3mbody\x1B[39m\n\n",
            String::from_utf8_lossy(&term)
        );
        term.clear();
    }

    #[test]
    fn it_should_format_json() {
        let mut term = Vec::new();

        let input = "{\"key\":\"value\"}";

        print_body(&mut term, input, ContentType::Json, false).expect("it not to return an error");

        term.flush().expect("it to flush");
        assert_eq!(
            "\n\x1B[38;5;3m{\n  \"key\": \"value\"\n}\x1B[39m\n\n",
            String::from_utf8_lossy(&term)
        );
        term.clear();

        print_body(&mut term, input, ContentType::Json, true).expect("it not to return an error");
        term.flush().expect("it to flush");
        assert_eq!(
            format!("\n\x1B[38;5;3m{input}\x1B[39m\n\n"),
            String::from_utf8_lossy(&term)
        );
        term.clear();
    }

    #[test]
    fn it_not_should_format_json_if_pretty_printing_disable() {
        let mut term = Vec::new();

        let input = "{\"key\":\"value\"}";

        print_body(&mut term, input, ContentType::Json, true).expect("it not to return an error");

        term.flush().expect("it to flush");
        assert_eq!(
            format!("\n\x1B[38;5;3m{input}\x1B[39m\n\n"),
            String::from_utf8_lossy(&term)
        );
        term.clear();
    }
}
