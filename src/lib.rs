//! DOI (Digital Object Identifier) parsing and normalization library

use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;
use thiserror::Error;

pub mod crossref;

pub use crossref::client::CrossrefClient;
pub use crossref::config::CrossrefConfig;
pub use crossref::error::CrossrefError;
pub use crossref::models::CrossrefResponse;

/// Errors that can occur during DOI extraction
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DoiError {
    /// No DOI found in the input string
    #[error("no DOI found in input")]
    NotFound,
    /// Invalid input provided
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// A parsed DOI with both original and canonical forms
#[derive(Debug, Clone, PartialEq)]
pub struct Doi {
    /// The original DOI as extracted (preserves case)
    pub original: String,
    /// The canonical form (lowercase prefix only)
    pub canonical: String,
}

impl Doi {
    /// Create a new DOI from an extracted string
    fn new(extracted: &str) -> Self {
        let original = extracted.to_string();

        // Canonicalize: lowercase the "10." prefix only, preserve suffix case
        let canonical = if extracted.len() >= 3 {
            let prefix = &extracted[..3];
            let suffix = &extracted[3..];
            format!("{}{}", prefix.to_lowercase(), suffix)
        } else {
            extracted.to_lowercase()
        };

        Self {
            original,
            canonical,
        }
    }
}

/// Extract DOI from a URL or string
///
/// # Algorithm
/// 1. Search for DOI pattern `10.\d+/.+` anywhere in the string
/// 2. If no match, percent-decode the URL and retry
/// 3. If multiple matches, choose the first
/// 4. Strip trailing punctuation: `. , ; : ) ] }`
/// 5. Canonicalize: lowercase prefix only
///
/// # Errors
/// Returns `DoiError::NotFound` if no DOI pattern is found
pub fn extract_doi_from_url(input: &str) -> Result<Doi, DoiError> {
    if input.is_empty() {
        return Err(DoiError::InvalidInput("empty string".to_string()));
    }

    // Try to find DOI in the original string
    if let Some(doi) = find_doi(input) {
        return Ok(doi);
    }

    // If no match, try percent-decoding and search again
    let decoded = percent_decode(input);
    if decoded != input
        && let Some(doi) = find_doi(&decoded) {
            return Ok(doi);
        }

    Err(DoiError::NotFound)
}

/// Static regex for DOI pattern matching
/// Pattern: `10.\d+/.+` - matches "10." followed by digits, then "/", then any characters
/// We stop at whitespace or URL delimiters to extract just the DOI portion
static DOI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"10\.\d+/[^\s?#&=]+").unwrap());

