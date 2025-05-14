use clap::Parser;

use crate::{
    config::{Cli, HittCommand},
    error::HittCliError,
};

mod completions;
mod run;
mod sse;

#[inline]
pub async fn execute_command<W: std::io::Write + Send>(term: &mut W) -> Result<(), HittCliError> {
    let cli = Cli::parse();

    match cli.command {
        HittCommand::Completions(args) => {
            completions::completion_command(term, &args).map_err(HittCliError::Io)
        }

        HittCommand::Run(args) => run::run_command(term, &args).await,

        HittCommand::ServerSentEvent(args) => sse::sse_command(term, args).await,
    }
}
