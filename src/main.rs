use cases::{TestFailures, TestSuite};
use junit::create_junit_file;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use structopt::StructOpt;

mod cases;
mod junit;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Options {
    #[structopt(long)]
    release: bool,
}

fn main() {
    let options = Options::from_args();

    let mut args = Vec::new();
    if options.release {
        args.push("--release");
    }

    let process = Command::new("cargo")
        .arg("test")
        .arg("--no-fail-fast")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = process.stdout.unwrap();
    let reader = BufReader::new(stdout);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let lines_ref = lines.iter().map(|item| &**item).collect::<Vec<&str>>();

    let suite = TestSuite::from(&lines_ref);
    let failures = TestFailures::from(&lines_ref);

    create_junit_file(&suite, &failures);
}
