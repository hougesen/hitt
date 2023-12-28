use hitt_request::send_request;

use crate::{
    config::RunCommandArguments,
    error::HittCliError,
    fs::{find_http_files, parse_requests_threaded},
    terminal::handle_response,
};

pub async fn run_command(
    term: &console::Term,
    args: &RunCommandArguments,
) -> Result<(), HittCliError> {
    let http_client = reqwest::Client::new();

    let http_file_paths = if std::fs::metadata(&args.path).map(|metadata| metadata.is_dir())? {
        find_http_files(&args.path)
    } else {
        vec![args.path.clone()]
    };

    let timeout = args.timeout.map(core::time::Duration::from_millis);

    let parsed_files = parse_requests_threaded(http_file_paths).await?;

    for (path, file) in parsed_files {
        if !args.vim {
            term.write_line(&format!("hitt: running {path:?}"))?;
        }

        for req in file {
            match send_request(&http_client, &req, &timeout).await {
                Ok(response) => handle_response(term, &response, args),
                Err(request_error) => {
                    if request_error.is_timeout() {
                        return Err(HittCliError::RequestTimeout(req.method, req.uri));
                    }

                    Err(HittCliError::Reqwest(req.method, req.uri, request_error))
                }
            }?;
        }
    }

    term.flush()?;

    Ok(())
}
