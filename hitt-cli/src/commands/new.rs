use core::str::FromStr;

use console::{Key, Term};
use http::{HeaderName, HeaderValue, Uri};

use crate::{
    config::NewCommandArguments,
    error::HittCliError,
    terminal::{
        editor::editor_input,
        input::{confirm_input, select_input, text_input_prompt},
        TEXT_RED, TEXT_RESET,
    },
};

#[inline]
fn set_method(term: &Term) -> Result<String, std::io::Error> {
    let http_methods = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    select_input(term, "Which HTTP method?", &http_methods)
}

#[inline]
fn set_url(term: &Term) -> Result<String, std::io::Error> {
    text_input_prompt(
        term,
        "What should the url be?",
        |input| !input.is_empty() && Uri::from_str(input).is_ok(),
        |input| format!("{TEXT_RED}'{input}' is not a valid url{TEXT_RESET}"),
    )
}

#[inline]
fn set_headers(term: &Term) -> Result<Vec<(String, String)>, std::io::Error> {
    let mut headers = Vec::new();

    let lower_y_key = Key::Char('y');

    let mut writing_headers =
        confirm_input(term, "Do you want to add headers? (Y/n)", &lower_y_key)?;

    let key_validator = |input: &str| !input.is_empty() && HeaderName::from_str(input).is_ok();
    let format_key_error =
        |input: &str| format!("{TEXT_RED}'{input}' is not a valid header key{TEXT_RESET}");

    let value_validator = |input: &str| !input.is_empty() && HeaderValue::from_str(input).is_ok();
    let format_value_error =
        |input: &str| format!("{TEXT_RED}'{input}' is not a valid header value{TEXT_RESET}");

    while writing_headers {
        let key = text_input_prompt(
            term,
            "What should the key be?",
            key_validator,
            format_key_error,
        )?;

        let value = text_input_prompt(
            term,
            "What should the value be?",
            value_validator,
            format_value_error,
        )?;

        headers.push((key, value));

        writing_headers =
            confirm_input(term, "Do you want to add more headers? (Y/n)", &lower_y_key)?;
    }

    Ok(headers)
}

#[inline]
fn try_find_content_type(headers: &[(String, String)]) -> Option<&str> {
    for (key, value) in headers {
        if key.eq_ignore_ascii_case("content-type") {
            return Some(value);
        }
    }

    None
}

#[cfg(test)]
mod test_try_find_content_type {
    use crate::commands::new::try_find_content_type;

    #[test]
    fn it_should_return_none_if_not_exist() {
        let headers = vec![("a".to_owned(), "b".to_owned())];

        assert!(try_find_content_type(&headers).is_none());
    }

    #[test]
    fn it_should_return_type_if_exists() {
        let content_type = "application/json";

        let headers = vec![("content-type".to_owned(), content_type.to_owned())];

        assert_eq!(Some(content_type), try_find_content_type(&headers));
    }

    #[test]
    fn it_should_ignore_case() {
        {
            let content_type = "application/JSON";

            let headers = vec![("content-type".to_owned(), content_type.to_owned())];

            assert_eq!(Some(content_type), try_find_content_type(&headers));
        };

        {
            let content_type = "application/JSON".to_lowercase();

            let headers = vec![("content-type".to_owned(), content_type.clone())];

            assert_eq!(Some(content_type.as_str()), try_find_content_type(&headers));
        };

        {
            let content_type = "application/JSON".to_uppercase();

            let headers = vec![("content-type".to_owned(), content_type.clone())];

            assert_eq!(Some(content_type.as_str()), try_find_content_type(&headers));
        }
    }
}

#[inline]
fn set_body(term: &Term, content_type: Option<&str>) -> Result<Option<String>, std::io::Error> {
    if !confirm_input(term, "Do you want to add a body? (Y/n)", &Key::Char('y'))? {
        return Ok(None);
    }

    editor_input(term, content_type)
}

fn save_request(
    path: &std::path::Path,
    method: &str,
    url: &str,
    headers: &[(String, String)],
    body: Option<String>,
) -> Result<(), HittCliError> {
    let mut contents = format!("{method} {url}\n");

    if !headers.is_empty() {
        for (key, value) in headers {
            contents.push_str(key);
            contents.push_str(": ");
            contents.push_str(value);
            contents.push('\n');
        }
    }

    if let Some(content) = body {
        contents.push('\n');
        contents.push_str(&content);
        contents.push('\n');
    }

    std::fs::write(path, contents).map_err(|error| HittCliError::IoWrite(path.to_path_buf(), error))
}

#[cfg(test)]
mod test_save_request {
    use super::save_request;

    #[test]
    fn it_should_save_request_input() {
        let file = tempfile::Builder::new()
            .prefix("test-hitt-")
            .suffix(".http")
            .rand_bytes(12)
            .tempfile()
            .expect("it to return a file path");

        let method = "GET";
        let url = "https://mhouge.dk/";
        let headers = vec![
            ("x-key1".to_owned(), "x-value1".to_owned()),
            ("x-key2".to_owned(), "x-value2".to_owned()),
        ];

        let body = "{
  \"key\": \"value\"
}";

        let expected_result = format!(
            "{method} {url}
x-key1: x-value1
x-key2: x-value2

{body}
"
        );

        save_request(file.path(), method, url, &headers, Some(body.to_owned()))
            .expect("it to save succesfully");

        let result = std::fs::read_to_string(file.path()).expect("it to read the string");

        assert_eq!(result, expected_result);
    }
}

#[inline]
async fn check_if_exist(term: &Term, path: &std::path::Path) -> Result<(), std::io::Error> {
    if tokio::fs::try_exists(path).await? {
        let should_continue = confirm_input(
            term,
            &format!("File '{path:?}' already exist, do you want to continue? (y/N)"),
            &Key::Char('n'),
        )?;

        if !should_continue {
            term.flush()?;
            std::process::exit(0);
        }
    }

    Ok(())
}

pub async fn new_command(
    term: &console::Term,
    args: &NewCommandArguments,
) -> Result<(), HittCliError> {
    check_if_exist(term, &args.path).await?;

    let method = set_method(term)?;

    let url = set_url(term)?;

    let headers = set_headers(term)?;

    let body = set_body(term, try_find_content_type(&headers))?;

    save_request(&args.path, &method, &url, &headers, body)
}
