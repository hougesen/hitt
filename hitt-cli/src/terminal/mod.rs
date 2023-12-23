use console::Term;
use hitt_request::HittResponse;

use crate::{config::RunCommandArguments, error::HittCliError};

use self::{body::print_body, headers::print_headers, status::print_status};

pub(crate) mod body;
pub(crate) mod editor;
pub(crate) mod headers;
pub(crate) mod input;
pub(crate) mod status;

pub const STYLE_RESET: &str = "\x1B[0m";

pub const STYLE_BOLD: &str = "\x1B[1m";

pub const TEXT_RED: &str = "\x1B[31m";

pub const TEXT_GREEN: &str = "\x1B[32m";

pub const TEXT_YELLOW: &str = "\x1B[33m";

pub const TEXT_RESET: &str = "\x1B[39m";

pub(crate) fn handle_response(
    term: &console::Term,
    response: HittResponse,
    args: &RunCommandArguments,
) -> Result<(), HittCliError> {
    print_status(
        term,
        &response.http_version,
        &response.method,
        &response.url,
        response.status_code.as_u16(),
        &response.duration,
    )?;

    if !args.hide_headers {
        print_headers(term, &response.headers)?;
    }

    if !args.hide_body && !response.body.is_empty() {
        let content_type = response
            .headers
            .get("content-type")
            .map(|x| x.to_str().expect("response content-type to be valid"));

        print_body(term, &response.body, content_type, args.disable_formatting)?;
    }

    if args.fail_fast
        && (response.status_code.is_client_error() || response.status_code.is_server_error())
    {
        term.flush()?;
        // NOTE: should the exit code be changed?
        std::process::exit(0);
    }

    Ok(())
}

pub(crate) fn write_prompt(term: &Term, prompt: &str) -> Result<(), std::io::Error> {
    term.write_line(prompt)
}

pub(crate) fn write_prompt_answer(
    term: &Term,
    prompt: &str,
    answer: &str,
) -> Result<(), std::io::Error> {
    term.write_line(&format!("{prompt} {TEXT_GREEN}[{answer}]{TEXT_RESET}"))
}