use hitt_request::send_request;

use crate::{
    config::RunCommandArguments,
    terminal::{handle_response, print_error},
};

async fn get_file_content(path: std::path::PathBuf) -> Result<String, std::io::Error> {
    tokio::fs::read(path)
        .await
        .map(|buf| String::from_utf8_lossy(&buf).to_string())
}

pub(crate) async fn handle_file(
    http_client: &reqwest::Client,
    path: std::path::PathBuf,
    args: &RunCommandArguments,
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

pub(crate) fn find_http_files(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    ignore::WalkBuilder::new(path)
        .build()
        .filter_map(|orginal_entry| {
            if let Ok(entry) = orginal_entry {
                let path = entry.path();

                if let Some(ext) = path.extension() {
                    if ext == "http" {
                        return Some(path.to_path_buf());
                    }
                }

                None
            } else {
                None
            }
        })
        .collect()
}
