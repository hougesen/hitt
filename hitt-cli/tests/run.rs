#[cfg(test)]
mod run_command {
    use std::io::Write;

    use predicates::prelude::PredicateBooleanExt;

    fn run_command(directory: Option<&std::path::Path>) -> assert_cmd::Command {
        let mut command =
            assert_cmd::Command::cargo_bin("hitt").expect("error setting up hitt binary");

        command.arg("run");

        if let Some(dir) = directory {
            command.current_dir(dir);
        }

        command
    }

    fn setup_test_input(dir: &std::path::Path, code: &str) -> tempfile::NamedTempFile {
        let mut b = tempfile::Builder::new();

        b.rand_bytes(12).suffix(".http");

        let mut f = b.tempfile_in(dir).unwrap();

        f.write_all(code.as_bytes()).unwrap();
        f.flush().unwrap();

        f
    }

    #[test]
    fn help_arg_outputs_message() {
        run_command(None)
            .arg("--help")
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }

    #[test]
    fn it_should_send_request() {
        let dir = tempfile::tempdir().unwrap();

        let method = "GET";
        let url = "https://api.goout.dk/";

        let input = format!("{method} {url}");

        let file = setup_test_input(dir.path(), &input);

        run_command(Some(dir.path()))
            .arg(file.path())
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not())
            .stdout(predicates::str::contains(format!(
                "HTTP/2.0 {method} {url} 200"
            )))
            .stdout(predicates::str::contains("Hello World!"));
    }

    #[test]
    fn with_hidden_body() {
        let dir = tempfile::tempdir().unwrap();

        let method = "GET";
        let url = "https://api.goout.dk/";

        let input = format!("{method} {url}");

        let file = setup_test_input(dir.path(), &input);

        run_command(Some(dir.path()))
            .arg("--hide-body")
            .arg(file.path())
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not())
            .stdout(predicates::str::contains(format!(
                "HTTP/2.0 {method} {url} 200"
            )))
            .stdout(predicates::str::contains("Hello World!").not());
    }

    #[test]
    fn with_timeout() {
        let dir = tempfile::tempdir().unwrap();

        let method = "GET";
        let url = "https://api.goout.dk/";

        let input = format!("{method} {url}");

        let file = setup_test_input(dir.path(), &input);

        run_command(Some(dir.path()))
            .arg("--timeout")
            .arg("0")
            .arg(file.path())
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not())
            .stdout(predicates::str::contains(format!(
                "{method} {url} - request timed out"
            )));
    }

    #[test]
    fn with_fail_fast() {
        let dir = tempfile::tempdir().unwrap();

        let url = "https://api.goout.dk/";
        let input = format!(
            "POST {url}

###

GET {url}"
        );

        let file = setup_test_input(dir.path(), &input);

        run_command(Some(dir.path()))
            .arg("--fail-fast")
            .arg(file.path())
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not())
            .stdout(predicates::str::contains(format!(
                "HTTP/2.0 POST {url} 404"
            )))
            .stdout(predicates::str::contains(
                "exiting early since --fail-fast is enabled",
            ))
            .stdout(predicates::str::contains(format!("GET {url}")).not());
    }

    #[test]
    fn it_should_support_multiple_input_paths() {
        let dir = tempfile::tempdir().unwrap();

        let method = "GET";
        let url = "https://api.goout.dk/";

        let input = format!("{method} {url}");

        let file1 = setup_test_input(dir.path(), &format!("{input}?1"));
        let file2 = setup_test_input(dir.path(), &format!("{input}?2"));

        run_command(Some(dir.path()))
            .arg(file1.path())
            .arg(file2.path())
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not())
            .stdout(predicates::str::contains(format!(
                "HTTP/2.0 {method} {url}?1 200"
            )))
            .stdout(predicates::str::contains("Hello World!"))
            .stdout(predicates::str::contains(format!(
                "HTTP/2.0 {method} {url}?2 200"
            )));
    }
}
