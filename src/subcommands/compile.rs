use crate::config::structure::MAIN_TEX_FILE;
use crate::logs::{log_info, Log};
use clap::ArgMatches;
use std::env;
use std::path::{Path, PathBuf};

pub fn compile(args: &ArgMatches) -> Result<(), Log> {
    let proj_name = args.get_one::<String>("project");

    // Look for the main `.tex` file to compile
    let main_tex_file = find_main_tex_file(&proj_name)?;
    log_info(format!("Compiling `{}`", main_tex_file.display()));

    Ok(())
}

pub fn find_main_tex_file(proj_name: &Option<&String>) -> Result<PathBuf, Log> {
    // Look for the main `.tex` file to be compiled
    match proj_name {

        // Check if proj_name is the main `.tex` file or if it's a
        // directory containing it
        Some(proj_name) => {
            let proj_name = Path::new(&proj_name);

            if !proj_name.exists() {
                Err(format!("File or directory `{}` doesn't exist.", proj_name.display()))
            }
            else if proj_name.is_file() {
                if proj_name.file_name() == Some(MAIN_TEX_FILE.as_ref()) {
                    Ok(proj_name.to_path_buf())
                }
                else if proj_name.file_name() == None {
                    Err(format!("Failed to retrieve the file name of `{}`", proj_name.display()))
                }
                else {
                    Err(format!(
                        "Invalid file name `{}`. Only the main `.tex` file (`{}`) can be compiled.",
                        proj_name.display(),
                        MAIN_TEX_FILE
                    ))
                }
            }
            else if proj_name.is_dir() {
                let current_directory = proj_name.to_path_buf();
                let candidate_main_tex_file = current_directory.join(MAIN_TEX_FILE);

                if candidate_main_tex_file.exists() && candidate_main_tex_file.is_file() {
                    Ok(candidate_main_tex_file)
                } else {
                    Err(format!(
                        "Failed to find the main `.tex` file (`{}`) inside `{}`.",
                        MAIN_TEX_FILE,
                        current_directory.display()
                    ))
                }
            }
            else {
                Err(format!("none"))
            }
        }

        // Check from the current directory upward until either
        // reaching the root directory or finding a main.tex file
        None => {
            let current_directory = env::current_dir().map_err(|e| {
                Log::FileSystemError(format!(
                    "While looking for `{}` file: failed to get current directory.\n{}",
                    MAIN_TEX_FILE,
                    e.to_string()
                ))
            })?;

            // Look for the main `.tex` file
            let mut current_directory = current_directory.to_path_buf();
            loop {
                let candidate_main_tex_file = current_directory.join(MAIN_TEX_FILE);

                if candidate_main_tex_file.exists() && candidate_main_tex_file.is_file() {
                    break Ok(candidate_main_tex_file);
                }

                // Iterate all parent directories until reaching the root
                if !current_directory.pop() {
                    break Err(format!(
                        "Can't find file `{}` neither in the current directory, nor in any parent directory.",
                        MAIN_TEX_FILE
                    ));
                }
            }
        }
    }.map_err(|e| Log::InvalidCommandLineArgument(e))
}
