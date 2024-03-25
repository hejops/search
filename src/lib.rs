//! Simple command-line tool to perform web searches
//!
//! Usage:
//! ```sh
//! search [engine] [query]
//! ```
//!
//! A file at `~/.config/search_engines` is required. This file must be
//! tab-separated. Empty lines and commented lines (beginning with `#`) are
//! ignored. All other lines must possess exactly two fields, containing the
//! name of a search engine, and a base URL to which a query will be appended:
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

// extern crate termion;

pub mod engines;
use std::env::Args;
use std::error::Error;
use std::io;
use std::io::Stdout;
use std::io::Write;
use std::ops::Not;

use termion::raw::RawTerminal;

use crate::engines::Engines;

pub fn print_usage(err: &str) {
    println!(
        "\
{err}

Usage:
    search <engine> <query>
",
    );
    // std::process::exit(1);
}

#[derive(Debug)]
pub struct SearchArgs {
    //{{{
    // TODO: make fields private?
    pub engine_name: String, // spaces not allowed
    pub query: String,
}

impl SearchArgs {
    pub fn parse<'a>(
        args: &'a mut Args,
        engines: &Engines,
    ) -> Result<SearchArgs, &'a str> {
        args.next();

        let engine_name = match args.next() {
            Some(n) => n,
            None => engines.fuzzy().ok_or("No valid engine specified")?,
        };

        let query = match args.next() {
            Some(q) => q,
            None => get_input("query").ok_or("No valid query specified")?,
        };

        Ok(SearchArgs { engine_name, query })
    }
}

//}}}

pub fn launch(url: &str) -> Result<(), Box<dyn Error>> {
    std::process::Command::new("xdg-open")
        .arg(url)
        .spawn()?
        .wait()?;
    Ok(())
}

/// Read a single line of user input, returning None if input was empty
pub fn get_input(v: &str) -> Option<String> {
    println!("Specify {v}: ");
    let mut result = String::new();
    let _ = io::stdin().read_line(&mut result);
    result = result.trim().to_string();
    result.is_empty().not().then_some(result)
}

// https://github.com/redox-os/termion/blob/master/examples/keys.rs

/// Clear entire screen and place cursor at (1,1)
fn clear(stdout: &mut RawTerminal<Stdout>) {
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1), // column, row
        termion::cursor::Hide,
    )
    .unwrap();
}
