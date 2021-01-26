#[derive(Debug)]
pub struct SubTask {
    pub actual_output: String,
    pub expected_output: String,
}

fn parse_content(content: &str) -> Vec<&str> {
    content
        .trim_end()
        .split("\n")
        .map(|s| s.trim_end())
        .collect()
}

pub fn create_sub_tasks(actual_outputs: String, expected_outputs: String) -> Vec<SubTask> {
    let mut actual_outputs: Vec<_> = parse_content(&actual_outputs);
    let mut expected_outputs: Vec<_> = parse_content(&expected_outputs);

    let actual_minus_expected = actual_outputs.len() as i32 - expected_outputs.len() as i32;
    if actual_minus_expected > 0 {
        for _ in 0..actual_minus_expected {
            expected_outputs.push("<none>");
        }
    } else if actual_minus_expected < 0 {
        for _ in 0..actual_minus_expected {
            actual_outputs.push("<none>");
        }
    }

    let mut sub_tasks: Vec<SubTask> = vec![];
    for (actual_output, expected_output) in actual_outputs.iter().zip(expected_outputs.iter()) {
        sub_tasks.push(SubTask {
            actual_output: actual_output.to_string(),
            expected_output: expected_output.to_string(),
        });
    }

    sub_tasks
}
