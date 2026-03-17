use clap::crate_version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Status {
    status: String,
    version: String,
}

impl Status {
    pub fn new() -> Self {
        Self {
            status: "ok".to_string(),
            version: crate_version!().to_string(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
