use std::str::FromStr;

use crate::{
    config::RunCommandArguments,
    fs::{handle_dir, handle_file, is_directory},
    terminal::print_error,
};

async fn run(
    http_client: &reqwest::Client,
    path: std::path::PathBuf,
    args: &RunCommandArguments,
) -> Result<(), std::io::Error> {
    match is_directory(&path).await {
        Ok(true) => handle_dir(http_client, path, args).await,
        Ok(false) => handle_file(http_client, path, args).await,
        Err(io_error) => {
            print_error(format!(
                "error checking if {path:?} is a directory\n{io_error:#?}"
            ));
            std::process::exit(1);
        }
    }
}

pub(crate) async fn run_command(args: &RunCommandArguments) -> Result<(), std::io::Error> {
    let http_client = reqwest::Client::new();

    match std::path::PathBuf::from_str(&args.path) {
        Ok(path) => run(&http_client, path, args).await,
        Err(parse_path_error) => {
            print_error(format!(
                "error parsing path {} as filepath\n{parse_path_error:#?}",
                args.path
            ));
            std::process::exit(1);
        }
    }
}
