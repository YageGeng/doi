use crate::Doi;
use crate::crossref::config::CrossrefConfig;
use crate::crossref::error::*;
use crate::crossref::models::CrossrefResponse;
use async_trait::async_trait;
use governor::clock::DefaultClock;
use governor::state::InMemoryState;
use governor::state::NotKeyed;
use governor::{Quota, RateLimiter};
use http::Extensions;
use reqwest::StatusCode;
use reqwest::header::{RETRY_AFTER, USER_AGENT};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::{
    Jitter, RetryDecision, RetryPolicy, RetryTransientMiddleware, Retryable, RetryableStrategy,
    default_on_request_failure,
};
use snafu::ResultExt;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::sync::Semaphore;

const DEFAULT_MAILTO: &str = "icoderdev@outlook.com";

pub struct CrossrefClient {
    client: ClientWithMiddleware,
    base_url: String,
    mailto: String,
    user_agent: Option<String>,
    concurrency: Arc<Semaphore>,
}

impl CrossrefClient {
    /// Build a Crossref client with retry and rate-limit middleware.
    pub fn new(config: CrossrefConfig) -> std::result::Result<Self, CrossrefError> {
        let mailto = config
            .mailto
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_MAILTO.to_string());
        let user_agent = config.user_agent.filter(|value| !value.trim().is_empty());
        let base_url = config.base_url.trim_end_matches('/').to_string();
        let concurrency = Arc::new(Semaphore::new(config.concurrency.max(1)));

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
        let limiter = RateLimitMiddleware::new(config.rate_limit_per_sec);

        let client = reqwest::Client::builder()
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
            client,
            base_url,
            mailto,
            user_agent,
            concurrency,
        })
    }

    /// Fetch metadata for a DOI from the Crossref REST API.
    pub async fn fetch_metadata(
        &self,
        doi: &Doi,
    ) -> std::result::Result<CrossrefResponse, CrossrefError> {
        let _permit = self.concurrency.acquire().await.context(SemaphoreSnafu {
            stage: "acquire-permit",
        })?;
        let url = format!("{}/works/{}", self.base_url, doi.canonical);
        let mut request = self
            .client
            .get(url)
            .query(&[("mailto", self.mailto.as_str())]);

        if let Some(app_name) = self.user_agent.as_ref() {
            let value = format!("{} {}", app_name, self.mailto);
            request = request.header(USER_AGENT, value);
        }

        let response = request
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
}

struct RateLimitMiddleware {
    limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl RateLimitMiddleware {
    /// Create a rate limiter with a per-second quota.
    fn new(rate_limit_per_sec: u32) -> Self {
        let per_second = NonZeroU32::new(rate_limit_per_sec.max(1))
            .unwrap_or_else(|| NonZeroU32::new(1).expect("nonzero"));
        let limiter = RateLimiter::direct(Quota::per_second(per_second));
        Self { limiter }
    }
}

#[async_trait]
impl Middleware for RateLimitMiddleware {
    /// Enforce rate limiting before forwarding the request.
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        self.limiter.until_ready().await;
        next.run(req, extensions).await
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
        res: &Result<reqwest::Response, reqwest_middleware::Error>,
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
