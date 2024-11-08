use std::io;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::clear;

pub const ENGINES_FILE: &str = "~/.config/search_engines";

#[derive(Debug)]
pub struct SearchEngine {
    //{{{
    name: String, // spaces not allowed
    // TODO: add description field? at that point, json might be better
    url_fragment: String,
}

impl SearchEngine {
    pub fn from_str(line: &str) -> Result<Self, String> {
        let mut parts = line.split('\t').map(|s| s.to_string());
        let name = parts.next().ok_or(format!("line has no name: {line}"))?;
        let url_fragment = parts.next().ok_or(format!("line has no url: {line}"))?;
        Ok(Self { name, url_fragment })
    }

    pub fn build_url(
        &self,
        query: &str,
    ) -> Result<String, url::ParseError> {
        let url = match self.url_fragment.contains("%s") {
            true => self.url_fragment.replace("%s", query),
            false => self.url_fragment.to_string() + query,
        };

        let _ = url::Url::parse(&self.url_fragment);
        Ok(url.to_string())
    }
}
//}}}

/// Read hardcoded ENGINES_FILE file into string
pub fn read() -> Result<String, io::Error> {
    use std::fs::read_to_string;
    let file = shellexpand::tilde(ENGINES_FILE);
    read_to_string(file.to_string())
}

pub struct Engines {
    engines: Vec<SearchEngine>,
}
impl Engines {
    /// Parse each line to construct a vec of SearchEngines, ignoring commented
    /// and empty lines
    pub fn build() -> Result<Self, String> {
        let contents = match read() {
            Ok(c) => c,
            Err(_) => {
                println!("Does not exist: {}", ENGINES_FILE);
                std::process::exit(1);
            }
        };

        let engines = contents
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(SearchEngine::from_str)
            .filter_map(|e| e.ok())
            .collect();

        Ok(Engines { engines })
    }

    /// Match a SearchEngine by name (exactly), returning None if not found
    pub fn select(
        self,
        name: &str,
    ) -> Option<SearchEngine> {
        self.engines.into_iter().find(|engine| engine.name == name)
    }

    fn filter(
        &self,
        input: &str,
    ) -> Option<Vec<&str>> {
        let mut matched = self
            .engines
            .iter()
            .filter(|e| e.name.starts_with(input))
            .peekable();

        matched.peek()?; // thanks clippy
        Some(matched.map(|e| e.name.as_str()).collect::<Vec<&str>>())
    }

    pub fn fuzzy(&self) -> Option<String> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        let mut input = String::new();

        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    // note: `then` requires closure, where ? cannot be used
                    return (!input.trim().is_empty()).then_some({
                        clear(&mut stdout);
                        writeln!(stdout, "{}", termion::cursor::Show).unwrap();
                        self.filter(&input)?.first()?.to_string()
                    });
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

            if let Some(avail) = &self.filter(&input) {
                // TODO: highlight text
                let mut text = "Available engines: ".to_string();
                text.push_str(&avail.join(" "));
                writeln!(stdout, "{}{}", termion::cursor::Goto(1, 2), text).unwrap();
            }
            stdout.flush().unwrap();
        }

        None
    }
}
