use std::path::Path;

use clap::{Arg, command};

#[tokio::main]
async fn main() {

    let matches = command!().arg(
        Arg::new("configuration-file")
        .required(true)
        .help("path to the configration file with feed and authentication information")
    ).get_matches();

    let configuration_path = matches.value_of("configuration-file");
    let configuration_path = match configuration_path {
        Some(path) => path,
        None => {
            eprintln!("Pass the path of the configuration file");
            return;
        },
    };

    match bitter_street_bot::run(Path::new(configuration_path)).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    }
}
