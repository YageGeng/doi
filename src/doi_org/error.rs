use snafu::Snafu;

/// Errors returned by doi.org metadata retrieval.
#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum DoiOrgError {
    #[snafu(display("HTTP request failed at {stage}: {source}"))]
    Request {
        stage: &'static str,
        source: reqwest::Error,
    },

    #[snafu(display("Failed to serialize at {stage}: {source}"))]
    SerializePath {
        source: serde_path_to_error::Error<serde_json::Error>,
        stage: &'static str,
    },
}
