use std::str::FromStr;

use hitt_request::send_request;
use tokio::fs;

async fn get_file_content(path: std::path::PathBuf) -> anyhow::Result<String> {
    let buffr = fs::read(path).await?;

    Ok(String::from_utf8_lossy(&buffr).to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args();

    if args.len() <= 1 {
        std::process::exit(1);
    }

    let http_client = reqwest::Client::new();

    if let Some(path_arg) = args.into_iter().nth(1) {
        let path = std::path::PathBuf::from_str(&path_arg)?;

        let fcontent = get_file_content(path).await?;

        for req in hitt_parser::parse_requests(&fcontent).unwrap() {
            let result = send_request(&http_client, &req).await?;

            println!("{:?}", result);
        }
    }

    Ok(())
}