/// Find DOI pattern in a string using strict regex `10.\d+/.+`
/// Returns the first match with trailing punctuation stripped
fn find_doi(input: &str) -> Option<Doi> {
    // Find the first match of the DOI pattern
    if let Some(mat) = DOI_REGEX.find(input) {
        let matched = mat.as_str();

        // Strip trailing punctuation from the matched DOI
        let end = strip_trailing_punctuation(matched);

        if end > "10.0/".len() {
            // Ensure we have at least "10." + digit + "/" + something
            let extracted = &matched[..end];
            return Some(Doi::new(extracted));
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
    fn doi_extract_simple() {
        let url = "https://doi.org/10.1000/182";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/182");
        assert_eq!(doi.canonical, "10.1000/182");
    }

    #[test]
    fn doi_extract_with_text() {
        let text = "See paper at 10.1000/182 for details";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_uppercase_prefix() {
        let url = "https://doi.org/10.1000/ABC123";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/ABC123");
        assert_eq!(doi.canonical, "10.1000/ABC123"); // Prefix lowercased, suffix preserved
    }

    #[test]
    fn doi_extract_mixed_case() {
        let url = "https://doi.org/10.1000/AbC123";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/AbC123");
        assert_eq!(doi.canonical, "10.1000/AbC123"); // Only 10. is lowercased, which it already is
    }

    #[test]
    fn doi_extract_trailing_punctuation() {
        let text = "Reference: 10.1000/182.";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_trailing_comma() {
        let text = "See 10.1000/182, and more";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_trailing_semicolon() {
        let text = "Cite: 10.1000/182;";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_trailing_colon() {
        let text = "DOI: 10.1000/182:";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_trailing_paren() {
        let text = "(see 10.1000/182)";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_trailing_bracket() {
        let text = "[10.1000/182]";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_trailing_brace() {
        let text = "{10.1000/182}";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_multiple_punctuation() {
        let text = "Ref: 10.1000/182.).";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_percent_encoded() {
        let url = "https://doi.org/10.1000%2F182";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_percent_encoded_with_suffix() {
        let url = "https://example.com/paper/10.1000%2Fabc123";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/abc123");
    }

    #[test]
    fn doi_extract_first_match_wins() {
        let text = "Papers: 10.1000/111 and 10.1000/222";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/111");
    }

    #[test]
    fn doi_extract_not_found() {
        let text = "No DOI here";
        let result = extract_doi_from_url(text);
        assert!(matches!(result, Err(DoiError::NotFound)));
    }

    #[test]
    fn doi_extract_invalid_no_registrant() {
        let text = "Invalid: 10.abc/123";
        let result = extract_doi_from_url(text);
        assert!(matches!(result, Err(DoiError::NotFound)));
    }

    #[test]
    fn doi_extract_invalid_no_suffix() {
        let text = "Invalid: 10.1000/";
        let result = extract_doi_from_url(text);
        assert!(matches!(result, Err(DoiError::NotFound)));
    }

    #[test]
    fn doi_extract_empty_input() {
        let result = extract_doi_from_url("");
        assert!(matches!(result, Err(DoiError::InvalidInput(_))));
    }

    #[test]
    fn doi_extract_complex_suffix() {
        let url = "https://doi.org/10.1016/j.cell.2021.01.001";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1016/j.cell.2021.01.001");
        assert_eq!(doi.canonical, "10.1016/j.cell.2021.01.001");
    }

    #[test]
    fn doi_extract_with_query_params() {
        let url = "https://doi.org/10.1000/182?foo=bar";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_with_fragment() {
        let url = "https://doi.org/10.1000/182#section1";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_in_parentheses() {
        let text = "Smith et al. (10.1000/182) found that...";
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/182");
    }

    #[test]
    fn doi_extract_long_suffix() {
        let url = "https://doi.org/10.1234/very.long.suffix.with.dots";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1234/very.long.suffix.with.dots");
    }

    #[test]
    fn doi_extract_with_special_chars() {
        let url = "https://doi.org/10.1000/abc-def_ghi";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/abc-def_ghi");
    }

    #[test]
    fn doi_extract_canonicalization_preserves_suffix_case() {
        let url = "https://doi.org/10.1000/ABC-xyz";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/ABC-xyz");
        assert_eq!(doi.canonical, "10.1000/ABC-xyz"); // 10. already lowercase, suffix preserved
    }

    #[test]
    fn doi_extract_uppercase_10() {
        let text = "DOI: 10.1000/123"; // 10. is already lowercase in input
        let doi = extract_doi_from_url(text).unwrap();
        assert_eq!(doi.original, "10.1000/123");
        assert_eq!(doi.canonical, "10.1000/123");
    }

    #[test]
    fn doi_extract_doi_in_middle_of_path() {
        let url = "https://example.com/papers/10.1000/182/download";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/182/download");
    }

    #[test]
    fn doi_extract_with_multiple_slashes_in_suffix() {
        let url = "https://doi.org/10.1000/a/b/c";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/a/b/c");
    }

    #[test]
    fn doi_extract_percent_encoded_multiple() {
        let url = "https://example.com/10.1000%2Fabc%2Fdef";
        let doi = extract_doi_from_url(url).unwrap();
        assert_eq!(doi.original, "10.1000/abc/def");
    }
}
