//! DOI (Digital Object Identifier) parsing and normalization library

use regex::Regex;
use snafu::{Snafu, ensure};
use std::borrow::Cow;
use std::str::FromStr;
use std::sync::LazyLock;

/// Errors returned when parsing a DOI from a string.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum DoiParseError {
    #[snafu(display("Invalid DOI in input at {stage}: {input}"))]
    InvalidDoi { stage: &'static str, input: String },
}

/// A parsed DOI containing the extracted DOI string
#[derive(Debug, Clone, PartialEq)]
pub struct Doi {
    /// The DOI as extracted from input
    pub value: String,
}

impl Doi {
    /// Create a new DOI from an extracted string
    fn new(extracted: &str) -> Self {
        Self {
            value: extracted.to_string(),
        }
    }

    /// Return the DOI as a string slice
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Return the DOI prefix portion (e.g. "10.1000")
    pub fn prefix(&self) -> Option<&str> {
        let (prefix, _) = self.value.split_once('/')?;
        if prefix.starts_with("10.") && prefix.len() > 3 {
            Some(prefix)
        } else {
            None
        }
    }

    /// Return the registrant number from the DOI prefix (e.g. "1000")
    pub fn registrant_number(&self) -> Option<&str> {
        let prefix = self.prefix()?;
        let registrant = &prefix[3..];
        if !registrant.is_empty() && registrant.chars().all(|c| c.is_ascii_digit()) {
            Some(registrant)
        } else {
            None
        }
    }

    /// Parse a DOI from input text, returning a typed error on failure.
    pub fn parse(input: &str) -> Result<Self, DoiParseError> {
        // Avoid returning a generic error for empty input.
        ensure!(
            !input.trim().is_empty(),
            InvalidDoiSnafu {
                stage: "parse-input",
                input: input.to_string(),
            }
        );

        extract_doi_from_url(input).ok_or_else(|| DoiParseError::InvalidDoi {
            stage: "extract-doi",
            input: input.to_string(),
        })
    }
}

impl FromStr for Doi {
    type Err = DoiParseError;

    /// Parse a DOI using the same logic as `Doi::parse`.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Doi::parse(input)
    }
}

/// Extract DOI from a URL or string
///
/// # Algorithm
/// 1. Search for DOI pattern `10.\d+/.+` anywhere in the string
/// 2. If no match, percent-decode the URL and retry
/// 3. If multiple matches, choose the first
/// 4. Strip trailing punctuation: `. , ; : ) ] }`
/// 5. Return the extracted DOI as-is
///
/// Returns `None` if no DOI pattern is found.
pub fn extract_doi_from_url(input: &str) -> Option<Doi> {
    if input.is_empty() {
        return None;
    }

    // Try to find DOI in the original string
    if let Some(doi) = find_doi(input) {
        return Some(doi);
    }

    // Try to derive DOI from an arXiv identifier in the original string
    if let Some(doi) = find_arxiv_doi(input) {
        return Some(doi);
    }

    // If no match, try percent-decoding and search again
    let decoded = percent_decode(input);
    if decoded != input
        && let Some(doi) = find_doi(&decoded)
    {
        return Some(doi);
    }

    if decoded != input
        && let Some(doi) = find_arxiv_doi(&decoded)
    {
        return Some(doi);
    }

    None
}

/// Static regex for DOI pattern matching
/// Pattern: `10.\d+/[^/]+` - matches "10." followed by digits, then "/", then a single-path segment
/// We stop at whitespace or URL delimiters to extract just the DOI portion
static DOI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"10\.\d+/[^\s?#&=/]+").unwrap());

/// Static regex for arXiv identifier matching (new-style ids only)
/// Matches: arXiv:2101.12345, arxiv.org/abs/2101.12345v2, arxiv.org/pdf/2101.12345.pdf
static ARXIV_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:arxiv:|arxiv\.org/(?:abs|pdf)/)(\d{4}\.\d{4,5})(?:v\d+)?").unwrap()
});

/// Find DOI pattern in a string using strict regex `10.\d+/.+`
/// Returns the first match with trailing punctuation stripped
fn find_doi(input: &str) -> Option<Doi> {
    // Find the first match of the DOI pattern
    if let Some(mat) = DOI_REGEX.find(input) {
        let matched = mat.as_str();

        // Strip trailing punctuation and common file suffixes from the matched DOI
        let mut end = strip_trailing_punctuation(matched);
        end = strip_trailing_file_suffix(matched, end);

        if end > "10.0/".len() {
            // Ensure we have at least "10." + digit + "/" + something
            let extracted = &matched[..end];
            return Some(Doi::new(extracted));
        }
    }

    None
}

