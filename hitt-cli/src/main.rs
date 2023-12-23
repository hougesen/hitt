use clap::Parser;

use self::{
    commands::{new::new_command, run::run_command},
    config::{Cli, Commands},
    error::HittCliError,
};

mod commands;
mod config;
mod error;
mod fs;
mod terminal;

#[tokio::main]
async fn main() -> Result<(), HittCliError> {
    let cli = Cli::parse();

    let term = console::Term::stdout();

    let command_result = match &cli.command {
        Commands::Run(args) => run_command(&term, args).await,
        Commands::New(args) => new_command(&term, args).await,
    };

    if let Err(err) = command_result {
        term.write_line(&err.to_string())?;
    }

    term.flush()?;

    Ok(())
}
