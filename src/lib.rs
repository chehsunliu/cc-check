mod sub_task;
mod task;

use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::task::TaskResult;

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

fn print_task_header(input_filepath: &Path, status: &str, status_color: term::color::Color) {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::BLACK).unwrap();
    t.bg(status_color).unwrap();
    write!(t, " {} ", status).unwrap();
    t.reset().unwrap();

    let folder_path = input_filepath.parent().unwrap().to_str().unwrap();
    let filename = input_filepath.file_name().unwrap().to_str().unwrap();
    t.fg(term::color::BRIGHT_BLACK).unwrap();
    write!(t, " {}/", folder_path).unwrap();
    t.fg(term::color::WHITE).unwrap();
    write!(t, "{}", filename).unwrap();
    t.reset().unwrap();

    writeln!(t).unwrap();
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tasks = load_tasks(
        config.executable.as_str(),
        config.input_folder.as_str(),
        config.output_folder.as_str(),
    )?;

    for task in tasks {
        match task.run()? {
            TaskResult::Accepted { .. } => {
                print_task_header(task.input_filepath.as_path(), "PASS", term::color::GREEN);
            }
            TaskResult::WrongAnswer { .. } => {
                print_task_header(task.input_filepath.as_path(), "FAIL", term::color::RED);
            }
            TaskResult::TimeLimitExceeded { .. } => {
                print_task_header(task.input_filepath.as_path(), "TLE ", term::color::RED);
            }
            TaskResult::RuntimeError { .. } => {
                print_task_header(task.input_filepath.as_path(), "RTE ", term::color::RED);
            }
        }
    }

    Ok(())
}
