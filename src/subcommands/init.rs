use crate::logs::{log_error, Log};
use clap::ArgMatches;
use std::fs;
use std::process::Command;

pub fn init(args: &ArgMatches) -> Result<(), Log> {
    let proj_name = args.get_one::<String>("project").unwrap();

    // FIX: check sanity of input argument

    // Make project directories
    fs::create_dir(proj_name).map_err(|e| {
        Log::FileSystemError(format!(
            "While creating directory {}:\n{}",
            proj_name,
            e.to_string()
        ))
    })?;
    fs::create_dir(format!("{}/out", proj_name)).map_err(|e| {
        Log::FileSystemError(format!(
            "While creating directory {}/out:\n{}",
            proj_name,
            e.to_string()
        ))
    })?;

    // Create main.tex file
    fs::File::create(format!("{}/main.tex", proj_name)).map_err(|e| {
        Log::FileSystemError(format!(
            "While creating file {}/main.tex:\n{}",
            proj_name,
            e.to_string()
        ))
    })?;
    // TODO: write a template latex code to main.tex

    // Run "git init" command
    let output = Command::new("git")
        .arg("init")
        .arg(proj_name)
        .output()
        .map_err(|e| {
            Log::FileSystemError(format!(
                "While running `git init {}`:\n{}",
                proj_name,
                e.to_string()
            ))
        })?;

    if !output.status.success() {
        log_error(format!(
            "An error occurred while running `git init {}`",
            proj_name
        ));
    }

    Ok(())
}
