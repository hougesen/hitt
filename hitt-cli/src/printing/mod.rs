use hitt_request::HittResponse;

use crate::{
    config::CliArguments,
    printing::{body::print_body, headers::print_headers},
};

use self::status::print_status;

mod body;
mod headers;
mod status;

pub const STYLE_RESET: &str = "\x1B[0m";

pub const STYLE_BOLD: &str = "\x1B[1m";

pub const TEXT_RED: &str = "\x1B[31m";

pub const TEXT_GREEN: &str = "\x1B[32m";

pub const TEXT_YELLOW: &str = "\x1B[33m";

pub const TEXT_RESET: &str = "\x1B[39m";

pub(crate) fn handle_response(response: HittResponse, args: &CliArguments) {
    print_status(
        &response.http_version,
        &response.method,
        &response.url,
        response.status_code.as_u16(),
        &response.duration,
    );

    if !args.hide_headers {
        print_headers(&response.headers);
    }

    if !args.hide_body && !response.body.is_empty() {
        let content_type = response
            .headers
            .get("content-type")
            .map(|x| x.to_str().expect("response content-type to be valid"));

        print_body(&response.body, content_type, args.disable_formatting);
    }

    if args.fail_fast
        && (response.status_code.is_client_error() || response.status_code.is_server_error())
    {
        // NOTE: should the exit code be changed?
        std::process::exit(0);
    }
}

#[inline]
pub fn print_error(message: String) {
    eprintln!("{TEXT_RED}hitt: {message}{TEXT_RESET}")
}
