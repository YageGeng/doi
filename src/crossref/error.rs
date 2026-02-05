use snafu::Snafu;
use tokio::sync::AcquireError;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum CrossrefError {
    #[snafu(display("HTTP request failed at {stage}: {source}"))]
    Request {
        stage: &'static str,
        source: reqwest_middleware::Error,
    },
    #[snafu(display("semaphore permit acquisition failed at {stage}: {source}"))]
    SemaphoreError {
        source: AcquireError,
        stage: &'static str,
    },
    #[snafu(display("reqwest error at {stage}: {source}"))]
    ReqwestError {
        stage: &'static str,
        source: reqwest::Error,
    },
}
