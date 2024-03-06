extern crate termion;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// https://github.com/redox-os/termion/blob/master/examples/keys.rs

struct Engines {
    engines: Vec<String>,
}
impl Engines {
    pub fn new() -> Self {
        let mut engines = vec![];
        for e in ["a", "bb", "amazon", "bandcamp", "discogs"] {
            engines.push(String::from(e))
        }
        Engines { engines }
    }
    pub fn filter(
        &self,
        input: &str,
    ) -> Option<Vec<&str>> {
        let mut avail = self
            .engines
            .iter()
            .filter(|e| e.starts_with(input))
            .peekable();

        avail.peek()?; // thanks clippy
        Some(avail.map(|x| x.as_str()).collect::<Vec<&str>>())
    }
}

pub fn fuzzy() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut input = String::new();
    let engines = Engines::new();

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

        if let Some(avail) = engines.filter(&input) {
            // TODO: highlight text
            let mut text = "Available engines: ".to_string();
            text.push_str(&avail.join(" "));
            writeln!(stdout, "{}{}", termion::cursor::Goto(1, 2), text).unwrap();
        }
        stdout.flush().unwrap();
    }
}
