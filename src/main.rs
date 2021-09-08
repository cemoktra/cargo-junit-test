use std::{io::{BufRead, BufReader}, process::{Command, Stdio}};

use cases::{TestCases, TestFailures};

use crate::junit::create_junit_file;

mod cases;
mod junit;

fn main() {
    let process = Command::new("cargo").arg("test").arg("--no-fail-fast").stdout(Stdio::piped()).spawn().unwrap();
    let stdout = process.stdout.unwrap();
    let reader = BufReader::new(stdout);
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let lines_ref = lines.iter().map(|item| &**item).collect::<Vec<&str>>();

    let cases = TestCases::from(&lines_ref);
    let failures = TestFailures::from(&lines_ref);

    create_junit_file(&cases, &failures);
}
