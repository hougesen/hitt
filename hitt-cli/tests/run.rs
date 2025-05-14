#[cfg(test)]
mod run_command {
    use predicates::prelude::PredicateBooleanExt;

    fn run_command() -> assert_cmd::Command {
        let mut command =
            assert_cmd::Command::cargo_bin("hitt").expect("error setting up hitt binary");

        command.arg("run");

        command
    }

    #[test]
    fn help_arg_outputs_message() {
        run_command()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }
}
