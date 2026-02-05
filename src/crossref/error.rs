use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CrossrefError {
    #[error("missing mailto parameter for Crossref request")]
    MissingMailto,
    #[error("invalid Crossref response: {0}")]
    InvalidResponse(String),
    #[error("Crossref parse error: {0}")]
    Parse(String),
}
