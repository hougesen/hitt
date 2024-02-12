use std::io::{stdout, Write};

use clap::Parser;
use crossterm::{
    queue,
    style::{Print, Stylize},
};

use self::{
    commands::run::run_command,
    config::{Cli, Commands},
};

mod commands;
mod config;
mod error;
mod fs;
mod terminal;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut term = stdout();

    let command_result = match &cli.command {
        Commands::Run(args) => run_command(&mut term, args).await,
    };

    if let Err(err) = command_result {
        queue!(term, Print(format!("hitt: {err}\n").red().bold()))?;
    }

    term.flush()?;

    Ok(())
}
