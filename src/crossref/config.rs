use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct CrossrefConfig {
    /// Base URL for the Crossref REST API (default: https://api.crossref.org/v1).
    pub base_url: String,
    /// HTTP client timeout applied to each request.
    pub timeout: Duration,
    /// Contact email used for polite pool identification in the User-Agent header.
    pub mailto: Option<String>,
    /// Application identifier used in the User-Agent header.
    pub user_agent: Option<String>,
    /// Optional explicit rate limit override (requests per second).
    pub rate_limit_per_sec: Option<u32>,
    /// Optional explicit concurrency override (simultaneous requests).
    pub concurrency: Option<usize>,
    /// Maximum number of retries for transient failures.
    pub retry_max: u32,
    /// Minimum backoff duration between retries.
    pub retry_min_backoff: Duration,
    /// Maximum backoff duration between retries.
    pub retry_max_backoff: Duration,
    /// Enable jitter for retry backoff to reduce thundering herd.
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
            rate_limit_per_sec: None,
            concurrency: None,
            retry_max: 5,
            retry_min_backoff: Duration::from_secs(1),
            retry_max_backoff: Duration::from_secs(60),
            retry_jitter: true,
        }
    }
}

impl CrossrefConfig {
    const DEFAULT_PUBLIC_RATE: u32 = 5;
    const DEFAULT_PUBLIC_CONCURRENCY: usize = 1;
    const DEFAULT_POLITE_RATE: u32 = 10;
    const DEFAULT_POLITE_CONCURRENCY: usize = 3;

    /// Return trimmed base URL without trailing slash.
    pub fn base_url_value(&self) -> String {
        self.base_url.trim_end_matches('/').to_string()
    }

    /// Return trimmed mailto when configured.
    pub fn mailto_value(&self) -> Option<&str> {
        self.mailto
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    /// Return trimmed user-agent when configured.
    pub fn user_agent_value(&self) -> Option<&str> {
        self.user_agent
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    /// Configure polite pool access with a contact email.
    pub fn polite(mut self, email: &str) -> Self {
        let trimmed = email.trim();
        if !trimmed.is_empty() {
            self.mailto = Some(trimmed.to_string());
        }
        self
    }

    /// Return effective rate limit per second based on mailto presence.
    pub fn rate_limit_per_sec_value(&self) -> u32 {
        if let Some(value) = self.rate_limit_per_sec {
            return value;
        }

        if self.mailto_value().is_some() {
            Self::DEFAULT_POLITE_RATE
        } else {
            Self::DEFAULT_PUBLIC_RATE
        }
    }

    /// Return effective concurrency limit based on mailto presence.
    pub fn concurrency_value(&self) -> usize {
        if let Some(value) = self.concurrency {
            return value;
        }

        if self.mailto_value().is_some() {
            Self::DEFAULT_POLITE_CONCURRENCY
        } else {
            Self::DEFAULT_PUBLIC_CONCURRENCY
        }
    }
}
