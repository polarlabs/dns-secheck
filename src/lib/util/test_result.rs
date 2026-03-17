use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct TestResult {
    timestamp: DateTime<Utc>,
    result: String,
}

impl TestResult {
    pub fn new() -> TestResult {
        TestResult {
            timestamp: Utc::now(),
            result: String::new(),
        }
    }
}

impl From<&str> for TestResult {
    fn from(s: &str) -> TestResult {
        TestResult {
            timestamp: Utc::now(),
            result: s.to_string(),
        }
    }
}
