use std::io::{Read, Write};

use console::{Key, Term};

use super::input::confirm_input;

#[inline]
fn get_default_editor() -> std::ffi::OsString {
    if let Some(prog) = std::env::var_os("VISUAL") {
        return prog;
    }

    if let Some(prog) = std::env::var_os("EDITOR") {
        return prog;
    }

    if cfg!(windows) {
        "notepad.exe".into()
    } else {
        "vi".into()
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

pub fn editor_input(
    term: &Term,
    content_type: Option<&str>,
) -> Result<Option<String>, std::io::Error> {
    let default_editor = get_default_editor().into_string().unwrap();

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

        let mut written_file = std::fs::File::open(path)?;
        let mut file_contents = String::new();
        written_file.read_to_string(&mut file_contents)?;

        return Ok(Some(file_contents));
    }
}
