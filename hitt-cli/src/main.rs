use clap::Parser;
use config::CliArguments;
use hitt_request::{send_request, HittResponse};
use std::str::FromStr;

mod config;

async fn get_file_content(path: std::path::PathBuf) -> anyhow::Result<String> {
    let buffr = tokio::fs::read(path).await?;

    Ok(String::from_utf8_lossy(&buffr).to_string())
}

#[inline]
fn print_status(method: &str, url: &str, status_code: u16) {
    println!("{method} {url} {status_code}");
}

#[inline]
fn print_headers(headers: &reqwest::header::HeaderMap) {
    for (key, value) in headers {
        if let Ok(value) = value.to_str() {
            println!("{key}: {value}",);
        } else {
            eprintln!("Error printing value for header: {key}");
        }
    }
}

#[inline]
fn format_json(input: &str) -> String {
    jsonformat::format(input, jsonformat::Indentation::TwoSpace)
}

#[inline]
fn print_pretty_json(input: &str) {
    let formatted_json = format_json(input);

    println!("{formatted_json}");
}

#[inline]
fn print_body(body: &str, content_type: Option<&str>) {
    match content_type {
        Some(content_type) => {
            if content_type.starts_with("application/json") {
                print_pretty_json(body);
            } else {
                println!("{body}");
            }
        }
        None => println!("{body}"),
    }
}

fn print_response(response: HittResponse, args: &CliArguments) {
    print_status(
        &response.method,
        &response.url,
        response.status_code.as_u16(),
    );

    if !args.hide_headers {
        println!("");

        print_headers(&response.headers);
    }

    if !args.hide_body && !response.body.is_empty() {
        let content_type = response
            .headers
            .get("content-type")
            .map(|x| x.to_str().expect("response content-type to be valid"));

        println!("");

        print_body(&response.body, content_type);
    }

    if args.fail_fast
        && (response.status_code.is_client_error() || response.status_code.is_server_error())
    {
        // NOTE: should the exit code be changed?
        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let args = CliArguments::parse();

    let http_client = reqwest::Client::new();

    let path = std::path::PathBuf::from_str(&args.path)?;

    let fcontent = get_file_content(path).await?;

    for req in hitt_parser::parse_requests(&fcontent).unwrap() {
        match send_request(&http_client, &req).await {
            Ok(response) => {
                print_response(response, &args);
            }
            Err(request_error) => panic!(
                "Error sending request {} {}\n{:#?}",
                req.method, req.uri, request_error,
            ),
        }
    }

    Ok(())
}
