use clap::CommandFactory;

use crate::config::{Cli, CompletionsCommandArguments};

pub fn completion_command<W: std::io::Write + Send>(
    term: &mut W,
    args: &CompletionsCommandArguments,
) {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    clap_complete::generate(args.shell, &mut cmd, cmd_name, term);
}
