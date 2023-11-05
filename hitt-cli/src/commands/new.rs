use std::str::FromStr;

use console::Term;
use hitt_parser::http::{HeaderName, HeaderValue, Uri};

use crate::{
    config::NewCommandArguments,
    terminal::{
        input::{boolean_input, select_input, text_input_prompt},
        print_error, TEXT_RED, TEXT_RESET,
    },
};

fn set_method(term: &Term) -> Result<String, std::io::Error> {
    let http_methods = [
        "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
    ];

    select_input(term, "Which HTTP method?", &http_methods)
}

fn set_url(term: &Term) -> Result<String, std::io::Error> {
    text_input_prompt(
        term,
        "What should the url be?",
        |input| !input.is_empty() && Uri::from_str(input).is_ok(),
        |input| format!("{TEXT_RED}'{input}' is not a valid url{TEXT_RESET}"),
    )
}

fn set_headers(term: &Term) -> Result<Vec<(String, String)>, std::io::Error> {
    let mut headers = Vec::new();

    let mut writing_headers = boolean_input(term, "Do you want to add headers? (Y/n)")?;

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

        writing_headers = boolean_input(term, "Do you want to add more headers? (Y/n)")?;
    }

    Ok(headers)
}

fn set_body(term: &Term) -> Result<Option<String>, std::io::Error> {
    if !boolean_input(term, "Do you want to add a body? (Y/n)")? {
        return Ok(None);
    }

    let body = text_input_prompt(
        term,
        "What should the body be?",
        |input| !input.is_empty(),
        |_| String::new(),
    )?;

    Ok(Some(body))
}

async fn save_request(
    path: &std::path::Path,
    method: String,
    url: String,
    headers: &[(String, String)],
    body: Option<String>,
) -> Result<(), std::io::Error> {
    let mut contents = format!("{method} {url}\n");

    if !headers.is_empty() {
        for (key, value) in headers {
            contents.push_str(key);
            contents.push_str(": ");
            contents.push_str(value);
            contents.push('\n');
        }
    }

    if let Some(body) = body {
        contents.push('\n');
        contents.push_str(&body);
        contents.push('\n');
    }

    tokio::fs::write(path, contents).await
}

pub(crate) async fn new_command(args: &NewCommandArguments) -> Result<(), std::io::Error> {
    match std::path::PathBuf::from_str(&args.path) {
        Ok(path) => {
            let term = console::Term::stdout();

            let method = set_method(&term)?;

            let url = set_url(&term)?;

            let headers = set_headers(&term)?;

            let body = set_body(&term)?;

            save_request(&path, method, url, &headers, body).await
        }
        Err(parse_path_error) => {
            print_error(format!(
                "error parsing path {} as filepath\n{parse_path_error:#?}",
                args.path
            ));
            std::process::exit(1);
        }
    }
}
