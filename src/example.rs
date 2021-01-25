use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct Example {
    executable: PathBuf,
    input_filepath: PathBuf,
    output_filepath: PathBuf,
}

pub fn examples_from(
    executable: &str,
    input_folder: &str,
    output_folder: &str,
) -> Result<Vec<Example>, Box<dyn Error>> {
    let mut examples: Vec<Example> = vec![];

    for entry in read_dir(input_folder)? {
        let entry = entry?;

        examples.push(Example {
            executable: PathBuf::from(executable),
            input_filepath: entry.path(),
            output_filepath: Path::new(output_folder).join(entry.file_name()),
        });
    }

    Ok(examples)
}
