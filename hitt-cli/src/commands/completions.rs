use clap::CommandFactory;

use crate::config::{Cli, CompletionsCommandArguments, TerminalShell};

#[inline]
pub fn completion_command<W: std::io::Write + Send>(
    term: &mut W,
    args: &CompletionsCommandArguments,
) -> std::io::Result<()> {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    match args.shell {
        TerminalShell::Bash => {
            clap_complete::generate(clap_complete::Shell::Bash, &mut cmd, cmd_name, term);
        }

        TerminalShell::Elvish => {
            clap_complete::generate(clap_complete::Shell::Elvish, &mut cmd, cmd_name, term);
        }

        TerminalShell::Powershell => {
            clap_complete::generate(clap_complete::Shell::PowerShell, &mut cmd, cmd_name, term);
        }

        TerminalShell::Fish => {
            clap_complete::generate(clap_complete::Shell::Fish, &mut cmd, cmd_name, term);
        }

        TerminalShell::Zsh => {
            clap_complete::generate(clap_complete::Shell::Zsh, &mut cmd, cmd_name, term);
        }

        TerminalShell::Nushell => {
            clap_complete::generate(clap_complete_nushell::Nushell, &mut cmd, cmd_name, term);
        }
    }

    term.flush()
}

#[cfg(test)]
mod test_completion_command {
    use crate::config::{CompletionsCommandArguments, TerminalShell};

    #[test]
    fn it_should_write_shell_completions() {
        let shells = [
            TerminalShell::Bash,
            TerminalShell::Elvish,
            TerminalShell::Fish,
            TerminalShell::Nushell,
            TerminalShell::Powershell,
            TerminalShell::Zsh,
        ];

        for shell in shells {
            let args = CompletionsCommandArguments { shell };

            let mut buffer = Vec::new();

            super::completion_command(&mut buffer, &args).expect("it to not fail");

            assert!(!buffer.is_empty());
        }
    }
}
