use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Flexible CSL value type used for ids, numbers, and booleans.
pub type CslValue = Value;

/// CSL-JSON item returned by doi.org content negotiation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CslMessage {
    /// Unique item identifier (string or number).
    #[serde(default)]
    pub id: Option<CslValue>,
    /// CSL item type (e.g. "article-journal").
    #[serde(rename = "type", default)]
    pub item_type: Option<String>,

    /// Optional citation key.
    pub citation_key: Option<String>,
    /// Optional item categories.
    pub categories: Option<Vec<String>>,
    /// Optional language code.
    pub language: Option<String>,
    /// Optional journal abbreviation (camelCase in CSL-JSON).
    #[serde(rename = "journalAbbreviation")]
    pub journal_abbreviation: Option<String>,
    /// Optional short title (camelCase in CSL-JSON).
    #[serde(rename = "shortTitle")]
    pub short_title: Option<CslValue>,
    /// Name variables.
    pub author: Option<Vec<CslName>>,
    pub chair: Option<Vec<CslName>>,
    pub collection_editor: Option<Vec<CslName>>,
    pub compiler: Option<Vec<CslName>>,
    pub composer: Option<Vec<CslName>>,
    pub container_author: Option<Vec<CslName>>,
    pub contributor: Option<Vec<CslName>>,
    pub curator: Option<Vec<CslName>>,
    pub director: Option<Vec<CslName>>,
    pub editor: Option<Vec<CslName>>,
    pub editorial_director: Option<Vec<CslName>>,
    pub executive_producer: Option<Vec<CslName>>,
    pub guest: Option<Vec<CslName>>,
    pub host: Option<Vec<CslName>>,
    pub interviewer: Option<Vec<CslName>>,
    pub illustrator: Option<Vec<CslName>>,
    pub narrator: Option<Vec<CslName>>,
    pub organizer: Option<Vec<CslName>>,
    pub original_author: Option<Vec<CslName>>,
    pub performer: Option<Vec<CslName>>,
    pub producer: Option<Vec<CslName>>,
    pub recipient: Option<Vec<CslName>>,
    pub reviewed_author: Option<Vec<CslName>>,
    pub script_writer: Option<Vec<CslName>>,
    pub series_creator: Option<Vec<CslName>>,
    pub translator: Option<Vec<CslName>>,

    /// Date variables.
    pub accessed: Option<CslDate>,
    pub available_date: Option<CslDate>,
    pub event_date: Option<CslDate>,
    pub issued: Option<CslDate>,
    pub original_date: Option<CslDate>,
    pub submitted: Option<CslDate>,

    /// Standard text/number fields.
    #[serde(rename = "abstract")]
    pub abstract_text: Option<String>,
    pub annote: Option<String>,
    pub archive: Option<String>,
    pub archive_collection: Option<String>,
    pub archive_location: Option<String>,
    pub archive_place: Option<String>,
    pub authority: Option<String>,
    pub call_number: Option<String>,
    pub chapter_number: Option<CslValue>,
    pub citation_number: Option<CslValue>,
    pub citation_label: Option<String>,
    pub collection_number: Option<CslValue>,
    pub collection_title: Option<CslValue>,
    pub container_title: Option<CslValue>,
    pub container_title_short: Option<CslValue>,
    pub dimensions: Option<String>,
    pub division: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    pub edition: Option<CslValue>,
    pub event: Option<String>,
    pub event_title: Option<String>,
    pub event_place: Option<String>,
    pub first_reference_note_number: Option<CslValue>,
    pub genre: Option<String>,
    #[serde(rename = "ISBN")]
    pub isbn: Option<CslValue>,
    #[serde(rename = "ISSN")]
    pub issn: Option<CslValue>,
    pub issue: Option<CslValue>,
    pub jurisdiction: Option<String>,
    pub keyword: Option<String>,
    pub locator: Option<CslValue>,
    pub medium: Option<String>,
    pub note: Option<String>,
    pub number: Option<CslValue>,
    pub number_of_pages: Option<CslValue>,
    pub number_of_volumes: Option<CslValue>,
    pub original_publisher: Option<String>,
    pub original_publisher_place: Option<String>,
    pub original_title: Option<CslValue>,
    pub page: Option<CslValue>,
    pub page_first: Option<CslValue>,
    pub part: Option<CslValue>,
    pub part_title: Option<String>,
    #[serde(rename = "PMCID")]
    pub pmcid: Option<String>,
    #[serde(rename = "PMID")]
    pub pmid: Option<String>,
    pub printing: Option<CslValue>,
    pub publisher: Option<String>,
    pub publisher_place: Option<String>,
    pub references: Option<String>,
    pub reviewed_genre: Option<CslValue>,
    pub reviewed_title: Option<CslValue>,
    pub scale: Option<String>,
    pub section: Option<String>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub supplement: Option<CslValue>,
    pub title: Option<CslValue>,
    pub title_short: Option<CslValue>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    pub version: Option<String>,
    pub volume: Option<CslValue>,
    pub volume_title: Option<CslValue>,
    pub volume_title_short: Option<CslValue>,
    pub year_suffix: Option<String>,

    /// Custom fields for extensions.
    pub custom: Option<BTreeMap<String, Value>>,
}

/// CSL-JSON name variable representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CslName {
    /// Family name.
    pub family: Option<String>,
    /// Given name.
    pub given: Option<String>,
    /// Dropping particle (e.g., "van").
    pub dropping_particle: Option<String>,
    /// Non-dropping particle (e.g., "de").
    pub non_dropping_particle: Option<String>,
    /// Name suffix.
    pub suffix: Option<String>,
    /// Comma suffix flag.
    pub comma_suffix: Option<CslValue>,
    /// Static ordering flag.
    pub static_ordering: Option<CslValue>,
    /// Literal name (institution or mononym).
    pub literal: Option<String>,
    /// Parse names flag.
    pub parse_names: Option<CslValue>,
}

/// CSL-JSON date variable representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CslDate {
    /// Date parts array, e.g. [[2024, 1, 5]].
    pub date_parts: Option<Vec<Vec<CslValue>>>,
    /// Season value for seasonal dates.
    pub season: Option<CslValue>,
    /// Circa indicator.
    pub circa: Option<CslValue>,
    /// Literal date string.
    pub literal: Option<String>,
    /// Raw date string.
    pub raw: Option<String>,
}
