use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::csl::value::{ClsDatePart, CslValue};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DoiMetadata {
    /// The DOI identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// doi item type
    #[serde(rename = "type")]
    pub item_type: String,
    /// doi item categories
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    /// doi item publisher
    pub publisher: String,
    /// doi item issued date
    pub issued: Issued,
    /// doi item doi
    #[serde(rename = "DOI")]
    pub doi: String,
    /// doi item title
    pub title: String,
    /// doi item language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// doi item abstract
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    /// doi item journal
    #[serde(rename = "URL")]
    pub url: String,
    /// doi item author
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<Author>,
    /// doi item issn
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issn: Vec<String>,
    /// doi item volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<CslValue>,
    /// doi item reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<Reference>,
    /// doi item issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<CslValue>,
    /// doi metadata source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// doi item reference count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_count: Option<usize>,
    /// doi item referenced by count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_referenced_by_count: Option<usize>,
    /// doi item content domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_domain: Option<ContentDomain>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Issued {
    pub date_parts: ClsDatePart,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Author {
    pub given: String,
    pub family: String,
    #[serde(rename = "ORCID", skip_serializing_if = "Option::is_none")]
    pub orcid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affiliation: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JournalIssue {
    pub issue: String,
    pub published_print: PublishedPrint,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PublishedPrint {
    pub date_parts: ClsDatePart,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Reference {
    pub key: String,
    #[serde(rename = "DOI", skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<CslValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<CslValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContentDomain {
    pub domain: Vec<String>,
}
