pub mod info {
    pub const APP: &str = "latex";
    pub const DESCRIPTION: &str = "Tool to manage LaTeX projects";
}

pub mod structure {
    use const_format::formatcp;

    pub const OUTPUT_DIRECTORY: &str = "out";
    pub const MAIN_FILE_NAME: &str = "main";
    pub const MAIN_TEX_FILE: &str = formatcp!("{}.tex", MAIN_FILE_NAME);
    pub const MAIN_PDF_FILE: &str = formatcp!("{}.pdf", MAIN_FILE_NAME);
}
