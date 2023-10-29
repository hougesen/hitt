use async_recursion::async_recursion;
use hitt_request::send_request;

use crate::{
    config::CliArguments,
    printing::{handle_response, print_error},
};

async fn get_file_content(path: std::path::PathBuf) -> Result<String, std::io::Error> {
    let buffr = tokio::fs::read(path).await?;

    Ok(String::from_utf8_lossy(&buffr).to_string())
}

pub(crate) async fn is_directory(path: &std::path::Path) -> Result<bool, std::io::Error> {
    tokio::fs::metadata(path).await.map(|m| m.is_dir())
}

pub(crate) async fn handle_file(
    http_client: &reqwest::Client,
    path: std::path::PathBuf,
    args: &CliArguments,
) -> Result<(), std::io::Error> {
    println!("hitt: running {path:?}");

    let fcontent = get_file_content(path).await?;

    for req in hitt_parser::parse_requests(&fcontent).unwrap() {
        match send_request(http_client, &req).await {
            Ok(response) => handle_response(response, args),
            Err(request_error) => {
                print_error(format!(
                    "error sending request {} {}\n{request_error:#?}",
                    req.method, req.uri,
                ));
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

async fn handle_dir_entry(
    http_client: &reqwest::Client,
    entry: tokio::fs::DirEntry,
    args: &CliArguments,
) -> Result<(), std::io::Error> {
    let metadata = entry.metadata().await?;

    match metadata.is_dir() {
        true => handle_dir(http_client, entry.path(), args).await,
        false => {
            let entry_path = entry.path();

            if let Some(ext) = entry_path.extension() {
                if ext == "http" || ext == "rest" {
                    return handle_file(http_client, entry_path, args).await;
                }
            }

            Ok(())
        }
    }
}

#[async_recursion]
pub(crate) async fn handle_dir(
    http_client: &reqwest::Client,
    path: std::path::PathBuf,
    args: &CliArguments,
) -> Result<(), std::io::Error> {
    if !args.recursive {
        print_error(format!(
            "{path:?} is a directory, but the recursive argument was not supplied"
        ));

        std::process::exit(1);
    }

    let mut read_dir_result = tokio::fs::read_dir(&path).await?;

    while let Some(entry) = read_dir_result.next_entry().await? {
        handle_dir_entry(http_client, entry, args).await?;
    }

    Ok(())
}
