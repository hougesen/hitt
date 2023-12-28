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

pub(crate) async fn new_command(
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
