mod sub_task;
mod task;

use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::sub_task::SubTask;
use crate::task::TaskResult;
use std::time::Duration;

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

fn print_task_header(
    input_filepath: &Path,
    status: &str,
    status_color: term::color::Color,
    elapsed_time: Duration,
) {
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

    t.fg(term::color::BRIGHT_BLACK).unwrap();
    write!(t, "   (time: {:.3}s)", elapsed_time.as_secs_f32()).unwrap();
    t.reset().unwrap();

    writeln!(t).unwrap();
}

fn print_sub_tasks(sub_tasks: Vec<SubTask>) {
    let mut t = term::stdout().unwrap();
    let indent = "  ";

    writeln!(t).unwrap();

    for (index, sub_task) in sub_tasks.iter().enumerate() {
        write!(t, "{}", indent).unwrap();
        if sub_task.actual_output == sub_task.expected_output {
            t.fg(term::color::GREEN).unwrap();
            write!(t, "\u{2713}").unwrap();
            t.reset().unwrap();
        } else {
            t.fg(term::color::RED).unwrap();
            write!(t, "\u{2717}").unwrap();
            t.reset().unwrap();
        }
        write!(t, " subtask#{}\n", index + 1).unwrap();

        if sub_task.actual_output != sub_task.expected_output {
            t.fg(term::color::BRIGHT_BLACK).unwrap();
            write!(t, "{}{}expected: ", indent, indent).unwrap();
            t.fg(term::color::GREEN).unwrap();
            writeln!(t, "{}", sub_task.expected_output).unwrap();

            t.fg(term::color::BRIGHT_BLACK).unwrap();
            write!(t, "{}{}received: ", indent, indent).unwrap();
            t.fg(term::color::RED).unwrap();
            writeln!(t, "{}", sub_task.actual_output).unwrap();

            t.reset().unwrap();
        }
    }

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
            TaskResult::Accepted {
                elapsed_time,
                sub_tasks,
            } => {
                print_task_header(
                    task.input_filepath.as_path(),
                    "PASS",
                    term::color::GREEN,
                    elapsed_time,
                );
                print_sub_tasks(sub_tasks);
            }
            TaskResult::WrongAnswer {
                elapsed_time,
                sub_tasks,
            } => {
                print_task_header(
                    task.input_filepath.as_path(),
                    "FAIL",
                    term::color::RED,
                    elapsed_time,
                );
                print_sub_tasks(sub_tasks);
            }
            TaskResult::TimeLimitExceeded { elapsed_time } => {
                print_task_header(
                    task.input_filepath.as_path(),
                    "TLE ",
                    term::color::YELLOW,
                    elapsed_time,
                );
            }
            TaskResult::RuntimeError { elapsed_time } => {
                print_task_header(
                    task.input_filepath.as_path(),
                    "RTE ",
                    term::color::BRIGHT_RED,
                    elapsed_time,
                );
            }
        }
    }

    Ok(())
}
