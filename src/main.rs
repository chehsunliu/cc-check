use cc_check;
use clap::{App, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("GG")
        .arg(
            Arg::with_name("executable")
                .long("executable")
                .takes_value(true)
                .value_name("FILE")
                .required(true)
                .help("The executable to run"),
        )
        .arg(
            Arg::with_name("input-folder")
                .long("input-folder")
                .takes_value(true)
                .value_name("FOLDER")
                .required(true)
                .help("The folder containing testing files"),
        )
        .arg(
            Arg::with_name("output-folder")
                .long("output-folder")
                .takes_value(true)
                .value_name("FOLDER")
                .required(true)
                .help("The folder containing expected result files"),
        )
        .get_matches();

    let executable = matches.value_of("executable").unwrap().to_string();
    let input_folder = matches.value_of("input-folder").unwrap().to_string();
    let output_folder = matches.value_of("output-folder").unwrap().to_string();

    cc_check::run(cc_check::Config {
        executable,
        input_folder,
        output_folder,
    })
}
