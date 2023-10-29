use std::str::FromStr;

use clap::Parser;
use config::CliArguments;
use fs::{handle_dir, handle_file, is_directory};
use printing::print_error;

mod config;
mod fs;
mod printing;

async fn run(
    http_client: &reqwest::Client,
    path: std::path::PathBuf,
    args: &CliArguments,
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

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = CliArguments::parse();

    let http_client = reqwest::Client::new();

    match std::path::PathBuf::from_str(&args.path) {
        Ok(path) => run(&http_client, path, &args).await,
        Err(parse_path_error) => {
            print_error(format!(
                "error parsing path {} as filepath\n{parse_path_error:#?}",
                args.path
            ));
            std::process::exit(1);
        }
    }
}
