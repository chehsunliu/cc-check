use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::sub_task::{create_sub_tasks, SubTask};

pub enum TaskResult {
    Accepted {
        elapsed_time: Duration,
        sub_tasks: Vec<SubTask>,
    },
    WrongAnswer {
        elapsed_time: Duration,
        sub_tasks: Vec<SubTask>,
    },
    TimeLimitExceeded {
        elapsed_time: Duration,
    },
    RuntimeError {
        elapsed_time: Duration,
    },
}

pub struct Task {
    pub executable: PathBuf,
    pub input_filepath: PathBuf,
    pub output_filepath: PathBuf,
}

fn create_final(
    child: Child,
    output_filepath: &Path,
    elapsed_time: Duration,
) -> Result<TaskResult, Box<dyn Error>> {
    let actual_outputs = match child.stdout {
        Some(mut stdout) => {
            let mut tmp = String::new();
            stdout.read_to_string(&mut tmp)?;
            tmp
        }
        None => "".to_string(),
    };
    let expected_outputs = read_to_string(output_filepath)?;
    let sub_tasks = create_sub_tasks(actual_outputs, expected_outputs);

    let mismatched_count = sub_tasks
        .iter()
        .filter(|t| t.expected_output != t.actual_output)
        .count();

    if mismatched_count == 0 {
        Ok(TaskResult::Accepted {
            elapsed_time,
            sub_tasks,
        })
    } else {
        Ok(TaskResult::WrongAnswer {
            elapsed_time,
            sub_tasks,
        })
    }
}

impl Task {
    pub fn new(executable: PathBuf, input_filepath: PathBuf, output_filepath: PathBuf) -> Task {
        Task {
            executable,
            input_filepath,
            output_filepath,
        }
    }

    pub fn run(&self) -> Result<TaskResult, Box<dyn Error>> {
        let file = File::open(self.input_filepath.as_path())?;
        let mut child = Command::new(self.executable.as_path())
            .stdin(Stdio::from(file))
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut elapsed_time = Duration::new(0, 0);
        let execution_time_limit = Duration::from_millis(800);
        let now = Instant::now();

        let exit_status = loop {
            if let Some(status) = child.try_wait()? {
                break status;
            }

            sleep(Duration::from_millis(1));
            elapsed_time = now.elapsed();

            if elapsed_time >= execution_time_limit {
                child.kill()?;
                return Ok(TaskResult::TimeLimitExceeded { elapsed_time });
            }
        };

        if exit_status.success() {
            create_final(child, self.output_filepath.as_path(), elapsed_time)
        } else {
            Ok(TaskResult::RuntimeError { elapsed_time })
        }
    }
}
