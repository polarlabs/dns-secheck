mod status;
pub use status::Status;

mod test_report;
pub use test_report::TestReport;

pub(crate) mod base32;
pub(crate) mod string;
mod test_result;

pub use test_result::TestResult;

use crate::constants::DNS_SECHECK_KEY_LENGTH_USIZE;
use rand::RngExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestKey(String);

impl TestKey {
    pub fn new() -> TestKey {
        let mut rng = rand::rng();

        let code: String = (0..DNS_SECHECK_KEY_LENGTH_USIZE)
            .map(|_| rng.random_range(0..10).to_string())
            .collect();

        TestKey(code)
    }
}

impl From<&[u8]> for TestKey {
    fn from(bytes: &[u8]) -> TestKey {
        TestKey(String::from_utf8_lossy(bytes).into_owned())
    }
}

impl From<&str> for TestKey {
    fn from(str: &str) -> TestKey {
        TestKey(str.to_string())
    }
}

impl From<String> for TestKey {
    fn from(string: String) -> TestKey {
        TestKey(string)
    }
}
