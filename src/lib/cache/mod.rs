use crate::constants::{DNS_SECHECK_HTTP_CACHE_SIZE, DNS_SECHECK_HTTP_CACHE_TIME_TO_IDLE_S};
use crate::util::TestReport;
use crate::util::{TestKey, TestResult};

#[derive(Clone)]
pub struct Cache(moka::future::Cache<TestKey, TestReport>);

impl Cache {
    pub fn new() -> Cache {
        let cache = moka::future::Cache::builder()
            .max_capacity(DNS_SECHECK_HTTP_CACHE_SIZE)
            .time_to_idle(DNS_SECHECK_HTTP_CACHE_TIME_TO_IDLE_S)
            .build();

        Cache(cache)
    }

    pub async fn get(&self, key: &TestKey) -> Option<TestReport> {
        self.0.get(key).await
    }

    pub async fn insert(&self, key: &TestKey, value: TestReport) {
        self.0.insert(key.clone(), value).await;
    }

    pub async fn upsert(&self, key: TestKey, value: TestResult) {
        self.0
            .entry(key.clone())
            .and_upsert_with(|v| {
                let mut vec = if let Some(entry) = v {
                    entry.into_value()
                } else {
                    TestReport::new()
                };

                vec.push(value);

                std::future::ready(vec)
            })
            .await;
    }
}
