use crate::Doi;
use crate::crossref::config::CrossrefConfig;
use crate::crossref::error::*;
use crate::crossref::models::CrossrefResponse;
use crate::crossref::rate_limit::RateLimitMiddleware;
use reqwest::StatusCode;
use reqwest::header::{RETRY_AFTER, USER_AGENT};
use reqwest_middleware::*;
use reqwest_retry::{policies::ExponentialBackoff, *};
use snafu::ResultExt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::sync::Semaphore;

pub struct CrossrefClient {
    client: ClientWithMiddleware,
    base_url: String,
    concurrency: Arc<Semaphore>,
}

impl CrossrefClient {
    /// Build a Crossref client with retry and rate-limit middleware.
    pub fn new(config: CrossrefConfig) -> std::result::Result<Self, CrossrefError> {
        let concurrency = Arc::new(Semaphore::new(config.concurrency_value().max(1)));

        let retry_state = RetryAfterState::new();
        let retry_policy = RetryAfterPolicy::new(
            ExponentialBackoff::builder()
                .retry_bounds(config.retry_min_backoff, config.retry_max_backoff)
                .jitter(if config.retry_jitter {
                    Jitter::Full
                } else {
                    Jitter::None
                })
                .build_with_max_retries(config.retry_max),
            retry_state.clone(),
        );
        let retry_strategy = RetryAfterStrategy::new(retry_state);

        let retry_middleware =
            RetryTransientMiddleware::new_with_policy_and_strategy(retry_policy, retry_strategy);

        let limiter = RateLimitMiddleware::new(config.rate_limit_per_sec_value());

        let client = reqwest::Client::builder()
            .default_headers(Self::default_headers(&config))
            .timeout(config.timeout)
            .build()
            .context(ReqwestSnafu {
                stage: "build-client",
            })?;

        let client = ClientBuilder::new(client)
            .with(limiter)
            .with(retry_middleware)
            .build();

        Ok(Self {
            base_url: config.base_url_value(),
            client,
            concurrency,
        })
    }

    /// Fetch metadata for a DOI from the Crossref REST API.
    pub async fn metadata(
        &self,
        doi: &Doi,
    ) -> std::result::Result<CrossrefResponse, CrossrefError> {
        let _permit = self.concurrency.acquire().await.context(SemaphoreSnafu {
            stage: "acquire-permit",
        })?;

        let url = self.build_url(doi);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context(RequestSnafu {
                stage: "send-request",
            })?
            .error_for_status()
            .context(ReqwestSnafu {
                stage: "http-status",
            })?;

        response
            .json::<CrossrefResponse>()
            .await
            .context(ReqwestSnafu {
                stage: "parse-json",
            })
    }

    /// Build the Crossref REST API URL for a DOI.
    fn build_url(&self, doi: &Doi) -> String {
        format!("{}/works/{}", self.base_url, doi.as_str())
    }

    /// Build default headers for the Crossref client.
    fn default_headers(config: &CrossrefConfig) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(user_agent) = Self::user_agent_header_value(config)
            && let Ok(value) = reqwest::header::HeaderValue::from_str(&user_agent)
        {
            headers.insert(USER_AGENT, value);
        }

        headers
    }

    /// Build the User-Agent header value when configured.
    fn user_agent_header_value(config: &CrossrefConfig) -> Option<String> {
        match (config.user_agent_value(), config.mailto_value()) {
            (Some(agent), Some(mailto)) => Some(format!("{} mailto:{}", agent, mailto)),
            (Some(agent), None) => Some(agent.to_string()),
            (None, Some(mailto)) => Some(format!("mailto:{}", mailto)),
            (None, None) => None,
        }
    }
}

#[derive(Clone)]
struct RetryAfterState {
    next_retry: Arc<Mutex<Option<SystemTime>>>,
}

impl RetryAfterState {
    /// Initialize retry state with no pending Retry-After value.
    fn new() -> Self {
        Self {
            next_retry: Arc::new(Mutex::new(None)),
        }
    }

    /// Store the next allowed retry time from Retry-After.
    fn set_retry_after(&self, value: SystemTime) {
        if let Ok(mut guard) = self.next_retry.lock() {
            *guard = Some(value);
        }
    }

    /// Read and clear the stored Retry-After time.
    fn take_retry_after(&self) -> Option<SystemTime> {
        self.next_retry.lock().ok()?.take()
    }
}

struct RetryAfterPolicy {
    backoff: ExponentialBackoff,
    state: RetryAfterState,
}

impl RetryAfterPolicy {
    /// Build a retry policy with backoff and Retry-After tracking.
    fn new(backoff: ExponentialBackoff, state: RetryAfterState) -> Self {
        Self { backoff, state }
    }
}

impl RetryPolicy for RetryAfterPolicy {
    /// Decide whether to retry, honoring Retry-After when present.
    fn should_retry(&self, request_start_time: SystemTime, n_past_retries: u32) -> RetryDecision {
        let decision = self
            .backoff
            .should_retry(request_start_time, n_past_retries);

        match decision {
            RetryDecision::Retry { execute_after } => {
                if let Some(retry_after) = self.state.take_retry_after() {
                    let adjusted = if retry_after > execute_after {
                        retry_after
                    } else {
                        execute_after
                    };
                    return RetryDecision::Retry {
                        execute_after: adjusted,
                    };
                }
                RetryDecision::Retry { execute_after }
            }
            RetryDecision::DoNotRetry => RetryDecision::DoNotRetry,
        }
    }
}

struct RetryAfterStrategy {
    state: RetryAfterState,
}

impl RetryAfterStrategy {
    /// Create a strategy that records Retry-After values.
    fn new(state: RetryAfterState) -> Self {
        Self { state }
    }
}

impl RetryableStrategy for RetryAfterStrategy {
    /// Mark responses or errors as retryable based on status.
    fn handle(
        &self,
        res: &std::result::Result<reqwest::Response, reqwest_middleware::Error>,
    ) -> Option<Retryable> {
        match res {
            Ok(response) => {
                let status = response.status();
                if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                    if status == StatusCode::TOO_MANY_REQUESTS
                        && let Some(retry_after) = parse_retry_after(response)
                    {
                        self.state.set_retry_after(retry_after);
                    }
                    return Some(Retryable::Transient);
                }
                None
            }
            Err(error) => default_on_request_failure(error),
        }
    }
}

/// Parse the Retry-After header into an absolute time.
fn parse_retry_after(response: &reqwest::Response) -> Option<SystemTime> {
    let header_value = response.headers().get(RETRY_AFTER)?.to_str().ok()?;
    if let Ok(seconds) = header_value.parse::<u64>() {
        return SystemTime::now().checked_add(Duration::from_secs(seconds));
    }
    httpdate::parse_http_date(header_value).ok()
}
