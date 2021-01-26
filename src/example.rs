use std::error::Error;
use std::path::PathBuf;

pub struct Example {
    executable: PathBuf,
    input_filepath: PathBuf,
    output_filepath: PathBuf,
}

impl Example {
    pub fn new(executable: PathBuf, input_filepath: PathBuf, output_filepath: PathBuf) -> Example {
        Example {
            executable,
            input_filepath,
            output_filepath,
        }
    }
}
