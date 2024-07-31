use crate::logs::Log;
use clap::ArgMatches;
use std::fs;
use std::process::Command;

pub fn init(args: &ArgMatches) -> Result<(), Log> {
    let proj_name = args.get_one::<String>("project").unwrap();

    // FIX: check sanity of input argument

    // Make project directories
    fs::create_dir(proj_name).map_err(|e| Log::FileSystemError(e.to_string()))?;
    fs::create_dir(format!("{}/out", proj_name))
        .map_err(|e| Log::FileSystemError(e.to_string()))?;

    // Create main.tex file
    fs::File::create(format!("{}/main.tex", proj_name))
        .map_err(|e| Log::FileSystemError(e.to_string()))?;
    // TODO: write a template latex code to main.tex

    Ok(())
}
