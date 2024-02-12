use std::sync::Arc;

use crossterm::{style::Print, style::Stylize, QueueableCommand};
use hitt_request::send_request;

use crate::{
    config::{variables::parse_variable_argument, RunCommandArguments},
    error::HittCliError,
    fs::{find_http_files, parse_file, parse_files},
    terminal::handle_response,
};

pub async fn run_command<W: std::io::Write + Send>(
    term: &mut W,
    args: &RunCommandArguments,
) -> Result<(), HittCliError> {
    let http_client = reqwest::Client::new();

    let is_dir_path = std::fs::metadata(&args.path).map(|metadata| metadata.is_dir())?;

    if is_dir_path && !args.recursive {
        return Err(HittCliError::RecursiveNotEnabled);
    }

    let mut vars = std::collections::HashMap::new();

    if let Some(arg_variables) = args.var.clone() {
        for var in arg_variables {
            let (key, value) = parse_variable_argument(&var)?;

            vars.insert(key, value);
        }
    }

    let parsed_files = if is_dir_path {
        parse_files(find_http_files(&args.path), vars).await
    } else {
        parse_file(&args.path, Arc::new(vars))
            .await
            .map(|r| vec![r])
    }?;

    let timeout = args.timeout.map(core::time::Duration::from_millis);

    let mut request_count: u16 = 0;

    for (path, file) in parsed_files {
        if !args.vim {
            if request_count > 0 {
                term.queue(Print('\n'))?;
            }

            term.queue(Print(format!("hitt: running {path:?}\n").cyan()))?;
            term.flush()?;
        }

        for req in file {
            if !args.vim || request_count != 0 {
                term.queue(Print('\n'))?;
            }

            match send_request(&http_client, &req, &timeout).await {
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
