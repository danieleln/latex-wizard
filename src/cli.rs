use crate::config;
use clap::{Arg, ArgAction, Command};

pub fn build_parser() -> Command {
    Command::new(config::info::APP)
        .about(config::info::DESCRIPTION)
        .author("Daniele Monzani")
        .subcommand_required(true)
        // Init subcommand
        .subcommand(
            Command::new("init")
                .about("Start a new LaTeX project")
                // Name of the LaTeX project to initialize
                .arg(
                    Arg::new("project")
                        .help("Name of the LaTeX project to initialize")
                        .required(true),
                ),
        )
        // Compile subcommand
        .subcommand(
            Command::new("compile")
                .about("Compile a LaTeX project")
                .arg(
                    Arg::new("project")
                        .help("Name of the LaTeX project to compile")
                        .required(false),
                )
                .arg(
                    Arg::new("clean")
                        .short('c')
                        .long("clean")
                        .help("Start the compilation from scratch, by deleting the output of previous compilations")
                        .action(ArgAction::SetTrue)
                        .required(false)
                ),
        )
}
