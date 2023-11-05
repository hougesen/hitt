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

pub(crate) fn confirm_input(
    term: &Term,
    prompt: &str,
    default_value: Key,
) -> Result<bool, std::io::Error> {
    loop {
        let mut line_count = 0;

        write_prompt(term, prompt)?;
        line_count += 1;

        let input = term.read_key()?;

        term.clear_last_lines(line_count)?;

        if input == Key::Char('y')
            || input == Key::Char('Y')
            || (input == Key::Enter && default_value == Key::Char('y'))
        {
            write_prompt_answer(term, prompt, "y")?;
            return Ok(true);
        }

        if input == Key::Char('n')
            || input == Key::Char('N')
            || (input == Key::Enter && default_value == Key::Char('n'))
        {
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

/*
pub(crate) fn editor_input(term: &Term) -> Result<String, std::io::Error> {
    let mut input: Vec<Vec<char>> = vec![vec![]];

    loop {
        let mut line_count = 0;

        write_prompt(term, "Body input")?;
        line_count += 1;

        let input_len = input.len();

        for line in input.iter() {
            let formatted_line: String = line.iter().collect();

            term.write
            term.write_line(&formatted_line)?;
            line_count += 1;
        }

        let (x, y) = term.size();

        term.move_cursor_to(0, x as usize - 3)?;

        match term.read_key()? {
            Key::Unknown => todo!(),
            Key::UnknownEscSeq(_) => todo!(),

            Key::Enter => input.push(Vec::new()),
            Key::Escape => todo!(),
            Key::Backspace => {
                if input[input_len - 1].is_empty() {
                    if input_len > 1 {
                        input.pop();
                    }
                } else {
                    input[input_len - 1].pop();
                }
            }
            Key::Home => todo!(),
            Key::End => break,
            Key::Tab => todo!(),
            Key::BackTab => todo!(),
            Key::Alt => todo!(),
            Key::Del => todo!(),
            Key::Shift => todo!(),
            Key::Insert => todo!(),
            Key::ArrowLeft
            | Key::ArrowRight
            | Key::ArrowUp
            | Key::ArrowDown
            | Key::PageUp
            | Key::PageDown => continue,
            Key::Char(ch) => input[input_len - 1].push(ch),
            v => todo!(),
        };

        term.clear_last_lines(line_count)?;
    }

    let mut x = String::new();

    for line in input {
        for c in line {
            x.push(c);
        }
        x.push('\n');
    }

    Ok(x.trim().to_string())
}
*/
