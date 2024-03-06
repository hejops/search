use std::error::Error;

use search::engines::Engines;
use search::SearchArgs;

// TODO: docopt
// TODO: missing query -> prompt
// TODO: search suggestions -- https://www.startpage.com/osuggestions?q=XXX

fn main() -> Result<(), Box<dyn Error>> {
    let engines = Engines::build()?;

    let mut args = std::env::args();
    let args = SearchArgs::parse(&mut args, &engines).unwrap_or_else(|e| {
        search::print_usage(e);
        std::process::exit(1);
    });

    let engine = Engines::select(engines, &args.engine_name)
        .ok_or(format!("engine not found: {}", &args.engine_name))?;
    let url = engine.build_url(&args.query)?;
    search::launch(&url);
    Ok(())
}
