use crate::{
    config::RunCommandArguments,
    fs::{find_http_files, handle_file},
    terminal::print_error,
};

pub(crate) async fn run_command(args: &RunCommandArguments) -> Result<(), std::io::Error> {
    let http_client = reqwest::Client::new();

    // TODO: figure out a way to remove this clone
    let cloned_path = args.path.clone();

    match std::fs::metadata(&args.path).map(|metadata| metadata.is_dir()) {
        Ok(true) => {
            let http_files = find_http_files(&args.path);

            for http_file in http_files {
                handle_file(&http_client, http_file, args).await?;
            }

            Ok(())
        }
        Ok(false) => handle_file(&http_client, cloned_path, args).await,
        Err(io_error) => {
            print_error(format!(
                "error checking if {:?} is a directory\n{io_error:#?}",
                args.path
            ));
            std::process::exit(1);
        }
    }
}
