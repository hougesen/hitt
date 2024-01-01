use console::Term;
use hitt_formatter::ContentType;
use hitt_request::HittResponse;

use crate::{config::RunCommandArguments, error::HittCliError};

use self::{body::print_body, headers::print_headers, status::print_status};

pub mod body;
pub mod editor;
pub mod headers;
pub mod input;
pub mod status;

pub fn handle_response(
    term: &console::Term,
    response: &HittResponse,
    args: &RunCommandArguments,
) -> Result<(), HittCliError> {
    print_status(
        term,
        response.http_version,
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
            .map_or(ContentType::Unknown, |value| {
                ContentType::from(value.to_str().unwrap_or_default())
            });

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

#[inline]
pub fn write_prompt(term: &Term, prompt: &str) -> Result<(), std::io::Error> {
    term.write_line(prompt)
}

#[cfg(test)]
mod test_write_prompt {
    use super::write_prompt;

    #[test]
    fn it_should_not_error() {
        let term = console::Term::stdout();

        // TODO: actually validate stdout
        write_prompt(&term, "What is your prefered http testing tool?")
            .expect("it not to raise an error");
    }
}

#[inline]
pub fn write_prompt_answer(term: &Term, prompt: &str, answer: &str) -> Result<(), std::io::Error> {
    term.write_line(&format!("{prompt} [{}]", console::style(answer).green()))
}

#[cfg(test)]
mod test_write_prompt_answer {
    use super::write_prompt_answer;

    #[test]
    fn it_should_not_error() {
        let term = console::Term::stdout();

        // TODO: actually validate stdout
        write_prompt_answer(&term, "What is your prefered http testing tool?", "hitt")
            .expect("it not to raise an error");
    }
}
