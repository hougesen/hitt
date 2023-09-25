use std::str::FromStr;

use hitt_parser::HittRequest;

use tokio::fs;

async fn send_request(input: HittRequest) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let req = client
        .request(input.method, input.uri.to_string())
        .headers(input.headers)
        .body(input.body.unwrap_or_default())
        .build()?;

    client.execute(req).await
}

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

    if let Some(path_arg) = args.into_iter().nth(1) {
        let path = std::path::PathBuf::from_str(&path_arg)?;

        let fcontent = get_file_content(path).await?;

        for req in hitt_parser::parse_requests(&fcontent).unwrap() {
            let result = send_request(req).await?;

            println!("{} {}", result.status(), result.url());
        }
    }
    Ok(())
}
