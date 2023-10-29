use hitt_request::HittResponse;

use crate::{
    config::CliArguments,
    printing::{body::print_body, headers::print_headers},
};

use self::status::print_status;

mod body;
mod headers;
mod status;

pub(crate) fn print_response(response: HittResponse, args: &CliArguments) {
    print_status(
        &response.method,
        &response.url,
        response.status_code.as_u16(),
    );

    if !args.hide_headers {
        println!();

        print_headers(&response.headers);
    }

    if !args.hide_body && !response.body.is_empty() {
        let content_type = response
            .headers
            .get("content-type")
            .map(|x| x.to_str().expect("response content-type to be valid"));

        println!();

        print_body(&response.body, content_type, args.disable_formatting);
    }

    if args.fail_fast
        && (response.status_code.is_client_error() || response.status_code.is_server_error())
    {
        // NOTE: should the exit code be changed?
        std::process::exit(0);
    }
}
