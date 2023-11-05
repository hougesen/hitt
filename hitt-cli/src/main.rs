use clap::Parser;
use commands::{new::new_command, run::run_command};
use config::{Cli, Commands};

mod commands;
mod config;
mod fs;
mod terminal;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => run_command(args).await,
        Commands::New(args) => new_command(args).await,
    }
}
