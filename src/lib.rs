//! DOI (Digital Object Identifier) parsing and normalization library

pub mod csl;
pub mod doi_org;
pub mod parse;

pub use csl::*;
pub use doi_org::client::DoiOrgClient;
pub use doi_org::config::DoiOrgConfig;
pub use doi_org::error::DoiOrgError;

pub use parse::Doi;
pub use parse::DoiParseError;
pub use parse::extract_doi_from_url;
