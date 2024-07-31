mod cli;
mod config;
mod logs;
mod subcommands;

fn main() {
    let result = app();
    match result {
        Err(log) => println!("{}", log),
        Ok(()) => {}
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
