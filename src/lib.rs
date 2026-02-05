//! DOI (Digital Object Identifier) parsing and normalization library

pub mod crossref;
pub mod parse;

pub use crossref::client::CrossrefClient;
pub use crossref::config::CrossrefConfig;
pub use crossref::error::CrossrefError;
pub use crossref::models::CrossrefResponse;

pub use parse::Doi;
pub use parse::extract_doi_from_url;
