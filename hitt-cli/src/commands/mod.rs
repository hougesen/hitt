use clap::Parser;

use crate::{
    config::{Cli, Commands},
    error::HittCliError,
};

mod completions;
mod run;

pub async fn execute_command<W: std::io::Write + Send>(term: &mut W) -> Result<(), HittCliError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => run::run_command(term, args).await,

        Commands::Completions(args) => {
            completions::completion_command(term, args);

            Ok(())
        }
    }
}
