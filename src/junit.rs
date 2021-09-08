use std::fs::File;
use xml::writer::{EmitterConfig, XmlEvent};
use crate::cases::{TestCase, TestFailure, TestOutcome};


pub fn create_junit_file(cases: &Vec<TestCase>, failures: &Vec<TestFailure>) {
    let mut file = File::create("junit.xml").unwrap();
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut file);

    writer.write(XmlEvent::start_element("testsuite").attr("id", "cargo test").attr("name", "cargo test").attr("tests", &cases.len().to_string())).unwrap();

    for case in cases {
        writer.write(XmlEvent::start_element("testcase").attr("id", &case.id).attr("name", &case.id)).unwrap();
        if case.outcome == TestOutcome::Failed {
            for failure in failures {
                if failure.id == case.id {
                    for output in &failure.outputs {
                        writer.write(XmlEvent::start_element("failure").attr("message", &output.data).attr("type", output.r#type.to_string())).unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                    }
                }
            }
        }
        writer.write(XmlEvent::end_element()).unwrap();
    }

    writer.write(XmlEvent::end_element()).unwrap();
}
