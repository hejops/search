//! Simple command-line tool to perform web searches on the command-line
//!
//! Usage:
//! ```sh
//! search <engine> <query>
//! ```
//!
//! A file at `~/.config/search_engines` is required. This file must be tab-separated. Empty lines
//! and commented lines (beginning with `#`) are ignored. All other lines must possess exactly two
//! fields, containing the name of a search engine, and a base URL to which a query will be
//! appended to:
//!
//! ```text
//! ddg  https://duckduckgo.com/?t=ffab&q=
//! ```
//!
//! One printf substitution is allowed:
//!
//! ```text
//! rust_errors    https://doc.rust-lang.org/error_codes/E%s.html
//! ```
//!
//! `xdg-open` will be called on resulting URL.

pub const ENGINES_FILE: &str = "~/.config/search_engines";

pub fn print_usage(err: &str) {
    println!(
        "\
{err}

Usage:
    search <engine> <query>

Requires: {ENGINES_FILE}\
",
    );
    // std::process::exit(1);
}

pub mod engine {

    use std::fs::read_to_string;
    use std::io;

    /// read hardcoded ENGINES_FILE file into string
    pub fn read_engines() -> Result<String, io::Error> {
        let file = shellexpand::tilde(crate::ENGINES_FILE);
        read_to_string(file.to_string())
    }

    #[derive(Debug)]
    pub struct SearchEngine {
        name: String, // spaces not allowed
        // TODO: add description field? at that point, json might be better
        url: String,
    }

    impl SearchEngine {
        pub fn build_url(&self, query: &str) -> Result<String, url::ParseError> {
            // let query = urlencoding::encode(query);
            let url = match self.url.contains("%s") {
                true => self.url.replace("%s", query),
                false => self.url.to_string() + query,
            };
            print!("{}", url);

            let _ = url::Url::parse(&self.url);
            Ok(url.to_string())
        }
    }

    /// parse each line to construct a vec of SearchEngines, ignoring commented and empty lines
    pub fn build_engines(contents: String) -> Result<Vec<SearchEngine>, String> {
        let mut engines = vec![];

        for line in contents.lines() {
            let mut line_iter = line
                .split('\t')
                .map(|s| s.trim().to_string())
                .filter(|line| !line.is_empty() && !line.starts_with('#'));
            let name = match line_iter.next() {
                Some(name) => name,
                None => continue,
            };
            let url = match line_iter.next() {
                Some(url) => url,
                None => continue,
            };

            let engine = SearchEngine { name, url };

            engines.push(engine)
        }

        // dbg!("{:#?}", engines);
        // panic!();

        Ok(engines)
    }

    /// match a SearchEngine by name, returning None if not found
    pub fn select_engine(engines: Vec<SearchEngine>, name: &str) -> Option<SearchEngine> {
        engines.into_iter().find(|engine| engine.name == name)
    }
}
