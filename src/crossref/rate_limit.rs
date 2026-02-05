use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter, clock::DefaultClock};
use http::Extensions;
use reqwest_middleware::{Middleware, Next};
use std::num::NonZeroU32;

/// Middleware that enforces a per-second rate limit for Crossref requests.
pub struct RateLimitMiddleware {
    limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl RateLimitMiddleware {
    /// Create a rate limiter with a per-second quota.
    pub fn new(rate_limit_per_sec: u32) -> Self {
        let per_second = NonZeroU32::new(rate_limit_per_sec.max(1))
            .unwrap_or_else(|| NonZeroU32::new(1).expect("nonzero"));
        let limiter = RateLimiter::direct(Quota::per_second(per_second));
        Self { limiter }
    }
}

#[async_trait::async_trait]
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
