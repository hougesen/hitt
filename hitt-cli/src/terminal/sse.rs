use crossterm::{
    execute,
    style::{Print, Stylize},
};

#[inline]
pub fn print_sse_connection_open<W: std::io::Write + Send>(
    term: &mut W,
    url: &str,
) -> std::io::Result<()> {
    execute!(term, Print(format!("hitt: connected to {url}\n").cyan()))
}
