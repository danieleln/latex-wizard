mod cli;
mod config;
mod logs;

use logs::Log;

fn main() {
    let result = app();
    match result {
        Err(log) => println!("{}", log),
        Ok(()) => {}
    }
}

fn app() -> Result<(), Log> {
    // Parse input arguments
    let args = cli::build_parser()
        .try_get_matches()
        .map_err(|e| clap::error::Error::from(e))?;

    // Run the requested subcommand
    match args.subcommand() {
        Some(("init", args)) => println!("init: {:?}", args),
        Some(("compile", args)) => println!("compile: {:?}", args),
        _ => unreachable!(),
    }

    Ok(())
}
