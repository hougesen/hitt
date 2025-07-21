use std::sync::Arc;

use futures::future::TryJoinAll;
use hitt_parser::HittRequest;

use crate::error::HittCliError;

#[inline]
pub async fn parse_file(
    path: &std::path::Path,
    input_variables: Arc<std::collections::HashMap<String, String>>,
) -> Result<(std::path::PathBuf, Vec<HittRequest>), HittCliError> {
    match tokio::fs::read(&path).await {
        Ok(buf) => {
            let content = String::from_utf8_lossy(&buf);

            match hitt_parser::parse_requests(&content, &input_variables) {
                Ok(reqs) => Ok((path.to_owned(), reqs)),
                Err(e) => Err(HittCliError::Parse(path.to_owned(), e)),
            }
        }
        Err(err) => Err(HittCliError::IoRead(path.to_owned(), err)),
    }
}

#[inline]
pub async fn parse_files(
    paths: Vec<std::path::PathBuf>,
    input_variables: std::collections::HashMap<String, String>,
) -> Result<Vec<(std::path::PathBuf, Vec<HittRequest>)>, HittCliError> {
    let vars = Arc::new(input_variables);

    let handles = paths
        .into_iter()
        .map(|path| {
            let var_clone = Arc::clone(&vars);

            tokio::task::spawn(async move { parse_file(&path, var_clone).await })
        })
        .collect::<TryJoinAll<_>>()
        .await
        .map_err(HittCliError::Join)?;

    let mut parsed_requests = Vec::new();

    for handle in handles {
        parsed_requests.push(handle?);
    }

    Ok(parsed_requests)
}

#[inline]
pub fn find_http_files(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    ignore::WalkBuilder::new(path)
        .git_ignore(true)
        .require_git(false)
        .add_custom_ignore_filename(".hittignore")
        .build()
        .filter_map(|original_entry| {
            if let Ok(entry) = original_entry {
                let entry_path = entry.path();

                if let Some(ext) = entry_path.extension()
                    && ext == "http"
                {
                    return Some(entry_path.to_path_buf());
                }
            }

            None
        })
        .collect()
}

#[cfg(test)]
mod test_find_http_files {
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

        let gitignore = "ignored_folder";

        std::fs::write(dir.path().join(".gitignore"), gitignore)
            .expect("it to write to .gitignore");

        let result = find_http_files(dir.path());

        assert_eq!(1, result.len());
    }
}
