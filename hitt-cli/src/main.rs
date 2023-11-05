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

    let command_result = match &cli.command {
        Commands::Run(args) => run_command(args).await,
        Commands::New(args) => new_command(args).await,
    };

    if let Err(err) = command_result {
        eprintln!("{}", err);
    }

    Ok(())
}
