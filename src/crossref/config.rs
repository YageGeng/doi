use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct CrossrefConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub mailto: Option<String>,
    pub user_agent: Option<String>,
    pub rate_limit_per_sec: u32,
    pub concurrency: usize,
    pub retry_max: u32,
    pub retry_min_backoff: Duration,
    pub retry_max_backoff: Duration,
    pub retry_jitter: bool,
}

impl Default for CrossrefConfig {
    /// Return default Crossref client configuration values.
    fn default() -> Self {
        Self {
            base_url: "https://api.crossref.org/v1".to_string(),
            timeout: Duration::from_secs(30),
            mailto: None,
            user_agent: None,
            rate_limit_per_sec: 10,
            concurrency: 3,
            retry_max: 5,
            retry_min_backoff: Duration::from_secs(1),
            retry_max_backoff: Duration::from_secs(60),
            retry_jitter: true,
        }
    }
}
