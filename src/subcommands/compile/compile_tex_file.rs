use crate::config::structure::{MAIN_FILE_NAME, MAIN_PDF_FILE, MAIN_TEX_FILE, OUTPUT_DIRECTORY};
use crate::logs::{log_error, log_info, Log};
use regex::Regex;
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
        "Compiling `{}` to `{}`.",
        project_directory.join(MAIN_TEX_FILE).display(),
        output_directory.join(MAIN_PDF_FILE).display()
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
    // 1. Run `pdflatex` a first time.
    // 2. When required, compile glossaries using `makeglossaries`.
    // 3. When required, compile biblography using `biber`.
    // 4. Run `pdflatex` a second time when either `makeglossaries` or
    //      `biber` did run.

    // Read the main `tex` file and check whether there is a glossary
    // and/or a bibliography
    let main_tex_file = project_directory.join(MAIN_TEX_FILE);
    let content = fs::read_to_string(&main_tex_file).map_err(|e| {
        Log::FileSystemError(format!(
            "Failed to read the content of `{}`. Unable to determine the presence of glossary and/or bibliography.\n{}",
            main_tex_file.display(),
            e.to_string()
        ))
    })?;

    // FIX: there are a couple of ways the detecion of glossary and bibliography
    //      could go wrong:
    //          1. If the main `.tex` file doesn't include directly the
    //             glossary/bibliography library, but they're included
    //             by secondary files.
    //          2. If the \usepackage takes some options with further
    //             square brackets inside:
    //                  % OK
    //                  \usepackage[my_opt=\my_command{y}]{biblatex}
    //                  % FAIL
    //                  \usepackage[my_opt=\my_command[x]{y}]{biblatex}
    //      These should be just minor cases, so they're ignored at the
    //      moment
    let glossary_re = Regex::new(r"\\usepackage(?:\[[^\[\]]*\])?\{glossaries\}").unwrap();
    let bibliography_re =
        Regex::new(r"\\usepackage(?:\[[^\[\]]*\])?\{(?:biblatex|bibtex|natbib)\}").unwrap();

    let has_glossary: bool = glossary_re.is_match(&content);
    let has_bibliography: bool = bibliography_re.is_match(&content);

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
    if has_glossary {
        log_info(format!("Glossary detected. Compiling it."));
        run_shell_cmd(
            Command::new("makeglossaries")
                .arg("-d")
                .arg(OUTPUT_DIRECTORY)
                .arg(MAIN_FILE_NAME),
        )?;
    }
    // Run biber
    if has_bibliography {
        log_info(format!("Bibliography detected. Compiling it."));
        run_shell_cmd(
            Command::new("biber")
                .arg("--input-directory")
                .arg(OUTPUT_DIRECTORY)
                .arg("--output-directory")
                .arg(OUTPUT_DIRECTORY)
                .arg(MAIN_FILE_NAME),
        )?;
    }

    // 4. Run pdflatex
    if has_glossary || has_bibliography {
        log_info(format!(
            "Recompiling the `.pdf` to include the newly generated glossary/bibliography."
        ));
        run_shell_cmd(
            Command::new("pdflatex")
                .arg("-halt-on-error")
                .arg("-output-directory")
                .arg(OUTPUT_DIRECTORY)
                .arg(MAIN_TEX_FILE),
        )?;
    }

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
