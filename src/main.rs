mod cli;
mod config;

fn main() {
    // Parses input arguments
    let args = cli::build_parser().try_get_matches();

    println!("{:?}", args);
}
