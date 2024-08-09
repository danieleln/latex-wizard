mod compile_tex_file;
mod find_root_dir;

use crate::config::structure::{MAIN_PDF_FILE, OUTPUT_DIRECTORY};
use crate::logs::{log_error, log_info, log_warning, Log};
use clap::ArgMatches;
use compile_tex_file::compile_tex_file;
use find_root_dir::find_project_root_directory;
use std::fs;
use std::path::PathBuf;

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
