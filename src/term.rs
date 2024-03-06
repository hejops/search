extern crate termion;

use std::io::stdin;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

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

/// Clear entire screen and place cursor at (1,1)
fn clear(stdout: &mut RawTerminal<Stdout>) {
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1), // column, row
    )
    .unwrap();
}

pub fn fuzzy() -> Option<String> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut input = String::new();
    let engines = Engines::new();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                // i do this weird double owning because returning static str seems sus to me,
                // but what do i know?
                clear(&mut stdout);
                let first = engines.filter(&input)?.first()?.to_owned();
                return Some(first.to_owned());
            }
            Key::Esc | Key::Char('q') => break,
            Key::Char(c) => input.push(c),
            Key::Backspace => {
                input.pop();
            }
            // Key::Left => println!("←"),
            // Key::Right => println!("→"),
            _ => {}
        }

        clear(&mut stdout);
        write!(stdout, "{}", input).unwrap();

        if let Some(avail) = engines.filter(&input) {
            // TODO: highlight text
            let mut text = "Available engines: ".to_string();
            text.push_str(&avail.join(" "));
            writeln!(stdout, "{}{}", termion::cursor::Goto(1, 2), text).unwrap();
        }
        stdout.flush().unwrap();
    }

    None
}
