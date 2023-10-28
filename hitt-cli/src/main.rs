use std::str::FromStr;

use hitt_request::{send_request, HittResponse};
use tokio::fs;

async fn get_file_content(path: std::path::PathBuf) -> anyhow::Result<String> {
    let buffr = fs::read(path).await?;

    Ok(String::from_utf8_lossy(&buffr).to_string())
}

struct CliConfig {
    /// Crash on request error if set to true
    fail_fast: bool,

    /// Whether or not to show response body
    print_body: bool,

    /// Whether or not to show response headers
    print_headers: bool,
}

impl Default for CliConfig {
    fn default() -> CliConfig {
        CliConfig {
            fail_fast: true,
            print_body: true,
            print_headers: true,
        }
    }
}

fn print_status(method: &str, url: &str, status_code: u16) {
    println!("{method} {url} {status_code}");
}

fn print_headers(headers: &reqwest::header::HeaderMap) {
    for (key, value) in headers {
        if let Ok(value) = value.to_str() {
            println!("{key}: {value}",);
        } else {
            eprintln!("Error printing value for header: {key}");
        }
    }
}

fn print_json(body: &str) {
    println!("{body}");
}

fn print_body(body: &str, content_type: Option<&str>) {
    match content_type {
        Some(content_type) => match content_type {
            "application/json" => print_json(body),

            _ => println!("{body}"),
        },
        None => println!("{body}"),
    }
}

fn print_response(response: HittResponse, config: &CliConfig) {
    print_status(
        &response.method,
        &response.url,
        response.status_code.as_u16(),
    );

    if config.print_headers {
        print_headers(&response.headers);
    }

    if config.print_body && !response.body.is_empty() {
        let content_type = response
            .headers
            .get("content-type")
            .map(|x| x.to_str().expect("response content-type to be valid"));

        println!("");

        print_body(&response.body, content_type);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args();

    if args.len() <= 1 {
        std::process::exit(1);
    }

    let config = CliConfig::default();

    let http_client = reqwest::Client::new();

    if let Some(path_arg) = args.into_iter().nth(1) {
        let path = std::path::PathBuf::from_str(&path_arg)?;

        let fcontent = get_file_content(path).await?;

        for req in hitt_parser::parse_requests(&fcontent).unwrap() {
            match send_request(&http_client, &req).await {
                Ok(response) => {
                    print_response(response, &config);
                }

                Err(request_error) => {
                    let error_message = format!(
                        "Error sending request {} {}\n{:#?}",
                        req.method, req.uri, request_error,
                    );

                    if config.fail_fast {
                        panic!("{}", error_message);
                    }

                    eprintln!("{}", error_message);
                }
            }
        }
    }

    Ok(())
}
