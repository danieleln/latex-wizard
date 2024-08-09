use crate::config::structure::MAIN_TEX_FILE;
use crate::logs::Log;
use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn find_project_root_directory(proj_name: &Option<&String>) -> Result<PathBuf, Log> {
    // Look for the main `.tex` file to be compiled
    match proj_name {
        // Check if proj_name is the main `.tex` file or if it's a
        // directory containing it
        Some(proj_name) => {
            let proj_name = Path::new(&proj_name).to_path_buf();

            if !proj_name.exists() {
                Err(Log::InvalidCommandLineArgument(format!(
                    "File or directory `{}` doesn't exist.",
                    proj_name.display()
                )))
            }
            // Check if proj_name is the main `.tex` file
            else if proj_name.is_file() {
                if proj_name.file_name() == Some(MAIN_TEX_FILE.as_ref()) {
                    let root_dir = proj_name.parent().ok_or(Log::FileSystemError(format!(
                        "Failed to get the directory containing `{}`",
                        proj_name.display(),
                    )))?;

                    // NOTE: when running `latex-wizard compile main.tex`,
                    //       PathBuf::from("main.tex").parent() evals
                    //       to Some("") rather than Some(".")
                    if root_dir.as_os_str().is_empty() {
                        Ok(PathBuf::from_str(".").unwrap())
                    } else {
                        Ok(root_dir.to_path_buf())
                    }
                } else if proj_name.file_name() == None {
                    Err(Log::FileSystemError(format!(
                        "Failed to retrieve the file name of `{}`",
                        proj_name.display()
                    )))
                } else {
                    Err(Log::InvalidCommandLineArgument(format!(
                        "Invalid file name `{}`. Only the main `.tex` file (`{}`) can be compiled.",
                        proj_name.display(),
                        MAIN_TEX_FILE
                    )))
                }
            }
            // Check if proj_name is the root directory of the project
            else if proj_name.is_dir() {
                let candidate_main_tex_file = proj_name.join(MAIN_TEX_FILE);

                if candidate_main_tex_file.exists() && candidate_main_tex_file.is_file() {
                    Ok(proj_name)
                } else {
                    Err(Log::InvalidCommandLineArgument(format!(
                        "Failed to find the main `.tex` file (`{}`) inside `{}`.",
                        MAIN_TEX_FILE,
                        proj_name.display()
                    )))
                }
            }
            // Invalid file
            else {
                Err(Log::InvalidCommandLineArgument(format!(
                    "File `{}` should be either the root directory of the project or the main `.tex` file (`{}`)",
                    proj_name.display(),
                    MAIN_TEX_FILE
                )))
            }
        }

        // Check if the current directory is the root directory of
        // the project. If not, check all the parent directories,
        // up to the root of the system "/"
        None => {
            let mut current_directory = env::current_dir()
                .map_err(|e| {
                    Log::FileSystemError(format!(
                        "While looking for `{}` file: failed to get current directory.\n{}",
                        MAIN_TEX_FILE,
                        e.to_string()
                    ))
                })?
                .to_path_buf();

            // Look for the main `.tex` file
            loop {
                let candidate_main_tex_file = current_directory.join(MAIN_TEX_FILE);

                if candidate_main_tex_file.exists() && candidate_main_tex_file.is_file() {
                    break Ok(current_directory);
                }

                // Iterate all parent directories until reaching the root
                if !current_directory.pop() {
                    break Err(Log::InvalidCommandLineArgument(format!(
                        "Can't find file `{}` neither in the current directory, nor in any parent directory.",
                        MAIN_TEX_FILE
                    )));
                }
            }
        }
    }
}
