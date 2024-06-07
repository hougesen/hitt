use std::io::{stdout, Write};

use commands::execute_command;
use crossterm::{
    queue,
    style::{Print, Stylize},
};

mod commands;
mod config;
mod error;
mod fs;
mod terminal;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut term = stdout();

    if let Err(err) = execute_command(&mut term).await {
        queue!(term, Print(format!("hitt: {err}\n").red().bold()))?;
    }

    term.flush()
}
