use std::io::{stdout, Write};

use clap::Parser;
use crossterm::{
    queue,
    style::{Print, Stylize},
};

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

    let mut term = stdout();

    let command_result = match &cli.command {
        Commands::Run(args) => run_command(&mut term, args).await,
        Commands::New(args) => new_command(&console::Term::stdout(), args).await,
    };

    if let Err(err) = command_result {
        queue!(term, Print(err.to_string().red().bold()), Print("\n"))?;
    }

    term.flush()?;

    Ok(())
}
