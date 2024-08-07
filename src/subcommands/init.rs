use crate::config::structure::{MAIN_TEX_FILE, OUTPUT_DIRECTORY};
use crate::logs::Log;
use clap::ArgMatches;
use std::fs;
use std::io::Write;
use std::process::Command;

pub fn init(args: &ArgMatches) -> Result<(), Log> {
    let proj_name = args.get_one::<String>("project").unwrap();

    // FIX: check sanity of input argument

    // Make project directories
    fs::create_dir(proj_name).map_err(|e| {
        Log::FileSystemError(format!(
            "While creating directory `{}`:\n{}",
            proj_name,
            e.to_string()
        ))
    })?;
    fs::create_dir(format!("{}/{}", proj_name, OUTPUT_DIRECTORY)).map_err(|e| {
        Log::FileSystemError(format!(
            "While creating directory `{}/{}`:\n{}",
            proj_name,
            OUTPUT_DIRECTORY,
            e.to_string()
        ))
    })?;

    // Create the main `.tex` file
    fs::File::create(format!("{}/{}", proj_name, MAIN_TEX_FILE)).map_err(|e| {
        Log::FileSystemError(format!(
            "While creating file `{}/{}`:\n{}",
            proj_name,
            MAIN_TEX_FILE,
            e.to_string()
        ))
    })?;
    // TODO: write a template latex code to the main `.tex` file

    // Create a .gitignore file
    let mut gitignore =
        fs::File::create(format!("{}/{}", proj_name, ".gitignore")).map_err(|e| {
            Log::FileSystemError(format!(
                "While creating file `{}/{}`:\n{}",
                proj_name,
                ".gitignore",
                e.to_string()
            ))
        })?;

    // Copy the template .gitignore file
    const GITIGNORE_TEMPLATE: &str = include_str!("../templates/gitignore");
    gitignore
        .write_all(GITIGNORE_TEMPLATE.as_bytes())
        .map_err(|e| {
            Log::FileSystemError(format!(
                "While writing to `{}/{}`:\n{}",
                proj_name,
                ".gitignore",
                e.to_string()
            ))
        })?;

    // Run "git init" command
    let output = Command::new("git")
        .arg("init")
        .arg(proj_name)
        .output()
        .map_err(|e| {
            Log::ShellCommandError(format!(
                "While initializing a git repo in `{}`:\n{}",
                proj_name,
                e.to_string()
            ))
        })?;

    if !output.status.success() {
        return Err(Log::ShellCommandError(format!(
            "An error occurred while initializing a git repo in `{}`.",
            proj_name
        )));
    }

    Ok(())
}
