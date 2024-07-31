const TAG_ERROR: &str = "[\x1b[31mERR\x1b[0m]";
// const TAG_WARNING: &str = "[\x1b[33mWRN\x1b[0m]";
// const TAG_INFO: &str = "[\x1b[36mWRN\x1b[0m]";
const TAG_HELP: &str = "[\x1b[32mHLP\x1b[0m]";

#[derive(thiserror::Error, Debug)]
pub enum Log {
    #[error("{} {}", TAG_HELP, .0)]
    HelpMessage(String),

    #[error("{} {}", TAG_ERROR, .0)]
    InvalidCommandLineArgument(String),
    // #[error("{} {}", TAG_ERROR, .0)]
    // Generic(String),
}

impl From<clap::error::Error> for Log {
    fn from(e: clap::error::Error) -> Self {
        match e.kind() {
            clap::error::ErrorKind::DisplayHelp => Log::HelpMessage(e.to_string()),
            _ => {
                let mut msg = e.to_string();
                if msg.starts_with("error: ") {
                    Log::InvalidCommandLineArgument(msg.split_off(7))
                } else {
                    Log::InvalidCommandLineArgument(msg)
                }
            }
        }
    }
}