/// Find arXiv identifier and derive the corresponding DOI.
fn find_arxiv_doi(input: &str) -> Option<Doi> {
    if let Some(caps) = ARXIV_REGEX.captures(input) {
        // Use the canonical arXiv DOI prefix with the extracted id.
        if let Some(arxiv_id) = caps.get(1) {
            let doi = format!("10.48550/arXiv.{}", arxiv_id.as_str());
            return Some(Doi::new(&doi));
        }
    }

    None
}

/// Strip trailing punctuation from a DOI string
/// Returns the new length after stripping punctuation: `. , ; : ) ] }`
fn strip_trailing_punctuation(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut end = bytes.len();

    while end > 0 {
        let c = bytes[end - 1] as char;
        if matches!(c, '.' | ',' | ';' | ':' | ')' | ']' | '}') {
            end -= 1;
        } else {
            break;
        }
    }

    end
}

/// Strip common file suffixes from a DOI string
/// Returns the new length after stripping suffixes like ".pdf" or "/pdf".
fn strip_trailing_file_suffix(s: &str, end: usize) -> usize {
    let trimmed = &s[..end];
    if ends_with_ascii_case_insensitive(trimmed, ".pdf")
        || ends_with_ascii_case_insensitive(trimmed, "/pdf")
    {
        end.saturating_sub(4)
    } else {
        end
    }
}

/// Case-insensitive ASCII suffix check
fn ends_with_ascii_case_insensitive(value: &str, suffix: &str) -> bool {
    if value.len() < suffix.len() {
        return false;
    }
    let start = value.len() - suffix.len();
    value[start..]
        .bytes()
        .zip(suffix.bytes())
        .all(|(left, right)| left.to_ascii_lowercase() == right)
}

