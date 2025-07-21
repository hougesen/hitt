#[cfg(test)]
mod sse_command {
    fn sse_command(directory: Option<&std::path::Path>) -> assert_cmd::Command {
        let mut command =
            assert_cmd::Command::cargo_bin("hitt").expect("error setting up hitt binary");

        command.arg("sse");

        if let Some(dir) = directory {
            command.current_dir(dir);
        }

        command
    }

    #[test]
    fn it_should_reject_invalid_urls() {
        let url = "thisisnotaurl";

        sse_command(None)
            .arg(url)
            .assert()
            .success()
            .stdout(predicates::str::contains(format!(
                "hitt: '{url}' is not a valid url"
            )));
    }
}
