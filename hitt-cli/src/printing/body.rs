#[inline]
fn format_json(input: &str) -> String {
    jsonformat::format(input, jsonformat::Indentation::TwoSpace)
}

#[inline]
fn print_pretty_json(input: &str) {
    let formatted_json = format_json(input);

    println!("{formatted_json}");
}

#[inline]
pub(crate) fn print_body(body: &str, content_type: Option<&str>, disable_pretty_printing: bool) {
    if disable_pretty_printing {
        println!("{body}");
        return;
    }

    match content_type {
        Some(content_type) => {
            if content_type.starts_with("application/json") {
                print_pretty_json(body);
            } else {
                println!("{body}");
            }
        }
        None => println!("{body}"),
    }
}
