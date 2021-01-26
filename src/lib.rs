mod sub_task;
mod task;

use crate::task::TaskResult;
use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct Config {
    pub executable: String,
    pub input_folder: String,
    pub output_folder: String,
}

fn load_tasks(
    executable: &str,
    input_folder: &str,
    output_folder: &str,
) -> Result<Vec<task::Task>, Box<dyn Error>> {
    let mut tasks: Vec<task::Task> = vec![];

    for entry in read_dir(input_folder)? {
        let entry = entry?;
        let task = task::Task::new(
            PathBuf::from(executable),
            entry.path(),
            Path::new(output_folder).join(entry.file_name()),
        );

        tasks.push(task);
    }

    tasks.sort_by(|a, b| a.input_filepath.cmp(&b.input_filepath));
    Ok(tasks)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tasks = load_tasks(
        config.executable.as_str(),
        config.input_folder.as_str(),
        config.output_folder.as_str(),
    )?;

    for task in tasks {
        match task.run()? {
            TaskResult::Accepted { .. } => {}
            TaskResult::WrongAnswer { .. } => {}
            TaskResult::TimeLimitExceeded { .. } => {}
            TaskResult::RuntimeError { .. } => {}
        }
    }

    Ok(())
}
