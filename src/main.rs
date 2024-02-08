use std::error::Error;

use search::engines;
use search::SearchArgs;
use search::ENGINES_FILE;

// TODO: docopt
// TODO: missing engine -> prompt / fuzzy
// TODO: missing query -> prompt
// TODO: search suggestions

fn main() -> Result<(), Box<dyn Error>> {
    let contents = match engines::read() {
        Ok(c) => c,
        Err(_) => {
            println!("Does not exist: {}", ENGINES_FILE);
            std::process::exit(1);
        }
    };

    let engines = engines::build(contents)?;

    let mut args = std::env::args();
    let args = SearchArgs::parse(&mut args, &engines).unwrap_or_else(|e| {
        search::print_usage(e);
        std::process::exit(1);
    });
    // println!("{:?}", args);

    let engine = engines::select(engines, &args.engine_name)
        .ok_or(format!("engine not found: {}", &args.engine_name))?;
    let url = engine.build_url(&args.query)?;
    search::launch(&url);
    Ok(())
}
