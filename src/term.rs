extern crate termion;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// https://github.com/redox-os/termion/blob/master/examples/keys.rs

fn get_engines(input: &str) -> Option<String> {
    let engines = ["a", "bb", "amazon", "bandcamp", "discogs"];
    let avail: Vec<&str> = engines
        .iter()
        .filter(|e| e.starts_with(input))
        .copied()
        .collect();

    if avail.is_empty() {
        return None;
    }

    // TODO: highlight text
    let mut prefix = "Available engines: ".to_string();
    prefix.push_str(&avail.join(" "));
    Some(prefix)
}

pub fn fuzzy() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut input = String::new();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1), // column, row
        input,
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Esc | Key::Char('q') => break,
            Key::Char(c) => input.push(c),
            Key::Backspace => {
                input.pop();
            }
            // Key::Left => println!("←"),
            // Key::Right => println!("→"),
            _ => {}
        }
        writeln!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            // termion::clear::CurrentLine,
            input,
        )
        .unwrap();

        if let Some(prefix) = get_engines(&input) {
            writeln!(stdout, "{}{}", termion::cursor::Goto(1, 2), prefix).unwrap()
        }
        stdout.flush().unwrap();
    }
}
