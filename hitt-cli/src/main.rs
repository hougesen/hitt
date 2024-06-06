use std::io::{stdout, Write};

use clap::{CommandFactory, Parser};
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

        Commands::Completions(args) => {
            let mut cmd = Cli::command();
            let cmd_name = cmd.get_name().to_string();

            clap_complete::generate(args.shell, &mut cmd, cmd_name, &mut term);

            Ok(())
        }
    };

    if let Err(err) = command_result {
        queue!(term, Print(format!("hitt: {err}\n").red().bold()))?;
    }

    term.flush()?;

    Ok(())
}
