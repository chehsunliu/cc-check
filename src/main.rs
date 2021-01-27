use cc_check;
use clap::{App, Arg};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let default_timeout = 3000;

    let matches = App::new("GG")
        .arg(
            Arg::with_name("executable")
                .long("executable")
                .short("e")
                .takes_value(true)
                .value_name("FILE")
                .required(true)
                .help("The executable to run"),
        )
        .arg(
            Arg::with_name("input-folder")
                .long("input-folder")
                .short("i")
                .takes_value(true)
                .value_name("FOLDER")
                .required(true)
                .help("The folder containing testing files"),
        )
        .arg(
            Arg::with_name("output-folder")
                .long("output-folder")
                .short("o")
                .takes_value(true)
                .value_name("FOLDER")
                .required(true)
                .help("The folder containing expected result files"),
        )
        .arg(
            Arg::with_name("task-timeout")
                .long("task-timeout")
                .takes_value(true)
                .value_name("SECONDS")
                .help(&format!(
                    "The timeout in milliseconds. [default: {}ms]",
                    default_timeout
                )),
        )
        .get_matches();

    let executable = matches.value_of("executable").unwrap().to_string();
    let input_folder = matches.value_of("input-folder").unwrap().to_string();
    let output_folder = matches.value_of("output-folder").unwrap().to_string();
    let task_timeout = match matches.value_of("task-timeout") {
        None => default_timeout,
        Some(s) => s.parse().unwrap(),
    };

    cc_check::run(cc_check::Config {
        executable,
        input_folder,
        output_folder,
        task_timeout: Duration::from_millis(task_timeout),
    })
}
