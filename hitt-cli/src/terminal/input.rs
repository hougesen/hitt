use console::{Key, Term};

use super::{write_prompt, write_prompt_answer, TEXT_GREEN, TEXT_RESET};

pub(crate) fn text_input_prompt(
    term: &Term,
    prompt: &str,
    validator: fn(&str) -> bool,
    error_message: fn(&str) -> String,
) -> Result<String, std::io::Error> {
    let mut input = String::new();

    while !validator(&input) {
        let mut line_count = 0;

        write_prompt(term, prompt)?;
        line_count += 1;

        if !input.is_empty() {
            term.write_line(&error_message(&input))?;
            line_count += 1;
        }

        input = term.read_line()?.trim().to_string();

        line_count += 1;

        term.clear_last_lines(line_count)?;
    }

    write_prompt_answer(term, prompt, &input)?;

    Ok(input)
}

pub(crate) fn boolean_input(term: &Term, prompt: &str) -> Result<bool, std::io::Error> {
    loop {
        let mut line_count = 0;

        write_prompt(term, prompt)?;
        line_count += 1;

        let input = term.read_key()?;

        term.clear_last_lines(line_count)?;

        if input == Key::Char('y') || input == Key::Enter {
            write_prompt_answer(term, prompt, "y")?;
            return Ok(true);
        }

        if input == Key::Char('n') {
            write_prompt_answer(term, prompt, "n")?;
            return Ok(false);
        }
    }
}

pub(crate) fn select_input(
    term: &Term,
    prompt: &str,
    items: &[&str],
) -> Result<String, std::io::Error> {
    if items.len() < 2 {
        return Ok(items[0].to_string());
    }

    let mut selecting = true;
    let mut option_index = 0;

    while selecting {
        let mut line_count = 0;

        write_prompt(term, prompt)?;
        line_count += 1;

        for (item_index, item) in items.iter().enumerate() {
            if item_index == option_index {
                term.write_line(&format!("{TEXT_GREEN}> {item }{TEXT_RESET}"))?;
            } else {
                term.write_line(&format!("  {item }"))?;
            }
            line_count += 1;
        }

        let key = term.read_key()?;

        term.clear_last_lines(line_count)?;

        match key {
            Key::ArrowUp | Key::Char('k') => {
                option_index = if option_index == 0 {
                    items.len() - 1
                } else {
                    option_index - 1
                }
            }
            Key::ArrowDown | Key::Char('j') => {
                option_index = if option_index < items.len() - 1 {
                    option_index + 1
                } else {
                    0
                }
            }
            Key::Enter => selecting = false,
            _ => continue,
        }
    }

    let selected = items[option_index].to_string();

    write_prompt_answer(term, prompt, &selected)?;

    Ok(selected)
}
