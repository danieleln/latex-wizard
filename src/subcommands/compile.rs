use crate::logs::Log;
use clap::ArgMatches;

pub fn compile(args: &ArgMatches) -> Result<(), Log> {
    println!("init: {:?}", args);
    Ok(())
}
