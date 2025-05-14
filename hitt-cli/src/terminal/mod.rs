use body::print_body;
use crossterm::{
    queue,
    style::{Print, Stylize},
};
use headers::print_headers;
use hitt_formatter::ContentType;
use hitt_request::HittResponse;
use status::print_status;

use crate::{config::RunCommandArguments, error::HittCliError};

pub mod body;
mod headers;
pub mod sse;
mod status;

#[inline]
pub fn print_running_file<W: std::io::Write + Send>(
    term: &mut W,
    path: &std::path::Path,
) -> std::io::Result<()> {
    queue!(term, Print(format!("hitt: running {path:?}\n").cyan()))
}

pub fn handle_response<W: std::io::Write + Send>(
    term: &mut W,
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
            .map(|value| ContentType::from(value.to_str().unwrap_or_default()))
            .unwrap_or_default();

        print_body(term, &response.body, content_type, args.disable_formatting)?;
    }

    if args.fail_fast
        && (response.status_code.is_client_error() || response.status_code.is_server_error())
    {
        return Err(HittCliError::FailFast);
    }

    Ok(())
}

#[cfg(test)]
mod test_handle_response {
    use std::io::Write;

    use hitt_request::HittResponse;
    use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};

    use crate::{config::RunCommandArguments, error::HittCliError, terminal::handle_response};

    #[test]
    fn it_should_print_the_response() {
        let n = "mads";
        let v = "hougesen";

        let response = HittResponse {
            url: "https://mhouge.dk/".to_owned(),
            method: "GET".to_owned(),
            status_code: StatusCode::OK,
            duration: core::time::Duration::from_millis(123),
            headers: HeaderMap::from_iter([
                (HeaderName::from_static(n), HeaderValue::from_static(v)),
                (HeaderName::from_static(n), HeaderValue::from_static(n)),
                (HeaderName::from_static(v), HeaderValue::from_static(v)),
                (HeaderName::from_static(v), HeaderValue::from_static(n)),
            ]),
            http_version: http::Version::HTTP_11,
            body: "mads was here".to_owned(),
        };

        let args = RunCommandArguments {
            disable_formatting: true,
            //
            paths: vec![std::path::PathBuf::new()],
            timeout: None,
            var: None,
            recursive: false,
            fail_fast: false,
            hide_body: false,
            hide_headers: false,
            vim: false,
        };

        let mut term = Vec::new();

        handle_response(&mut term, &response, &args).expect("it to be ok");

        let status = format!(
            "\x1b[38;5;10m\x1B[1m{:?} {} {} {} {}ms\n\x1B[0m",
            response.http_version,
            response.method,
            response.url,
            response.status_code.as_u16(),
            response.duration.as_millis(),
        );

        let headers = format!(
            "\x1B[38;5;3m{n}\x1B[39m: {v}\n\x1B[38;5;3m{n}\x1B[39m: {n}\n\x1B[38;5;3m{v}\x1B[39m: {v}\n\x1B[38;5;3m{v}\x1B[39m: {n}\n"
        );

        let body = format!("\n\x1B[38;5;3m{}\x1B[39m\n\n", response.body);

        let expected_response = format!("{status}{headers}{body}");

        term.flush().expect("it to flush");

        assert_eq!(expected_response, String::from_utf8_lossy(&term));
    }

    #[test]
    fn it_should_format_json_body() {
        let n = "content-type";
        let v = "application/json";

        let response = HittResponse {
            url: "https://mhouge.dk/".to_owned(),
            method: "GET".to_owned(),
            status_code: StatusCode::OK,
            duration: core::time::Duration::from_millis(123),
            headers: HeaderMap::from_iter([(
                HeaderName::from_static(n),
                HeaderValue::from_static(v),
            )]),
            http_version: http::Version::HTTP_11,
            body: "{\"key\": \"value\"}".to_owned(),
        };

        let args = RunCommandArguments {
            disable_formatting: false,
            paths: vec![std::path::PathBuf::new()],
            timeout: None,
            var: None,
            recursive: false,
            fail_fast: false,
            hide_body: false,
            hide_headers: false,
            vim: false,
        };

        let mut term = Vec::new();

        handle_response(&mut term, &response, &args).expect("it to be ok");

        let status = format!(
            "\x1b[38;5;10m\x1B[1m{:?} {} {} {} {}ms\n\x1B[0m",
            response.http_version,
            response.method,
            response.url,
            response.status_code.as_u16(),
            response.duration.as_millis(),
        );

        let headers = format!("\x1B[38;5;3m{n}\x1B[39m: {v}\n");

        let body = "\n\x1B[38;5;3m{\n  \"key\": \"value\"\n}\x1B[39m\n\n";

        let expected_response = format!("{status}{headers}{body}");

        term.flush().expect("it to flush");

        assert_eq!(expected_response, String::from_utf8_lossy(&term));
    }

    #[test]
    fn it_should_not_print_headers_if_hide_headers_enabled() {
        let response = HittResponse {
            url: "https://mhouge.dk/".to_owned(),
            method: "GET".to_owned(),
            status_code: StatusCode::OK,
            duration: core::time::Duration::from_millis(123),
            headers: HeaderMap::from_iter([(
                HeaderName::from_static("mads"),
                HeaderValue::from_static("hougesen"),
            )]),
            http_version: http::Version::HTTP_11,
            body: "mads was here".to_owned(),
        };

        let args = RunCommandArguments {
            hide_headers: true,
            disable_formatting: true,
            //
            paths: vec![std::path::PathBuf::new()],
            timeout: None,
            var: None,
            recursive: false,
            fail_fast: false,
            hide_body: false,
            vim: false,
        };

        let mut term = Vec::new();

        handle_response(&mut term, &response, &args).expect("it to be ok");

        let expected_response = format!(
            "\x1b[38;5;10m\x1B[1m{:?} {} {} {} {}ms\n\x1B[0m\n\x1B[38;5;3m{}\x1B[39m\n\n",
            response.http_version,
            response.method,
            response.url,
            response.status_code.as_u16(),
            response.duration.as_millis(),
            response.body,
        );

        term.flush().expect("it to flush");

        assert_eq!(expected_response, String::from_utf8_lossy(&term));
    }

    #[test]
    fn it_should_not_print_body_if_empty() {
        let n = "mads";
        let v = "hougesen";

        let response = HittResponse {
            url: "https://mhouge.dk/".to_owned(),
            method: "GET".to_owned(),
            status_code: StatusCode::OK,
            duration: core::time::Duration::from_millis(123),
            headers: HeaderMap::from_iter([
                (HeaderName::from_static(n), HeaderValue::from_static(v)),
                (HeaderName::from_static(n), HeaderValue::from_static(n)),
                (HeaderName::from_static(v), HeaderValue::from_static(v)),
                (HeaderName::from_static(v), HeaderValue::from_static(n)),
            ]),
            http_version: http::Version::HTTP_11,
            body: String::new(),
        };

        let args = RunCommandArguments {
            disable_formatting: true,
            hide_body: true,
            //
            paths: vec![std::path::PathBuf::new()],
            timeout: None,
            var: None,
            recursive: false,
            fail_fast: false,
            hide_headers: false,
            vim: false,
        };

        let mut term = Vec::new();

        handle_response(&mut term, &response, &args).expect("it to be ok");

        let status = format!(
            "\x1b[38;5;10m\x1B[1m{:?} {} {} {} {}ms\n\x1B[0m",
            response.http_version,
            response.method,
            response.url,
            response.status_code.as_u16(),
            response.duration.as_millis(),
        );

        let headers = format!(
            "\x1B[38;5;3m{n}\x1B[39m: {v}\n\x1B[38;5;3m{n}\x1B[39m: {n}\n\x1B[38;5;3m{v}\x1B[39m: {v}\n\x1B[38;5;3m{v}\x1B[39m: {n}\n"
        );

        let expected_response = format!("{status}{headers}");

        term.flush().expect("it to flush");

        assert_eq!(expected_response, String::from_utf8_lossy(&term));
    }

    #[test]
    fn it_should_not_print_body_if_hide_body_enabled() {
        let n = "mads";
        let v = "hougesen";

        let response = HittResponse {
            url: "https://mhouge.dk/".to_owned(),
            method: "GET".to_owned(),
            status_code: StatusCode::OK,
            duration: core::time::Duration::from_millis(123),
            headers: HeaderMap::from_iter([
                (HeaderName::from_static(n), HeaderValue::from_static(v)),
                (HeaderName::from_static(n), HeaderValue::from_static(n)),
                (HeaderName::from_static(v), HeaderValue::from_static(v)),
                (HeaderName::from_static(v), HeaderValue::from_static(n)),
            ]),
            http_version: http::Version::HTTP_11,
            body: "mads was here".to_owned(),
        };

        let args = RunCommandArguments {
            disable_formatting: true,
            hide_body: true,
            //
            paths: vec![std::path::PathBuf::new()],
            timeout: None,
            var: None,
            recursive: false,
            fail_fast: false,
            hide_headers: false,
            vim: false,
        };

        let mut term = Vec::new();

        handle_response(&mut term, &response, &args).expect("it to be ok");

        let status = format!(
            "\x1b[38;5;10m\x1B[1m{:?} {} {} {} {}ms\n\x1B[0m",
            response.http_version,
            response.method,
            response.url,
            response.status_code.as_u16(),
            response.duration.as_millis(),
        );

        let headers = format!(
            "\x1B[38;5;3m{n}\x1B[39m: {v}\n\x1B[38;5;3m{n}\x1B[39m: {n}\n\x1B[38;5;3m{v}\x1B[39m: {v}\n\x1B[38;5;3m{v}\x1B[39m: {n}\n"
        );

        let expected_response = format!("{status}{headers}");

        term.flush().expect("it to flush");

        assert_eq!(expected_response, String::from_utf8_lossy(&term));
    }

    #[test]
    fn it_should_return_early_if_fail_fast_enabled() {
        let n = "mads";
        let v = "hougesen";

        let response = HittResponse {
            url: "https://mhouge.dk/".to_owned(),
            method: "GET".to_owned(),
            status_code: StatusCode::SERVICE_UNAVAILABLE,
            duration: core::time::Duration::from_millis(123),
            headers: HeaderMap::from_iter([(
                HeaderName::from_static(n),
                HeaderValue::from_static(v),
            )]),
            http_version: http::Version::HTTP_11,
            body: "mads was here".to_owned(),
        };

        let args = RunCommandArguments {
            disable_formatting: true,
            fail_fast: true,
            //
            paths: vec![std::path::PathBuf::new()],
            timeout: None,
            var: None,
            recursive: false,
            hide_headers: false,
            hide_body: false,
            vim: false,
        };

        let mut term = Vec::new();

        let error = handle_response(&mut term, &response, &args).expect_err("it to fail fast");

        assert!(matches!(error, HittCliError::FailFast));

        let status = format!(
            "\x1b[38;5;9m\x1B[1m{:?} {} {} {} {}ms\n\x1B[0m",
            response.http_version,
            response.method,
            response.url,
            response.status_code.as_u16(),
            response.duration.as_millis(),
        );

        let headers = format!("\x1B[38;5;3m{n}\x1B[39m: {v}\n");

        let body = format!("\n\x1B[38;5;3m{}\x1B[39m\n\n", response.body);

        let expected_response = format!("{status}{headers}{body}");

        term.flush().expect("it to flush");

        assert_eq!(expected_response, String::from_utf8_lossy(&term));
    }
}
