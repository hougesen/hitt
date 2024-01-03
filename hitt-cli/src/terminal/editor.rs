use std::{ffi::OsString, io::Write};

use console::{Key, Term};

use super::input::confirm_input;

#[inline]
fn get_default_editor() -> std::ffi::OsString {
    if let Ok(prog) = std::env::var("VISUAL") {
        return OsString::from(prog);
    }

    if let Ok(prog) = std::env::var("EDITOR") {
        return OsString::from(prog);
    }

    #[cfg(windows)]
    {
        "notepad.exe".into()
    }
    #[cfg(not(windows))]
    {
        "vi".into()
    }
}

#[cfg(test)]
mod test_get_default_editor {
    use super::get_default_editor;

    #[test]
    fn test_default_values() {
        std::env::remove_var("EDITOR");
        std::env::remove_var("VISUAL");

        #[cfg(windows)]
        {
            assert_eq!("notepad.exe", get_default_editor());
        }
        #[cfg(not(windows))]
        {
            assert_eq!("vi", get_default_editor());
        }
    }

    #[test]
    fn test_editor_env_works() {
        std::env::remove_var("EDITOR");
        std::env::remove_var("VISUAL");

        std::env::set_var("EDITOR", "vim");

        assert_eq!("vim", get_default_editor());

        std::env::remove_var("EDITOR");
    }

    #[test]
    fn test_visual_env_works() {
        std::env::remove_var("EDITOR");
        std::env::remove_var("VISUAL");

        std::env::set_var("VISUAL", "nvim");

        assert_eq!("nvim", get_default_editor());

        std::env::remove_var("VISUAL");
    }
}

#[inline]
fn open_editor(
    cmd: &str,
    args: &[String],
    path: &std::path::Path,
) -> Result<std::process::ExitStatus, std::io::Error> {
    std::process::Command::new(cmd)
        .args(args)
        .arg(path)
        .spawn()?
        .wait()
}

#[inline]
fn create_temp_file(ext: &str) -> Result<tempfile::NamedTempFile, std::io::Error> {
    tempfile::Builder::new()
        .prefix("hitt-")
        .suffix(ext)
        .rand_bytes(12)
        .tempfile()
}

#[cfg(test)]
mod test_create_temp_file {
    use super::create_temp_file;

    #[test]
    fn it_should_return_a_file() {
        let file = create_temp_file(".http").expect("it to not throw an error");

        assert!(file.path().exists());
    }
}

#[inline]
fn build_editor_cmd(editor_cmd: String) -> (String, Vec<String>) {
    shell_words::split(&editor_cmd).map_or_else(
        |_| (editor_cmd, Vec::new()),
        |mut parts| {
            let cmd = parts.remove(0);
            (cmd, parts)
        },
    )
}

#[cfg(test)]
mod test_build_editor_cmd {
    use crate::terminal::editor::build_editor_cmd;

    #[test]
    fn it_should_return_command() {
        {
            let cmd = "nvim";

            assert_eq!(
                (cmd.to_owned(), Vec::new()),
                build_editor_cmd(cmd.to_owned())
            );
        };

        {
            let cmd = "nvim --mads --was --here";

            assert_eq!(
                (
                    "nvim".to_owned(),
                    vec!["--mads".to_owned(), "--was".to_owned(), "--here".to_owned()]
                ),
                build_editor_cmd(cmd.to_owned())
            );
        };
    }
}

#[inline]
fn content_type_to_ext(content_type: Option<&str>) -> &'static str {
    match content_type {
        Some("application/json") => ".json",
        Some("text/css") => ".css",
        Some("text/csv") => ".csv",
        Some("text/html") => ".html",
        Some("text/javascript") => ".js",
        Some("application/ld+json") => ".jsonld",
        Some("application/x-httpd-php") => ".php",
        Some("application/x-sh") => ".sh",
        Some("image/svg+xml") => ".svg",
        Some("application/xml" | "text/xml") => ".xml",
        _ => ".txt",
    }
}

#[cfg(test)]
mod test_content_type_to_ext {
    use crate::terminal::editor::content_type_to_ext;

    #[test]
    fn application_json() {
        assert_eq!(".json", content_type_to_ext(Some("application/json")));
    }

    #[test]
    fn text_css() {
        assert_eq!(".css", content_type_to_ext(Some("text/css")));
    }

    #[test]
    fn text_csv() {
        assert_eq!(".csv", content_type_to_ext(Some("text/csv")));
    }

    #[test]
    fn text_html() {
        assert_eq!(".html", content_type_to_ext(Some("text/html")));
    }

    #[test]
    fn text_javascript() {
        assert_eq!(".js", content_type_to_ext(Some("text/javascript")));
    }

    #[test]
    fn application_ld_json() {
        assert_eq!(".jsonld", content_type_to_ext(Some("application/ld+json")));
    }

    #[test]
    fn application_x_httpd_php() {
        assert_eq!(".php", content_type_to_ext(Some("application/x-httpd-php")));
    }

    #[test]
    fn application_x_sh() {
        assert_eq!(".sh", content_type_to_ext(Some("application/x-sh")));
    }

    #[test]
    fn image_svg_xml() {
        assert_eq!(".svg", content_type_to_ext(Some("image/svg+xml")));
    }

    #[test]
    fn application_xml() {
        assert_eq!(".xml", content_type_to_ext(Some("application/xml")));
    }

    #[test]
    fn text_xml() {
        assert_eq!(".xml", content_type_to_ext(Some("text/xml")));
    }

    #[test]
    fn unknown_content_type() {
        for i in u8::MIN..u8::MAX {
            assert_eq!(".txt", content_type_to_ext(Some(&i.to_string())));
        }
    }
}

pub fn editor_input(
    term: &Term,
    content_type: Option<&str>,
) -> Result<Option<String>, std::io::Error> {
    let default_editor = get_default_editor().to_string_lossy().to_string();

    let mut file = create_temp_file(content_type_to_ext(content_type))?;

    file.flush()?;

    let path = file.path();

    let ts = std::fs::metadata(path)?.modified()?;

    let (cmd, args) = build_editor_cmd(default_editor);

    let lower_y_key = Key::Char('y');

    loop {
        let status = open_editor(&cmd, &args, path)?;

        if status.success() && ts >= std::fs::metadata(path)?.modified()? {
            let confirm_close = confirm_input(
                term,
                "The body was not set, did you exit on purpose? (Y/n)",
                &lower_y_key,
            )?;

            if confirm_close {
                return Ok(None);
            }

            continue;
        }

        return std::fs::read_to_string(path).map(Some);
    }
}
