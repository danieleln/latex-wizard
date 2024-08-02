use crate::config::structure::{MAIN_FILE_NAME, MAIN_PDF_FILE, MAIN_TEX_FILE, OUTPUT_DIRECTORY};
use crate::logs::{log_error, log_info, log_warning, Log};
use clap::ArgMatches;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;

pub fn compile(args: &ArgMatches) -> Result<(), Log> {
    // Find the project to compile
    let proj_name = args.get_one::<String>("project");

    // Look for the root directory of the project (the directory that
    // contains the main `.tex` file
    let proj_dir = find_project_root_directory(&proj_name)?;

    // Clean the output directory if required
    let clean_flag = args.get_one::<bool>("clean");
    if clean_flag == Some(&true) {
        // Find the output directory and the output `.pdf` file
        let out_dir = proj_dir.join(OUTPUT_DIRECTORY);
        let out_pdf = out_dir.join(MAIN_PDF_FILE);

        let result = clean_output_directory(&out_dir, &out_pdf);
        if let Err(e) = result {
            println!("{}", e.to_string());
        }
    }

    // Finally, compile the project
    compile_tex_file(&proj_dir)
}

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

// Compile a tex file
pub fn compile_tex_file(project_directory: &PathBuf) -> Result<(), Log> {
    let output_directory = project_directory.join(OUTPUT_DIRECTORY);

    // Create the output directory if it's missing
    if !output_directory.exists() {
        fs::create_dir(&output_directory).map_err(|e| {
            Log::FileSystemError(format!(
                "While creating output directory `{}`:\n{}",
                output_directory.display(),
                e.to_string()
            ))
        })?;
    }

    // Log some infos to the user
    log_info(format!(
        "Compiling `{}/{}` to `{}`",
        project_directory.display(),
        MAIN_TEX_FILE,
        output_directory.display()
    ));

    // Change the current working directory to the base directory of
    // the project. Some LaTeX statements (like `\include{...}`)
    // fail to resolve if the working directory is different from
    // where the file being compiled is.
    let result = env::set_current_dir(project_directory);
    if let Err(e) = result {
        log_error(format!(
            "Failed to set the current working directory to `{}`. Some LaTeX statements (like `\\include`) might fail to compile.\n{}",
            project_directory.display(),
            e.to_string()
        ));
    }

    // Compilation process:
    // 1. run `pdflatex` a first time
    // 2. compile glossaries using `makeglossaries` command
    // 3. compile biblography using the `biber` command
    // 4. run `pdflatex` a second time

    // 1. Run pdflatex
    run_shell_cmd(
        Command::new("pdflatex")
            // NOTE: flags have only one hypen according to `man pdflatex`!
            .arg("-halt-on-error")
            .arg("-output-directory")
            .arg(OUTPUT_DIRECTORY)
            .arg(MAIN_TEX_FILE),
    )?;

    // 2. Run makeglossaries
    run_shell_cmd(
        Command::new("makeglossaries")
            .arg("-d")
            .arg(OUTPUT_DIRECTORY)
            .arg(MAIN_FILE_NAME),
    )?;
    // Run biber
    run_shell_cmd(
        Command::new("biber")
            .arg("--input-directory")
            .arg(OUTPUT_DIRECTORY)
            .arg("--output-directory")
            .arg(OUTPUT_DIRECTORY)
            .arg(MAIN_FILE_NAME),
    )?;

    // 4. Run pdflatex
    run_shell_cmd(
        Command::new("pdflatex")
            .arg("-halt-on-error")
            .arg("-output-directory")
            .arg(OUTPUT_DIRECTORY)
            .arg(MAIN_TEX_FILE),
    )?;

    Ok(())
}

fn run_shell_cmd(command: &mut Command) -> Result<(), Log> {
    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            Log::ShellCommandError(format!(
                "Failed to spawn command `pdflatex`:\n{}",
                e.to_string()
            ))
        })?;

    if !output.status.success() {
        return Err(Log::ShellCommandError(format!(
            "Command `{}` failed with exit code {}.\n{}\n{}",
            command.get_program().to_str().unwrap(),
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(())
}

fn clean_output_directory(output_directory: &PathBuf, output_pdf: &PathBuf) -> Result<(), Log> {
    // Just keep the file name of the output `.pdf` file
    let output_pdf = output_pdf.file_name().ok_or_else(|| {
        Log::FileSystemError(format!("Invalid file name `{}`", output_pdf.display()))
    })?;
    // Useful for later
    let output_pdf = Some(output_pdf);

    if !output_directory.exists() {
        log_info(format!(
            "The output directory `{}` is missing already. No file was removed.",
            output_directory.display(),
        ));
        return Ok(());
    }

    // Select files to be removed
    let files_to_remove: Vec<PathBuf> = fs::read_dir(output_directory)
        .map_err(|e| {
            Log::FileSystemError(format!(
                "An error occurred while reading the content of directory `{}`:\n{}",
                output_directory.display(),
                e.to_string(),
            ))
        })?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        // Remove any file in the output directory except for the output
        // `.pdf` file (in case compilation goes wrong, at least the
        // output `.pdf` is preserved)
        .filter(|path| !path.is_file() || !(path.file_name() == output_pdf))
        .collect();

    // Remove the selected files
    for file in files_to_remove {
        log_warning(format!("Removing `{}`", &file.display()));
        let result = fs::remove_file(&file);
        if let Err(e) = result {
            log_error(format!(
                "An error occurred while removing file `{}`:\n{}",
                file.display(),
                e.to_string()
            ));
        }
    }

    Ok(())
}
