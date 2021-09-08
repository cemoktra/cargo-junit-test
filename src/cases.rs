use regex::Regex;

pub struct TestCases {

}

#[derive(Debug, PartialEq)]
pub enum TestOutcome {
    Unknown,
    Passed,
    Failed,
    Ignored
}

impl TestOutcome {
    pub fn from(text: &str) -> TestOutcome {
        match text {
            "ok" => TestOutcome::Passed,
            "FAILED" => TestOutcome::Failed,
            "ignored" => TestOutcome::Ignored,
            _ => TestOutcome::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct TestCase {
    pub id: String,
    pub outcome: TestOutcome
}

#[derive(Debug)]
pub enum TestOutputType {
    Unknown,
    Stdout,
    Stderr
}

impl TestOutputType {
    pub fn from(text: &str) -> TestOutputType {
        match text {
            "stdout" => TestOutputType::Stdout,
            "stderr" => TestOutputType::Stderr,
            _ => TestOutputType::Unknown,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            TestOutputType::Unknown => "unknown",
            TestOutputType::Stdout => "stdout",
            TestOutputType::Stderr => "stderr",
        }
    }
}

#[derive(Debug)]
pub struct TestOutput {
    pub r#type: TestOutputType,
    pub data: String
}

#[derive(Debug)]
pub struct TestFailure {
    pub id: String,
    pub outputs: Vec<TestOutput>
}

pub struct TestFailures {
}

impl TestCases {
    pub fn from(test_output: &Vec<&str>) -> Vec<TestCase> {
        let mut test_cases = Vec::new();
        let re = Regex::new(r"test (.*) ... (.*)").unwrap();

        for line in test_output {
            match re.captures(line) {
                Some(captures) => {
                    let id = captures.get(1).unwrap().as_str();
                    if id != "result:" {
                        test_cases.push(TestCase {
                            id: id.into(),
                            outcome: TestOutcome::from(captures.get(2).unwrap().as_str())
                        });
                    }
                },
                None => {}
            };
        }

        test_cases
    }
}

impl TestFailures {
    pub fn from(test_output: &Vec<&str>) -> Vec<TestFailure> {
        let mut failures = Vec::new();
        // let mut failure = None;
        let re = Regex::new(r"---- (.*) (\S+) ----").unwrap();

        let mut lines = test_output.iter();
        while let Some(line) = lines.next() {
            match re.captures(line) {
                Some(captures) => {
                    let id = captures.get(1).unwrap().as_str();
                    let r#type = captures.get(2).unwrap().as_str();
                    let mut output = Vec::new();

                    let mut failure = TestFailure {
                        id: id.into(),
                        outputs: Vec::new()
                    };

                    loop {
                        match lines.next() {
                            Some(line) => {
                                if line.is_empty() {
                                    failure.outputs.push(TestOutput {
                                        r#type: TestOutputType::from(r#type),
                                        data: output.join("\n"),
                                    });
                                    break;
                                } else {
                                    output.push(line.clone())
                                }
                            },
                            None => {
                                break;
                            },
                        }
                    }

                    failures.push(failure);
                }
                None => {
                    if *line == "stderr:" {
                        let mut output = Vec::new();
                        loop {
                            match lines.next() {
                                Some(line) => {
                                    if line.is_empty() {
                                        match failures.last_mut() {
                                            Some(failure) => {
                                                failure.outputs.push(TestOutput {
                                                    r#type: TestOutputType::Stderr,
                                                    data: output.join("\n"),
                                                });
                                                break;
                                            },
                                            None => {
                                                break;
                                            },
                                        }
                                    } else {
                                        output.push(line.clone())
                                    }
                                },
                                None => {
                                    break;
                                },
                            }
                        }
                    }
                },
            }
        }

        failures
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn discover_cases() {
        let test_data = vec![
            "test module::passed ... ok",
            "test module::failed ... FAILED",
            "test src/lib.rs - Struct::passed (line 20) ... ok",
            "test src/lib.rs - Struct::failed (line 50) ... FAILED"
        ];

        let cases = super::TestCases::from(&test_data);

        assert_eq!(4, cases.len());
    }

    #[test]
    fn discover_failures() {
        let test_data = vec![
            "",
            "---- src/lib.rs - Monetary::zero (line 50) stdout ----",
            "Test executable failed (exit code 101).",
            "",
            "stderr:",
            "thread 'main' panicked at 'assertion failed: false', src/lib.rs:9:1",
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace",
            "",
            "---- tests::test_is_zero stdout ----",
            "thread 'tests::test_is_zero' panicked at 'assertion failed: false', src/lib.rs:186:9",
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace",
            "",
        ];

        let failures = super::TestFailures::from(&test_data);

        assert_eq!(2, failures.len());
    }
}