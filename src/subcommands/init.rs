use crate::logs::Log;
use clap::ArgMatches;

pub fn init(args: &ArgMatches) -> Result<(), Log> {
    println!("init: {:?}", args);
    Ok(())
}
