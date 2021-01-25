use std::error::Error;

mod example;

pub struct Config {
    pub executable: String,
    pub input_folder: String,
    pub output_folder: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let examples = example::examples_from(
        config.executable.as_str(),
        config.input_folder.as_str(),
        config.output_folder.as_str(),
    )?;

    Ok(())
}
