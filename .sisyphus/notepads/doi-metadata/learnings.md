# Learnings

## Notes
- (empty)

## DOI Extraction Implementation (2026-02-05)

### Algorithm Design
- DOI pattern matching uses strict regex `10.\d+/.+` implemented as a state machine
- First match wins when multiple DOIs present in input
- Percent-decoding fallback for URL-encoded DOIs (e.g., `10.1000%2F182`)

### Canonicalization Rules
- Only the `10.` prefix is lowercased; suffix case is preserved
- This matches CrossRef and DataCite normalization practices
- Example: `10.1000/ABC123` → canonical: `10.1000/ABC123`

### Trailing Punctuation Stripping
- Characters stripped: `. , ; : ) ] }`
- Applied after finding the DOI boundary
- Multiple trailing punctuation characters are all stripped

### DOI Boundary Detection
- Valid suffix characters: alphanumeric, `.`, `-`, `_`, `/`
- Boundaries detected: space, `?`, `#`, `&`, `=`, and other non-valid chars
- Trade-off: DOIs with slashes in suffix cannot be distinguished from URL path segments
  - Example: `10.1000/182/download` extracts as `10.1000/182/download`
  - This is acceptable since real DOIs legitimately contain slashes

### Error Handling
- `DoiError::NotFound` - no DOI pattern found in input
- `DoiError::InvalidInput` - empty or malformed input
- Uses `thiserror` for derive macro convenience

### Testing Strategy
- 30 unit tests covering:
  - Basic extraction from URLs and text
  - Canonicalization (prefix lowercase, suffix preserved)
  - Trailing punctuation stripping (all 7 chars)
  - Percent-decoding fallback
  - First-match-wins behavior
  - Error cases (not found, invalid input)
  - Complex suffixes with dots and slashes

## Crossref Models (2026-02-05)

### Schema Coverage
- Crossref `/works` response modeled from Swagger `WorkMessage` and `Work`
- `WorkRelation` uses a `BTreeMap<String, Vec<WorkRelationObject>>` to preserve relation keys
- Date structs allow optional `date-time`/`timestamp` for indexed/update payloads

### Fixtures
- Fixture JSON taken from `https://api.crossref.org/works/10.5555/12345678`

### Config Defaults
- Added Crossref config defaults for timeout, rate limiting, concurrency, and retry backoff
- Updated defaults to 10 req/s, concurrency 3, retry max 5, 1-60s backoff with jitter
- Default base_url set to `https://api.crossref.org/v1`

## DOI Extraction Refactor (2026-02-05)

### Regex Implementation
- Switched from state machine to `regex` crate for strict pattern matching
- Regex pattern: `10\.\d+/[^\s?#&=]+`
  - `10\.\d+/` matches the required prefix (10. + digits + /)
  - `[^\s?#&=]+` matches suffix until whitespace or URL delimiter
- Uses `std::sync::LazyLock` for efficient static regex compilation

### Boundary Handling
- Regex naturally stops at URL delimiters: whitespace, `?`, `#`, `&`, `=`
- Trailing punctuation stripped in post-processing step
- More accurate than manual state machine for complex edge cases

### Dependencies Added
- `regex = "1"` - for strict pattern matching
- `thiserror = "2"` - for error derive macros (already present)

### Implementation Notes
- `DOI_REGEX` compiled once using `LazyLock` for performance
- `strip_trailing_punctuation()` is a pure function for testability
- All 30 existing tests pass without modification

## Crossref Client (2026-02-05)

### Middleware + Retry
- `reqwest-middleware` chain uses `reqwest-ratelimit` with a `governor` rate limiter
- Retry policy uses exponential backoff with jitter and max retries from config
- `Retry-After` is honored for 429 responses via shared state between strategy and policy

### Request Defaults
- `mailto` query param always included (defaults to `icoderdev@outlook.com` when unset)
- Optional User-Agent only set when app name provided, formatted as `{app_name} {mailto}`

### Tests
- Wiremock tests cover 429 retry, 404 no-retry, mailto query, optional User-Agent, and JSON deserialization

## Crossref Middleware Switch (2026-02-05)

### Astral Middleware Integration
- `astral-reqwest-middleware` + `astral-reqwest-retry` require consistent middleware types
- Built a custom `RateLimitMiddleware` using `governor` to avoid type conflicts
- Middleware chain order: rate limit first, then retry policy

## Examples (2026-02-05)
- Added basic async example using `extract_doi_from_url` and `CrossrefClient::fetch_metadata`
- `CrossrefConfig::default()` plus optional `user_agent` is enough to compile the example
