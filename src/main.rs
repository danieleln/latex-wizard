mod cli;
mod config;
mod logs;
mod subcommands;

use logs::Log;
use std::process;

fn main() {
    let result = app();

    match result {
        Ok(()) => {}
        Err(Log::HelpMessage(msg)) => {
            println!("{}", msg);
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}

fn app() -> Result<(), logs::Log> {
    // Parse input arguments
    let args = cli::build_parser()
        .try_get_matches()
        .map_err(|e| clap::error::Error::from(e))?;

    // Run the requested subcommand
    match args.subcommand() {
        Some(("init", args)) => subcommands::init(&args),
        Some(("compile", args)) => subcommands::compile(&args),
        _ => unreachable!(),
    }
}
