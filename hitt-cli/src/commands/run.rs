use crate::{
    config::RunCommandArguments,
    fs::{handle_dir, handle_file, is_directory},
    terminal::print_error,
};

pub(crate) async fn run_command(args: &RunCommandArguments) -> Result<(), std::io::Error> {
    let http_client = reqwest::Client::new();

    // TODO: figure out a way to remove this clone
    let cloned_path = args.path.clone();

    match is_directory(&args.path).await {
        Ok(true) => handle_dir(&http_client, cloned_path, args).await,
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
