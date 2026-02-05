use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossrefResponse {
    pub status: String,
    #[serde(rename = "message-type")]
    pub message_type: String,
    #[serde(rename = "message-version")]
    pub message_version: String,
    pub message: CrossrefMessage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CrossrefMessage {
    #[serde(default)]
    pub institution: Vec<WorkInstitution>,
    #[serde(default)]
    pub indexed: Option<DateAndVersion>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub posted: Option<DateParts>,
    #[serde(rename = "publisher-location", default)]
    pub publisher_location: Option<String>,
    #[serde(rename = "update-to", default)]
    pub update_to: Vec<WorkUpdate>,
    #[serde(rename = "standards-body", default)]
    pub standards_body: Option<WorkStandardsBody>,
    #[serde(rename = "edition-number", default)]
    pub edition_number: Option<String>,
    #[serde(rename = "group-title", default)]
    pub group_title: Option<String>,
    #[serde(rename = "reference-count", default)]
    pub reference_count: Option<i64>,
    #[serde(default)]
    pub publisher: Option<String>,
    #[serde(default)]
    pub issue: Option<String>,
    #[serde(rename = "isbn-type", default)]
    pub isbn_type: Vec<WorkISSNType>,
    #[serde(default)]
    pub license: Vec<WorkLicense>,
    #[serde(default)]
    pub funder: Vec<WorkFunder>,
    #[serde(rename = "content-domain", default)]
    pub content_domain: Option<WorkDomain>,
    #[serde(default)]
    pub chair: Vec<Author>,
    #[serde(rename = "short-container-title", default)]
    pub short_container_title: Vec<String>,
    #[serde(default)]
    pub accepted: Option<DateParts>,
    #[serde(rename = "special-numbering", default)]
    pub special_numbering: Option<String>,
    #[serde(rename = "content-updated", default)]
    pub content_updated: Option<DateParts>,
    #[serde(rename = "published-print", default)]
    pub published_print: Option<DateParts>,
    #[serde(rename = "abstract", default)]
    pub abstract_text: Option<String>,
    #[serde(rename = "DOI", default)]
    pub doi: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub created: Option<Date>,
    #[serde(default)]
    pub approved: Option<DateParts>,
    #[serde(default)]
    pub page: Option<String>,
    #[serde(rename = "update-policy", default)]
    pub update_policy: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(rename = "is-referenced-by-count", default)]
    pub is_referenced_by_count: Option<i64>,
    #[serde(default)]
    pub title: Vec<String>,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(rename = "clinical-trial-number", default)]
    pub clinical_trial_number: Vec<WorkClinicalTrial>,
    #[serde(default)]
    pub author: Vec<Author>,
    #[serde(default)]
    pub member: Option<String>,
    #[serde(rename = "content-created", default)]
    pub content_created: Option<DateParts>,
    #[serde(rename = "published-online", default)]
    pub published_online: Option<DateParts>,
    #[serde(default)]
    pub reference: Vec<Reference>,
    #[serde(rename = "updated-by", default)]
    pub updated_by: Vec<WorkUpdate>,
    #[serde(default)]
    pub event: Option<WorkEvent>,
    #[serde(rename = "container-title", default)]
    pub container_title: Vec<String>,
    #[serde(default)]
    pub review: Option<WorkReview>,
    #[serde(default)]
    pub project: Vec<WorkProject>,
    #[serde(rename = "original-title", default)]
    pub original_title: Vec<String>,
    #[serde(default)]
    pub status: Option<PostedContentStatus>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub link: Vec<WorkLink>,
    #[serde(default)]
    pub deposited: Option<Date>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub resource: Option<Resources>,
    #[serde(default)]
    pub subtitle: Vec<String>,
    #[serde(rename = "short-title", default)]
    pub short_title: Vec<String>,
    #[serde(default)]
    pub issued: Option<DateParts>,
    #[serde(rename = "references-count", default)]
    pub references_count: Option<i64>,
    #[serde(rename = "journal-issue", default)]
    pub journal_issue: Option<WorkJournalIssue>,
    #[serde(rename = "URL", default)]
    pub url: Option<String>,
    #[serde(default)]
    pub relation: WorkRelation,
    #[serde(rename = "ISSN", default)]
    pub issn: Vec<String>,
    #[serde(rename = "issn-type", default)]
    pub issn_type: Vec<WorkISSNType>,
    #[serde(default)]
    pub subject: Vec<String>,
    #[serde(default)]
    pub published: Option<DateParts>,
    #[serde(default)]
    pub assertion: Vec<WorkAssertion>,
    #[serde(rename = "ISBN", default)]
    pub isbn: Vec<String>,
    #[serde(rename = "issue-title", default)]
    pub issue_title: Vec<String>,
    #[serde(rename = "aliases", default)]
    pub aliases: Vec<String>,
    #[serde(rename = "alternative-id", default)]
    pub alternative_id: Vec<String>,
    #[serde(rename = "archive", default)]
    pub archive: Vec<String>,
    #[serde(rename = "article-number", default)]
    pub article_number: Option<String>,
    #[serde(default)]
    pub editor: Vec<Author>,
    #[serde(rename = "free-to-read", default)]
    pub free_to_read: Option<WorkFreeToRead>,
    #[serde(rename = "part-number", default)]
    pub part_number: Option<String>,
    #[serde(rename = "proceedings-subject", default)]
    pub proceedings_subject: Option<String>,
    #[serde(rename = "published-other", default)]
    pub published_other: Option<DateParts>,
    #[serde(default)]
    pub translator: Vec<Author>,
    #[serde(default)]
    pub subtype: Option<String>,
    #[serde(default)]
    pub version: Option<VersionInfo>,
    #[serde(rename = "component-number", default)]
    pub component_number: Option<String>,
    #[serde(default)]
    pub degree: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DateParts {
    #[serde(rename = "date-parts", default)]
    pub date_parts: Vec<Vec<Option<i64>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Date {
    #[serde(rename = "date-parts", default)]
    pub date_parts: Vec<Vec<i64>>,
    #[serde(rename = "date-time", default)]
    pub date_time: Option<String>,
    #[serde(default)]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DateAndVersion {
    #[serde(rename = "date-parts", default)]
    pub date_parts: Vec<Vec<i64>>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(rename = "date-time", default)]
    pub date_time: Option<String>,
    #[serde(default)]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkAssertion {
    #[serde(default)]
    pub group: Option<WorkAssertionGroup>,
    #[serde(default)]
    pub explanation: Option<WorkAssertionExplanation>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "URL", default)]
    pub url: Option<String>,
    #[serde(default)]
    pub order: Option<i64>,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkAssertionGroup {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkAssertionExplanation {
    #[serde(rename = "URL", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Author {
    #[serde(rename = "ORCID", default)]
    pub orcid: Option<String>,
    #[serde(default)]
    pub suffix: Option<String>,
    #[serde(default)]
    pub given: Option<String>,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub affiliation: Vec<Affiliation>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "authenticated-orcid", default)]
    pub authenticated_orcid: Option<bool>,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub sequence: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Affiliation {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub place: Vec<String>,
    #[serde(default)]
    pub department: Vec<String>,
    #[serde(default)]
    pub acronym: Vec<String>,
    #[serde(default)]
    pub id: Vec<AffiliationIdentifier>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AffiliationIdentifier {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "id-type", default)]
    pub id_type: Option<String>,
    #[serde(rename = "asserted-by", default)]
    pub asserted_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkClinicalTrial {
    #[serde(rename = "clinical-trial-number", default)]
    pub clinical_trial_number: Option<String>,
    #[serde(default)]
    pub registry: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkDomain {
    #[serde(default)]
    pub domain: Vec<String>,
    #[serde(rename = "crossmark-restriction", default)]
    pub crossmark_restriction: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkEvent {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub start: Option<DateParts>,
    #[serde(default)]
    pub end: Option<DateParts>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkFreeToRead {
    #[serde(rename = "start-date", default)]
    pub start_date: Option<DateParts>,
    #[serde(rename = "end-date", default)]
    pub end_date: Option<DateParts>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkFunder {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "DOI", default)]
    pub doi: Option<String>,
    #[serde(rename = "doi-asserted-by", default)]
    pub doi_asserted_by: Option<String>,
    #[serde(default)]
    pub award: Vec<String>,
    #[serde(rename = "award-info", default)]
    pub award_info: Vec<WorkFunderAwardInfo>,
    #[serde(default)]
    pub id: Vec<FunderIdentifier>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkFunderAwardInfo {
    #[serde(rename = "award-number", default)]
    pub award_number: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FunderIdentifier {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "id-type", default)]
    pub id_type: Option<String>,
    #[serde(rename = "asserted-by", default)]
    pub asserted_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkInstitution {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub place: Vec<String>,
    #[serde(default)]
    pub department: Vec<String>,
    #[serde(default)]
    pub acronym: Vec<String>,
    #[serde(default)]
    pub id: Vec<AffiliationIdentifier>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkISSNType {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkJournalIssue {
    #[serde(default)]
    pub issue: Option<String>,
    #[serde(rename = "published-online", default)]
    pub published_online: Option<DateParts>,
    #[serde(rename = "published-print", default)]
    pub published_print: Option<DateParts>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkLicense {
    #[serde(rename = "URL", default)]
    pub url: Option<String>,
    #[serde(default)]
    pub start: Option<Date>,
    #[serde(rename = "delay-in-days", default)]
    pub delay_in_days: Option<i64>,
    #[serde(rename = "content-version", default)]
    pub content_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkLink {
    #[serde(rename = "URL", default)]
    pub url: Option<String>,
    #[serde(rename = "content-type", default)]
    pub content_type: Option<String>,
    #[serde(rename = "content-version", default)]
    pub content_version: Option<String>,
    #[serde(rename = "intended-application", default)]
    pub intended_application: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkProject {
    #[serde(rename = "award-end", default)]
    pub award_end: Vec<DateParts>,
    #[serde(rename = "award-planned-start", default)]
    pub award_planned_start: Vec<DateParts>,
    #[serde(rename = "award-start", default)]
    pub award_start: Vec<DateParts>,
    #[serde(rename = "lead-investigator", default)]
    pub lead_investigator: Vec<Investigator>,
    #[serde(rename = "award-planned-end", default)]
    pub award_planned_end: Vec<DateParts>,
    #[serde(default)]
    pub investigator: Vec<Investigator>,
    #[serde(default)]
    pub funding: Vec<Funding>,
    #[serde(rename = "project-title", default)]
    pub project_title: Vec<ProjectTitle>,
    #[serde(rename = "award-amount", default)]
    pub award_amount: Option<AwardAmount>,
    #[serde(rename = "co-lead-investigator", default)]
    pub co_lead_investigator: Vec<Investigator>,
    #[serde(rename = "project-description", default)]
    pub project_description: Vec<WorksProjectDescription>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Investigator {
    #[serde(rename = "ORCID", default)]
    pub orcid: Option<String>,
    #[serde(default)]
    pub suffix: Option<String>,
    #[serde(default)]
    pub given: Option<String>,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub affiliation: Vec<InvestigatorAffiliation>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "role-start", default)]
    pub role_start: Option<DateParts>,
    #[serde(rename = "authenticated-orcid", default)]
    pub authenticated_orcid: Option<bool>,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(rename = "alternate-name", default)]
    pub alternate_name: Option<String>,
    #[serde(default)]
    pub sequence: Option<String>,
    #[serde(rename = "role-end", default)]
    pub role_end: Option<DateParts>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InvestigatorAffiliation {
    #[serde(default)]
    pub id: Vec<AffiliationIdentifier>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Funding {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub scheme: Option<String>,
    #[serde(rename = "award-amount", default)]
    pub award_amount: Option<AwardAmount>,
    #[serde(default)]
    pub funder: Option<WorkFunder>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectTitle {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AwardAmount {
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub percentage: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorksProjectDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Reference {
    #[serde(default)]
    pub issn: Option<String>,
    #[serde(rename = "standards-body", default)]
    pub standards_body: Option<String>,
    #[serde(default)]
    pub issue: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(rename = "series-title", default)]
    pub series_title: Option<String>,
    #[serde(rename = "isbn-type", default)]
    pub isbn_type: Option<String>,
    #[serde(rename = "doi-asserted-by", default)]
    pub doi_asserted_by: Option<String>,
    #[serde(rename = "first-page", default)]
    pub first_page: Option<String>,
    #[serde(rename = "DOI", default)]
    pub doi: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub isbn: Option<String>,
    #[serde(default)]
    pub component: Option<String>,
    #[serde(rename = "article-title", default)]
    pub article_title: Option<String>,
    #[serde(rename = "volume-title", default)]
    pub volume_title: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(rename = "standard-designator", default)]
    pub standard_designator: Option<String>,
    #[serde(default)]
    pub year: Option<String>,
    #[serde(default)]
    pub unstructured: Option<String>,
    #[serde(default)]
    pub edition: Option<String>,
    #[serde(rename = "journal-title", default)]
    pub journal_title: Option<String>,
    #[serde(rename = "issn-type", default)]
    pub issn_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkRelation {
    #[serde(flatten, default)]
    pub relations: BTreeMap<String, Vec<WorkRelationObject>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkRelationObject {
    #[serde(rename = "id-type", default)]
    pub id_type: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "asserted-by", default)]
    pub asserted_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Resources {
    #[serde(default)]
    pub primary: Option<PrimaryResource>,
    #[serde(default)]
    pub secondary: Vec<SecondaryResource>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PrimaryResource {
    #[serde(rename = "URL", default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SecondaryResource {
    #[serde(rename = "URL", default)]
    pub url: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkReview {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(rename = "running-number", default)]
    pub running_number: Option<String>,
    #[serde(rename = "revision-round", default)]
    pub revision_round: Option<String>,
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(rename = "competing-interest-statement", default)]
    pub competing_interest_statement: Option<String>,
    #[serde(default)]
    pub recommendation: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkStandardsBody {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub acronym: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PostedContentStatus {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub update: Option<DateParts>,
    #[serde(rename = "status-description", default)]
    pub status_description: Vec<StatusDescription>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusDescription {
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkUpdate {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(rename = "DOI", default)]
    pub doi: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub updated: Option<Date>,
    #[serde(rename = "record-id", default)]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionInfo {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(rename = "version-description", default)]
    pub version_description: Vec<VersionInfoDescription>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionInfoDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
}
