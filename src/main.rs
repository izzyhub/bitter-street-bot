use feed_the_fed::run;

use clap::Arg;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = command!().arg(
        Arg::new("configuration-file")
        .required(true)
    ).get_matches();

    let configuration_path = matches.value_of_os("configuration-file").map(std::path::PathBuf::from);
    feed_the_fed::run(configuration)
}