/// Percent-decode a URL string
fn percent_decode(input: &str) -> Cow<'_, str> {
    let mut result = String::new();
    let mut changed = false;
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = &input[i + 1..i + 3];
            if let Ok(byte) = u8::from_str_radix(hex, 16) {
                result.push(byte as char);
                i += 3;
                changed = true;
                continue;
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }

    if changed {
        Cow::Owned(result)
    } else {
        Cow::Borrowed(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Extracts DOI from a basic doi.org URL.
    fn doi_extract_simple() {
        let url = "https://doi.org/10.1000/182";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Extracts DOI when embedded in surrounding text.
    fn doi_extract_with_text() {
        let text = "See paper at 10.1000/182 for details";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Preserves suffix case when DOI prefix is uppercase.
    fn doi_extract_uppercase_prefix() {
        let url = "https://doi.org/10.1000/ABC123";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/ABC123");
    }

    #[test]
    /// Ensures mixed-case DOI suffix is preserved.
    fn doi_extract_mixed_case() {
        let url = "https://doi.org/10.1000/AbC123";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/AbC123");
    }

    #[test]
    /// Strips trailing period after DOI.
    fn doi_extract_trailing_punctuation() {
        let text = "Reference: 10.1000/182.";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Strips trailing comma after DOI.
    fn doi_extract_trailing_comma() {
        let text = "See 10.1000/182, and more";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Strips trailing semicolon after DOI.
    fn doi_extract_trailing_semicolon() {
        let text = "Cite: 10.1000/182;";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Strips trailing colon after DOI.
    fn doi_extract_trailing_colon() {
        let text = "DOI: 10.1000/182:";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Strips trailing right parenthesis after DOI.
    fn doi_extract_trailing_paren() {
        let text = "(see 10.1000/182)";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Strips trailing right bracket after DOI.
    fn doi_extract_trailing_bracket() {
        let text = "[10.1000/182]";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Strips trailing right brace after DOI.
    fn doi_extract_trailing_brace() {
        let text = "{10.1000/182}";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Strips multiple trailing punctuation characters.
    fn doi_extract_multiple_punctuation() {
        let text = "Ref: 10.1000/182.).";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Decodes percent-encoded DOI path separator.
    fn doi_extract_percent_encoded() {
        let url = "https://doi.org/10.1000%2F182";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Decodes percent-encoded DOI within a longer path.
    fn doi_extract_percent_encoded_with_suffix() {
        let url = "https://example.com/paper/10.1000%2Fabc123";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/abc123");
    }

    #[test]
    /// Uses the first DOI when multiple are present.
    fn doi_extract_first_match_wins() {
        let text = "Papers: 10.1000/111 and 10.1000/222";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/111");
    }

    #[test]
    /// Returns None when no DOI appears in input.
    fn doi_extract_not_found() {
        let text = "No DOI here";
        let result = extract_doi_from_url(text);
        assert!(result.is_none());
    }

    #[test]
    /// Returns None for inputs without a registrant number.
    fn doi_extract_invalid_no_registrant() {
        let text = "Invalid: 10.abc/123";
        let result = extract_doi_from_url(text);
        assert!(result.is_none());
    }

    #[test]
    /// Returns None for inputs missing a DOI suffix.
    fn doi_extract_invalid_no_suffix() {
        let text = "Invalid: 10.1000/";
        let result = extract_doi_from_url(text);
        assert!(result.is_none());
    }

    #[test]
    /// Returns None for empty input strings.
    fn doi_extract_empty_input() {
        let result = extract_doi_from_url("");
        assert!(result.is_none());
    }

    #[test]
    /// Extracts DOI with complex, dotted suffix.
    fn doi_extract_complex_suffix() {
        let url = "https://doi.org/10.1016/j.cell.2021.01.001";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1016/j.cell.2021.01.001");
    }

    #[test]
    /// Ignores query parameters after DOI.
    fn doi_extract_with_query_params() {
        let url = "https://doi.org/10.1000/182?foo=bar";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Ignores URL fragments after DOI.
    fn doi_extract_with_fragment() {
        let url = "https://doi.org/10.1000/182#section1";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Extracts DOI when enclosed in parentheses.
    fn doi_extract_in_parentheses() {
        let text = "Smith et al. (10.1000/182) found that...";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Extracts DOI with long suffix and dots.
    fn doi_extract_long_suffix() {
        let url = "https://doi.org/10.1234/very.long.suffix.with.dots";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1234/very.long.suffix.with.dots");
    }

    #[test]
    /// Extracts DOI containing allowed special characters.
    fn doi_extract_with_special_chars() {
        let url = "https://doi.org/10.1000/abc-def_ghi";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/abc-def_ghi");
    }

    #[test]
    /// Extracts DOI from real-world article URLs.
    fn doi_extract_real_world_links() {
        let frontiers = "https://www.frontiersin.org/journals/microbiology/articles/10.3389/fmicb.2017.01663/pdf ";
        let springer = "https://link.springer.com/chapter/10.1007/978-0-387-74907-5_34 ";
        let tykx =
            "http://tykx.xml-journal.net/cn/article/pdf/preview/10.16469/j.css.2011.06.015.pdf";
        let taylor = "https://www.taylorfrancis.com/chapters/edit/10.4324/9781351254762-9/anatomy-restlessness-megan-perry ";

        let doi = extract_doi_from_url(frontiers).unwrap();
        assert_eq!(doi.value, "10.3389/fmicb.2017.01663");

        let doi = extract_doi_from_url(springer).unwrap();
        assert_eq!(doi.value, "10.1007/978-0-387-74907-5_34");

        let doi = extract_doi_from_url(tykx).unwrap();
        assert_eq!(doi.value, "10.16469/j.css.2011.06.015");

        let doi = extract_doi_from_url(taylor).unwrap();
        assert_eq!(doi.value, "10.4324/9781351254762-9");
    }

    #[test]
    /// Extracts registrant number from the DOI prefix.
    fn doi_extract_registrant() {
        let url = "https://doi.org/10.1000/abc-def_ghi";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.registrant_number(), Some("1000"));
    }

    #[test]
    /// Returns the registrant number without the "10." prefix.
    fn doi_extract_registrant_number() {
        let url = "https://doi.org/10.5281/zenodo.123";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.registrant_number(), Some("5281"));
    }

    #[test]
    /// Handles uppercase "10." prefix case explicitly.
    fn doi_extract_uppercase_10() {
        let text = "DOI: 10.1000/123"; // 10. is already lowercase in input
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.1000/123");
    }

    #[test]
    /// Extracts DOI embedded in a longer URL path.
    fn doi_extract_doi_in_middle_of_path() {
        let url = "https://example.com/papers/10.1000/182/download";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/182");
    }

    #[test]
    /// Extracts DOI with only one slash in the suffix.
    fn doi_extract_with_multiple_slashes_in_suffix() {
        let url = "https://doi.org/10.1000/a/b/c";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/a");
    }

    #[test]
    /// Derives DOI from an arXiv abstract URL.
    fn doi_extract_from_arxiv_abs_url() {
        let url = "https://arxiv.org/abs/2101.12345v2";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.48550/arXiv.2101.12345");
    }

    #[test]
    /// Derives DOI from an arXiv PDF URL.
    fn doi_extract_from_arxiv_pdf_url() {
        let url = "https://arxiv.org/pdf/2101.12345.pdf";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.48550/arXiv.2101.12345");
    }

    #[test]
    /// Derives DOI from an arXiv identifier string.
    fn doi_extract_from_arxiv_id() {
        let text = "arXiv:2101.12345";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.value, "10.48550/arXiv.2101.12345");
    }

    #[test]
    /// Decodes multiple percent-encoded path separators.
    fn doi_extract_percent_encoded_multiple() {
        let url = "https://example.com/10.1000%2Fabc%2Fdef";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.value, "10.1000/abc");
    }
}
