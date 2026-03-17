use serde::Serialize;

use crate::util::test_result::TestResult;

#[derive(Clone, Serialize)]
pub struct TestReport(Vec<TestResult>);

impl TestReport {
    pub fn new() -> TestReport {
        TestReport(Vec::new())
    }

    pub fn push(&mut self, value: TestResult) {
        self.0.push(value);
    }
}
