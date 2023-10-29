use std::str::FromStr;

use clap::Parser;
use config::CliArguments;
use hitt_request::send_request;
use printing::print_response;

mod config;
mod printing;

async fn get_file_content(path: std::path::PathBuf) -> anyhow::Result<String> {
    let buffr = tokio::fs::read(path).await?;

    Ok(String::from_utf8_lossy(&buffr).to_string())
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
