use crossterm::{QueueableCommand, style::Print};
use hitt_parser::HittRequest;
use hitt_request::send_request;

use crate::{
    config::{RunCommandArguments, variables::parse_variable_argument},
    error::HittCliError,
    fs::{find_http_files, parse_files},
    terminal::{handle_response, print_running_file},
};

#[inline]
fn build_variable_map(
    var: Option<&Vec<String>>,
) -> Result<std::collections::HashMap<String, String>, HittCliError> {
    let mut vars = std::collections::HashMap::new();

    if let Some(arg_variables) = var {
        for var in arg_variables {
            let (key, value) = parse_variable_argument(var)?;

            vars.insert(key, value);
        }
    }

    Ok(vars)
}

#[cfg(test)]
mod test_build_variable_map {
    use super::build_variable_map;

    #[test]
    fn it_should_parse_variables() {
        let input = vec![
            "name=hougesen".to_owned(),
            "host=https://mhouge.dk/?query=asd".to_owned(),
        ];

        let variables = build_variable_map(Some(&input)).expect("it to return a map");

        assert_eq!(variables.len(), 2);

        let host_var = variables.get("host").expect("it to be some");
        assert_eq!(host_var, "https://mhouge.dk/?query=asd");

        let name_var = variables.get("name").expect("it to be some");
        assert_eq!(name_var, "hougesen");
    }
}

async fn get_requests(
    input_paths: &[std::path::PathBuf],
    recursive: bool,
    var_input: Option<&Vec<String>>,
) -> Result<Vec<(std::path::PathBuf, Vec<HittRequest>)>, HittCliError> {
    let mut found_paths = Vec::new();

    for path in input_paths {
        let is_dir_path = std::fs::metadata(path).map(|metadata| metadata.is_dir())?;

        if is_dir_path {
            if !recursive {
                return Err(HittCliError::RecursiveNotEnabled);
            }

            for p in find_http_files(path) {
                if !found_paths.contains(&p) {
                    found_paths.push(p);
                }
            }
        } else if !found_paths.contains(path) {
            found_paths.push(path.clone());
        }
    }

    parse_files(found_paths, build_variable_map(var_input)?).await
}

#[cfg(test)]
mod test_get_requests {
    use crate::{commands::run::get_requests, error::HittCliError};

    #[tokio::test]
    async fn it_should_return_a_list_of_requests() {
        let f = tempfile::Builder::new()
            .prefix("hitt-")
            .rand_bytes(12)
            .suffix(".hitt")
            .tempfile()
            .expect("it to create a file");

        std::fs::write(f.path(), "GET https://mhouge.dk/").expect("it to write successfully");

        let files = get_requests(&[f.path().to_path_buf()], false, None)
            .await
            .expect("it to return a list of requests");

        assert_eq!(1, files.len());

        let file = files.first().expect("it to be some");
        let requests = &file.1;

        assert_eq!(requests.len(), 1);

        let req = requests.first().expect("it to be some");

        assert_eq!(req.uri.to_string(), "https://mhouge.dk/");
        assert_eq!(req.method, http::Method::GET);
        assert!(req.headers.is_empty());
        assert!(req.http_version.is_none());
        assert!(req.body.is_none());
    }

    #[tokio::test]
    async fn is_should_reject_dir_when_recursive_false() {
        let dir = tempfile::Builder::new()
            .prefix("hitt-")
            .rand_bytes(12)
            .suffix(".hitt")
            .tempdir()
            .expect("it to create a dir");

        let p = dir.path();

        let err = get_requests(&[p.to_path_buf()], false, None)
            .await
            .expect_err("expect it to return a missing recursive arg error");

        assert!(matches!(err, HittCliError::RecursiveNotEnabled));
    }

    #[tokio::test]
    async fn it_should_allow_dir_when_recursive_true() {
        let dir = tempfile::Builder::new()
            .prefix("hitt-")
            .rand_bytes(12)
            .suffix(".hitt")
            .tempdir()
            .expect("it to create a file");

        let dir_path = dir.path();

        let file_path = dir_path.join("file.http");

        std::fs::write(&file_path, "GET https://mhouge.dk/").expect("it to write successfully");

        let files = get_requests(&[dir_path.to_path_buf()], true, None)
            .await
            .expect("it to return a list of requests");

        assert_eq!(1, files.len());

        let file = files.first().expect("it to be some");

        assert_eq!(&file.0, &file_path);

        let requests = &file.1;

        assert_eq!(requests.len(), 1);

        let req = requests.first().expect("it to be some");

        assert_eq!(req.uri.to_string(), "https://mhouge.dk/");
        assert_eq!(req.method, http::Method::GET);
        assert!(req.headers.is_empty());
        assert!(req.http_version.is_none());
        assert!(req.body.is_none());
    }
}

pub async fn run_command<W: std::io::Write + Send>(
    term: &mut W,
    args: &RunCommandArguments,
) -> Result<(), HittCliError> {
    let http_client = reqwest::ClientBuilder::new()
        .http09_responses()
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    let timeout = args.timeout.map(core::time::Duration::from_millis);

    let mut request_count: u16 = 0;

    for (path, file) in get_requests(&args.paths, args.recursive, args.var.as_ref()).await? {
        if !args.vim {
            if request_count > 0 {
                term.queue(Print('\n'))?;
            }

            print_running_file(term, &path)?;

            term.flush()?;
        }

        for req in file {
            if !args.vim || request_count != 0 {
                term.queue(Print('\n'))?;
            }

            match send_request(&http_client, &req, timeout.as_ref()).await {
                Ok(response) => handle_response(term, &response, args),
                Err(request_error) => {
                    if request_error.is_timeout() {
                        return Err(HittCliError::RequestTimeout(req.method, req.uri));
                    }

                    Err(HittCliError::Reqwest(req.method, req.uri, request_error))
                }
            }?;
            request_count += 1;
        }
    }

    Ok(())
}
