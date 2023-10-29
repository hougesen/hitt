#[inline]
pub(crate) fn print_status(method: &str, url: &str, status_code: u16) {
    println!("{method} {url} {status_code}");
}
