use crate::config::structure::{MAIN_FILE_NAME, MAIN_TEX_FILE, OUTPUT_DIRECTORY};
use crate::logs::{log_error, log_info, Log};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
