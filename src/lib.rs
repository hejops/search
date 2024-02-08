//! Simple command-line tool to perform web searches
//!
//! Usage:
//! ```sh
//! search <engine> <query>
//! ```
//!
//! A file at `~/.config/search_engines` is required. This file must be tab-separated. Empty lines
//! and commented lines (beginning with `#`) are ignored. All other lines must possess exactly two
//! fields, containing the name of a search engine, and a base URL to which a query will be
//! appended:
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
//! `xdg-open` will be called on the resulting URL.

use std::env::Args;
use std::io;
use std::ops::Not;

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

#[derive(Debug)]
pub struct SearchArgs {
    //{{{
    pub engine_name: String, // spaces not allowed
    pub query: String,
}

impl SearchArgs {
    pub fn parse<'a>(
        args: &'a mut Args,
        engines: &'a [SearchEngine],
    ) -> Result<SearchArgs, &'a str> {
        args.next();

        let engine_name = match args.next() {
            Some(n) => n,
            None => engines::list(engines).ok_or("No valid engine specified")?,
        };

        let query = match args.next() {
            Some(q) => q,
            None => get_input("query").ok_or("No valid query specified")?,
        };

        Ok(SearchArgs { engine_name, query })
    }
}
//}}}

#[derive(Debug)]
pub struct SearchEngine {
    //{{{
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
        // print!("{}", url);

        let _ = url::Url::parse(&self.url);
        Ok(url.to_string())
    }
}
//}}}

pub fn get_input(v: &str) -> Option<String> {
    println!("Specify {v}: ");
    let mut result = String::new();
    let _ = io::stdin().read_line(&mut result);
    result.trim().is_empty().not().then_some(result)
}

pub fn launch(url: &str) {
    // TODO: any vulnerabilities here?
    // TODO: opener crate
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

pub mod engines {

    use crate::get_input;
    use crate::SearchEngine;
    use std::fs::read_to_string;
    use std::io;

    /// read hardcoded ENGINES_FILE file into string
    pub fn read() -> Result<String, io::Error> {
        let file = shellexpand::tilde(crate::ENGINES_FILE);
        read_to_string(file.to_string())
    }

    /// parse each line to construct a vec of SearchEngines, ignoring commented and empty lines
    pub fn build(contents: String) -> Result<Vec<SearchEngine>, String> {
        let mut engines = vec![];

        for line in contents
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
        {
            let mut parts = line.split('\t').map(|s| s.to_string());
            let name = parts.next().ok_or(format!("line has no name: {line}"))?;
            let url = parts.next().ok_or(format!("line has no url: {line}"))?;
            engines.push(SearchEngine { name, url });
        }

        // dbg!("{:#?}", engines);
        // panic!();

        Ok(engines)
    }

    /// match a SearchEngine by name, returning None if not found
    pub fn select(engines: Vec<SearchEngine>, name: &str) -> Option<SearchEngine> {
        engines.into_iter().find(|engine| engine.name == name)
    }

    pub fn list(engines: &[SearchEngine]) -> Option<String> {
        // TODO: simple fuzzy implementation (engine only): read chars (like python readchar) and
        // accumulate into a string, allow backspace
        // for each keypress, select engines that start with (or contain) the string, join them
        // with space may require screen clear, or some overlay

        for e in engines.iter().map(|e| &e.name) {
            print!("{} ", e);
        }
        println!();
        get_input("engine")
    }
}
