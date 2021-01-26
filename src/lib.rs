mod example;

use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct Config {
    pub executable: String,
    pub input_folder: String,
    pub output_folder: String,
}

fn create_examples(
    executable: &str,
    input_folder: &str,
    output_folder: &str,
) -> Result<Vec<example::Example>, Box<dyn Error>> {
    let mut examples: Vec<example::Example> = vec![];

    for entry in read_dir(input_folder)? {
        let entry = entry?;
        let ex = example::Example::new(
            PathBuf::from(executable),
            entry.path(),
            Path::new(output_folder).join(entry.file_name()),
        );

        examples.push(ex);
    }

    Ok(examples)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let examples = create_examples(
        config.executable.as_str(),
        config.input_folder.as_str(),
        config.output_folder.as_str(),
    )?;

    Ok(())
}
