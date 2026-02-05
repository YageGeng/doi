use std::time::Duration;

/// Configuration for doi.org metadata retrieval.
#[derive(Debug, Clone, PartialEq)]
pub struct DoiOrgConfig {
    /// Base URL for doi.org content negotiation (default: https://doi.org).
    pub base_url: String,
    /// HTTP client timeout applied to each request.
    pub timeout: Duration,
    /// Contact email used for polite identification in the User-Agent header.
    pub mailto: Option<String>,
    /// Application identifier used in the User-Agent header.
    pub user_agent: Option<String>,
}

impl Default for DoiOrgConfig {
    /// Return default doi.org client configuration values.
    fn default() -> Self {
        Self {
            base_url: "https://doi.org".to_string(),
            timeout: Duration::from_secs(30),
            mailto: None,
            user_agent: None,
        }
    }
}

impl DoiOrgConfig {
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

    /// Configure polite access with a contact email.
    pub fn polite(mut self, email: &str) -> Self {
        let trimmed = email.trim();
        if !trimmed.is_empty() {
            self.mailto = Some(trimmed.to_string());
        }
        self
    }
}
