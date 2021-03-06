use crate::cases::{TestFailure, TestOutcome, TestSuite};
use std::fs::File;
use xml::writer::{EmitterConfig, XmlEvent};

pub fn create_junit_file(suite: &TestSuite, failures: &[TestFailure]) {
    let mut file = File::create("junit.xml").unwrap();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut file);

    writer
        .write(
            XmlEvent::start_element("testsuite")
                .attr("id", &suite.id)
                .attr("name", &suite.id)
                .attr("tests", &suite.cases.len().to_string())
                .attr("time", &format!("{:2}", suite.duration)),
        )
        .unwrap();

    for case in &suite.cases {
        writer
            .write(
                XmlEvent::start_element("testcase")
                    .attr("id", &case.id)
                    .attr("name", &case.id),
            )
            .unwrap();
        if case.outcome == TestOutcome::Failed {
            for failure in failures {
                if failure.id == case.id {
                    for output in &failure.outputs {
                        writer
                            .write(
                                XmlEvent::start_element("failure")
                                    .attr("message", &output.data)
                                    .attr("type", output.r#type.to_string()),
                            )
                            .unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                    }
                }
            }
        }
        writer.write(XmlEvent::end_element()).unwrap();
    }
    writer.write(XmlEvent::end_element()).unwrap();
}
