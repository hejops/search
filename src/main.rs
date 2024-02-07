use std::env::Args;
use std::error::Error;

use search::engine::*;
use search::ENGINES_FILE;

fn launch(url: &str) {
    // TODO: any vulnerabilities here?
    // TODO: opener crate
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

// TODO: docopt
// TODO: missing engine -> prompt / fuzzy
// TODO: missing query -> prompt

#[derive(Debug)]
struct SearchArgs {
    engine_name: String, // spaces not allowed
    query: String,
}

impl SearchArgs {
    fn parse(args: &mut Args) -> Result<SearchArgs, &str> {
        args.next();
        let engine_name = args.next().ok_or("No engine specified!")?;
        let query = args.next().ok_or("No query specified!")?;
        Ok(SearchArgs { engine_name, query })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = match read_engines() {
        Ok(c) => c,
        Err(_) => {
            println!("Does not exist: {}", ENGINES_FILE);
            std::process::exit(1);
        }
    };

    let mut args = std::env::args();
    let args = SearchArgs::parse(&mut args).unwrap_or_else(|e| {
        search::print_usage(e);
        std::process::exit(1);
    });

    let engines = build_engines(contents)?;
    let engine = select_engine(engines, &args.engine_name)
        .ok_or(format!("engine not found: {}", &args.engine_name))?;
    let url = engine.build_url(&args.query)?;
    launch(&url);
    Ok(())
}
