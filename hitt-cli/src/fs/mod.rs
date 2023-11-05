use hitt_parser::HittRequest;

use crate::error::HittCliError;

async fn get_file_content(path: &std::path::Path) -> Result<String, std::io::Error> {
    std::fs::read(path).map(|buf| String::from_utf8_lossy(&buf).to_string())
}

pub async fn parse_requests_threaded(
    paths: Vec<std::path::PathBuf>,
) -> Result<Vec<(std::path::PathBuf, Vec<HittRequest>)>, HittCliError> {
    let handles = paths
        .into_iter()
        .map(|path| {
            tokio::task::spawn(async move {
                (
                    get_file_content(&path)
                        .await
                        .map(|content| {
                            hitt_parser::parse_requests(&content)
                                .map_err(|error| HittCliError::Parse(path.clone(), error))
                        })
                        .map_err(|error| HittCliError::IoRead(path.clone(), error)),
                    path,
                )
            })
        })
        .collect::<Vec<_>>();

    let mut parsed_requests = Vec::new();

    for handle in handles {
        let result = handle.await.map_err(HittCliError::Join)?;

        // TODO: clean up this mess
        let reqs = result.0??;

        parsed_requests.push((reqs, result.1));
    }

    Ok(parsed_requests
        .into_iter()
        .map(|(reqs, path)| (path, reqs))
        .collect())
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
