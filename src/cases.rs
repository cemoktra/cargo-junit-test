use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum TestOutcome {
    Unknown,
    Passed,
    Failed,
    Ignored,
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
    pub outcome: TestOutcome,
}

#[derive(Debug)]
pub enum TestOutputType {
    Unknown,
    Stdout,
    Stderr,
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
    pub data: String,
}

#[derive(Debug)]
pub struct TestFailure {
    pub id: String,
    pub outputs: Vec<TestOutput>,
}

#[derive(Debug)]
pub struct TestSuite {
    pub id: String,
    pub cases: Vec<TestCase>,
    pub duration: f64,
}

pub struct TestFailures {}

impl TestSuite {
    pub fn from(test_output: &[&str]) -> TestSuite {
        let mut test_suite = TestSuite {
            id: "unittests".into(),
            cases: Vec::new(),
            duration: 0.0,
        };
        let re_cases = Regex::new(r"test (.*) ... (.*)").unwrap();
        let re_duration = Regex::new(r"finished in (\d+\.\d+)").unwrap();

        for line in test_output {
            if let Some(captures) = re_cases.captures(line) {
                let id = captures.get(1).unwrap().as_str();
                if id != "result:" {
                    test_suite.cases.push(TestCase {
                        id: id.into(),
                        outcome: TestOutcome::from(captures.get(2).unwrap().as_str()),
                    });
                }
            }

            if let Some(captures) = re_duration.captures(line) {
                test_suite.duration += captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<f64>()
                    .ok()
                    .unwrap_or(0.0);
            }
        }

        test_suite
    }
}

impl TestFailures {
    pub fn from(test_output: &[&str]) -> Vec<TestFailure> {
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
                        outputs: Vec::new(),
                    };

                    for line in &mut lines {
                        if line.is_empty() {
                            failure.outputs.push(TestOutput {
                                r#type: TestOutputType::from(r#type),
                                data: output.join("\n"),
                            });
                            break;
                        } else {
                            output.push(String::from(*line));
                        }
                    }

                    failures.push(failure);
                }
                None => {
                    if *line == "stderr:" {
                        let mut output = Vec::new();
                        for line in &mut lines {
                            if line.is_empty() {
                                match failures.last_mut() {
                                    Some(failure) => {
                                        failure.outputs.push(TestOutput {
                                            r#type: TestOutputType::Stderr,
                                            data: output.join("\n"),
                                        });
                                        break;
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            } else {
                                output.push(String::from(*line))
                            }
                        }
                    }
                }
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
            "test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.50s",
            "    Doc-tests whatever",
            "test src/lib.rs - Struct::passed (line 20) ... ok",
            "test src/lib.rs - Struct::failed (line 50) ... FAILED",
            "test result: ok. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s",
        ];

        let suite = super::TestSuite::from(&test_data);
        assert_eq!("unittests", suite.id);
        assert_eq!(2.6, suite.duration);
        assert_eq!(4, suite.cases.len());
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
