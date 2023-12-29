use std::sync::Arc;

use hitt_parser::HittRequest;

use crate::error::HittCliError;

#[inline]
async fn get_file_content(path: &std::path::Path) -> Result<String, std::io::Error> {
    tokio::fs::read(path)
        .await
        .map(|buf| String::from_utf8_lossy(&buf).to_string())
}

pub async fn parse_requests_threaded(
    paths: Vec<std::path::PathBuf>,
    input_variables: std::collections::HashMap<String, String>,
) -> Result<Vec<(std::path::PathBuf, Vec<HittRequest>)>, HittCliError> {
    let vars = Arc::new(input_variables);

    let handles = paths
        .into_iter()
        .map(|path| {
            let var_clone = Arc::clone(&vars);

            tokio::task::spawn(async move {
                (
                    get_file_content(&path)
                        .await
                        .map(|content| {
                            hitt_parser::parse_requests(&content, &var_clone)
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

pub fn find_http_files(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    ignore::WalkBuilder::new(path)
        .git_ignore(true)
        .require_git(false)
        .build()
        .filter_map(|orginal_entry| {
            if let Ok(entry) = orginal_entry {
                let entry_path = entry.path();

                if let Some(ext) = entry_path.extension() {
                    if ext == "http" {
                        return Some(entry_path.to_path_buf());
                    }
                }
            }

            None
        })
        .collect()
}

#[cfg(test)]
mod test_find_http_files {
    use std::io::Write;

    use super::find_http_files;

    #[test]
    fn it_should_return_a_list_of_files() {
        let dir = tempfile::Builder::new()
            .prefix("hitt-")
            .rand_bytes(12)
            .tempdir()
            .expect("it to return a valid dir");

        std::fs::create_dir_all(dir.path().join("nested")).expect("it to create dir");

        std::fs::File::create(dir.path().join("not-a-http-file.js"))
            .expect("it to create the file");

        std::fs::File::create(dir.path().join("nested/file1.http")).expect("it to create a file");

        std::fs::File::create(dir.path().join("nested/file2.http")).expect("it to create a file");

        let result = find_http_files(dir.path());

        assert_eq!(2, result.len());
    }

    #[test]
    fn it_should_respect_gitignore() {
        let dir = tempfile::Builder::new()
            .prefix("hitt-")
            .rand_bytes(12)
            .tempdir()
            .expect("it to return a valid dir");

        std::fs::create_dir_all(dir.path().join("ignored_folder"))
            .expect("it to create directories");

        std::fs::File::create(dir.path().join("not-a-http-file.js"))
            .expect("it to create the file");

        std::fs::File::create(dir.path().join("not-ignored-file.http"))
            .expect("it to create the file");

        std::fs::File::create(dir.path().join("ignored_folder/file1.http"))
            .expect("it to create the file");

        std::fs::File::create(dir.path().join("ignored_folder/file2.http"))
            .expect("it to create a file");

        let gitignore = "
ignored_folder
";

        std::fs::File::create(dir.path().join(".gitignore"))
            .expect("it to create .gitignore")
            .write_all(gitignore.as_bytes())
            .expect("it to write to .gitignore");

        let result = find_http_files(dir.path());

        assert_eq!(1, result.len());
    }
}
