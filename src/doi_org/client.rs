use crate::doi_org::config::DoiOrgConfig;
use crate::doi_org::error::*;
use crate::{Doi, csl::*};
use reqwest::header::{ACCEPT, USER_AGENT};
use snafu::ResultExt;

/// Client for doi.org content negotiation.
pub struct DoiOrgClient {
    client: reqwest::Client,
    base_url: String,
}

impl DoiOrgClient {
    const CSL_JSON_ACCEPT: &'static str = "application/vnd.citationstyles.csl+json";

    /// Build a doi.org client with configured defaults.
    pub fn new(config: DoiOrgConfig) -> std::result::Result<Self, DoiOrgError> {
        let base_url = config.base_url_value();
        let client = reqwest::Client::builder()
            .default_headers(Self::default_headers(&config))
            .timeout(config.timeout)
            .build()
            .context(RequestSnafu {
                stage: "build-client",
            })?;

        Ok(Self { client, base_url })
    }

    /// Fetch CSL-JSON metadata for a DOI via doi.org content negotiation.
    pub async fn metadata(&self, doi: &Doi) -> std::result::Result<DoiMetadata, DoiOrgError> {
        let url = self.build_url(doi);

        let response = self
            .client
            .get(url)
            .header(ACCEPT, Self::CSL_JSON_ACCEPT)
            .send()
            .await
            .context(RequestSnafu {
                stage: "send-request",
            })?
            .error_for_status()
            .context(RequestSnafu {
                stage: "http-status",
            })?;

        let text = response.text().await.context(RequestSnafu {
            stage: "response-body",
        })?;

        let mut deserializer = serde_json::Deserializer::from_str(&text);

        serde_path_to_error::deserialize::<_, DoiMetadata>(&mut deserializer).context(
            SerializePathSnafu {
                stage: "parse-json",
            },
        )
    }

    /// Build the doi.org URL for a DOI.
    fn build_url(&self, doi: &Doi) -> String {
        format!("{}/{}", self.base_url, doi.as_str())
    }

    /// Build default headers for the doi.org client.
    fn default_headers(config: &DoiOrgConfig) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(user_agent) = Self::user_agent_header_value(config)
            && let Ok(value) = reqwest::header::HeaderValue::from_str(&user_agent)
        {
            headers.insert(USER_AGENT, value);
        }
        headers
    }

    /// Build the User-Agent header value when configured.
    fn user_agent_header_value(config: &DoiOrgConfig) -> Option<String> {
        match (config.user_agent_value(), config.mailto_value()) {
            (Some(agent), Some(mailto)) => Some(format!("{} mailto:{}", agent, mailto)),
            (Some(agent), None) => Some(agent.to_string()),
            (None, Some(mailto)) => Some(format!("mailto:{}", mailto)),
            (None, None) => None,
        }
    }
}
