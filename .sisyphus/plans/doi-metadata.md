# DOI Metadata Fetch (Crossref)

## TL;DR

> **Quick Summary**: Build an async Rust library that extracts a DOI from an HTTP URL via strict regex, normalizes it, and fetches full Crossref metadata using reqwest with astral-reqwest-middleware (rate limit + retry/backoff).
> 
> **Deliverables**:
> - DOI extraction + normalization utilities (strict `10.\d+/.+`, percent-decode, strip trailing punctuation)
> - Async Crossref client with middleware (rate limit + retry on 429/5xx)
> - Fully typed Crossref response structs (top-level + message)
> - Tests (unit + mocked HTTP)
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 2 waves
> **Critical Path**: Task 1 → Task 3

---

## Context

### Original Request
User needs: (1) extract DOI from HTTP URL, (2) fetch DOI metadata via Crossref with reqwest + middleware for rate limiting/retry, (3) metadata must be structured and full Crossref response; use strict DOI regex; optional User-Agent; mailto required.

### Interview Summary
**Key Discussions**:
- Regex-based DOI extraction anywhere in URL; strict `10.\d+/.+`; percent-decode before matching; strip trailing punctuation.
- Async-only API; tests after implementation; no caching; no alternate data sources.
- Crossref REST `/works/{doi}` with `mailto=icoderdev@outlook.com`; optional User-Agent from caller.
- Full Crossref response required (top-level fields + full message), fully typed structs.
- Middleware: astral-reqwest-middleware + astral-reqwest-retry; add rate limit via reqwest-ratelimit + governor.

**Research Findings**:
- Repo is a minimal Rust lib (`src/lib.rs` placeholder only; no deps).
- Crossref requires polite pool (mailto + UA) and responds with 429 for rate limits.
- astral-reqwest-middleware has retry, no rate limit; need reqwest-ratelimit.
- DOI normalization aligns with RFC 3986; RFC 3650/3651/3652 are Handle protocol (not implemented).

### Metis Review
**Identified Gaps (addressed)**:
- DOI match policy: choose first match in string (deterministic).
- Percent-decoding scope: attempt regex on raw URL; if no match, percent-decode and retry.
- Timeouts/base URL: make configurable via client config (for tests/mocks).
- Retry behavior: custom strategy retries 429/5xx and honors Retry-After if present.
- Tests must be mocked (no real Crossref calls) and validate headers, retry, and parsing.

---

## Work Objectives

### Core Objective
Provide a small async Rust library that extracts and normalizes DOIs from URLs and fetches fully typed Crossref metadata with polite, rate-limited requests.

### Concrete Deliverables
- `Doi` type with original + canonical form
- `extract_doi_from_url(url: &str) -> Result<Doi, DoiError>`
- `CrossrefClient` (shared reqwest client with middleware)
- `fetch_metadata(doi: &Doi) -> Result<CrossrefResponse, CrossrefError>`
- Fully typed `CrossrefResponse` / `CrossrefMessage` structs

### Definition of Done
- [x] `cargo test` passes locally with mocked HTTP (no real network)
- [x] Extracts DOI from URLs with percent-encoding and trailing punctuation
- [x] Fetches Crossref metadata with mailto, optional User-Agent, retry on 429/5xx

### Must Have
- Strict DOI regex `10.\d+/.+` and canonicalization (lowercase prefix, preserve suffix)
- Optional User-Agent; always include mailto query param
- Full Crossref response (top-level + message) via typed structs

### Must NOT Have (Guardrails)
- No Handle protocol implementation (RFC 3650/3651/3652 as protocol)
- No alternate data sources (DataCite/doi.org content negotiation)
- No caching or storage
- No sync API or CLI

---

## Verification Strategy (MANDATORY)

> **UNIVERSAL RULE: ZERO HUMAN INTERVENTION**
>
> ALL tasks in this plan MUST be verifiable WITHOUT any human action.

### Test Decision
- **Infrastructure exists**: YES (basic Rust test module)
- **Automated tests**: Tests-after
- **Framework**: Rust built-in + `tokio` for async tests + mock HTTP crate

### Agent-Executed QA Scenarios (MANDATORY — ALL tasks)

Each task includes concrete, tool-driven scenarios with exact commands and evidence paths.

---

## Execution Strategy

### Parallel Execution Waves

Wave 1 (Start Immediately):
├── Task 1: Define Crossref models, config, errors
└── Task 2: Implement DOI extraction + normalization

Wave 2 (After Wave 1):
└── Task 3: Crossref client + public API + mocked HTTP tests

Critical Path: Task 1 → Task 3
Parallel Speedup: ~30% faster than sequential

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 3 | 2 |
| 2 | None | 3 | 1 |
| 3 | 1, 2 | None | None |

---

## TODOs

> Implementation + Test = ONE Task. Never separate.

- [x] 1. Define Crossref models, config, and errors

  **What to do**:
  - Add dependencies: `serde`, `serde_json`, `reqwest`, `reqwest-middleware`, `astral-reqwest-retry`, `reqwest-ratelimit`, `governor`, `thiserror`, `tokio` (dev), and a mock HTTP crate (e.g., `wiremock`).
  - Define `CrossrefResponse` (status, message-type, message-version, message).
  - Define fully typed `CrossrefMessage` from Crossref Swagger schema; use `#[serde(default)]` for optional fields.
  - Add fixture JSON at `tests/fixtures/crossref.json` from a Crossref example response.
  - Add `CrossrefClientConfig` with base URL, timeout, mailto, optional user_agent, rate_limit, concurrency, retry policy.
  - Define error enums: `DoiError`, `CrossrefError` (invalid DOI, HTTP status, deserialize errors, retry exhausted).

  **Must NOT do**:
  - Do not add caching or alternate data sources
  - Do not implement Handle protocol

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Multi-struct modeling + config + error taxonomy
  - **Skills**: [x]
    - No special skills required
  - **Skills Evaluated but Omitted**:
    - `rust-error-snafu`: Using `thiserror` for simplicity

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 2)
  - **Blocks**: Task 3
  - **Blocked By**: None

  **References**:
  - `Cargo.toml` - add dependencies
  - `src/lib.rs` - replace placeholder and re-export modules
  - Crossref Swagger: `https://api.crossref.org/swagger-ui/index.html` (schema for message fields)
  - Crossref REST docs: `https://www.crossref.org/documentation/retrieve-metadata/rest-api/`

  **Acceptance Criteria**:
- [x] `cargo test -q crossref_models` exits 0
- [x] Deserialization test loads a fixture JSON into `CrossrefResponse` and preserves top-level fields

  **Agent-Executed QA Scenarios**:

  Scenario: Deserialize Crossref fixture into typed structs
    Tool: Bash
    Preconditions: Fixture JSON stored at `tests/fixtures/crossref.json`
    Steps:
      1. Run: `cargo test -q crossref_models -- --nocapture > .sisyphus/evidence/task-1-crossref-models.txt`
      2. Assert: exit code 0
    Expected Result: Tests pass, types deserialize successfully
    Evidence: `.sisyphus/evidence/task-1-crossref-models.txt`

- [x] 2. Implement DOI extraction + normalization

  **What to do**:
  - Implement `extract_doi_from_url(url: &str) -> Result<Doi, DoiError>`.
  - Match strict regex `10.\d+/.+` anywhere in the URL.
  - If no match, percent-decode the URL string and re-run match.
  - If multiple matches, pick the first match in string order.
  - Strip trailing punctuation characters (e.g., `. , ; : ) ] }`) from the matched DOI.
  - Canonicalize: lowercase prefix only, preserve suffix case; keep original in `Doi`.

  **Must NOT do**:
  - Do not relax regex beyond `10.\d+/.+`
  - Do not attempt Handle protocol resolution

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Isolated parsing logic and unit tests
  - **Skills**: []
    - No special skills required
  - **Skills Evaluated but Omitted**:
    - `rust-error-snafu`: Keeping error types simple via `thiserror`

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Task 3
  - **Blocked By**: None

  **References**:
  - `src/lib.rs` - current placeholder to replace with modules
  - RFC 3986: `https://datatracker.ietf.org/doc/html/rfc3986` (percent-encoding rules)

  **Acceptance Criteria**:
- [x] `cargo test -q doi_extract` exits 0
- [x] Extracts DOI from URLs with query params, fragments, and percent-encoding
- [x] Invalid DOI inputs return a typed error (no panic)
- [x] Strips trailing punctuation and returns canonical DOI

  **Agent-Executed QA Scenarios**:

  Scenario: Extract DOI from URL with punctuation and encoding
    Tool: Bash
    Preconditions: Unit tests exist under `src/doi.rs` or `tests/doi_extract.rs`
    Steps:
      1. Run: `cargo test -q doi_extract -- --nocapture > .sisyphus/evidence/task-2-doi-extract.txt`
      2. Assert: exit code 0
    Expected Result: Tests pass for encoded DOI, trailing punctuation, and multiple matches
    Evidence: `.sisyphus/evidence/task-2-doi-extract.txt`

  Scenario: Invalid DOI input returns error
    Tool: Bash
    Preconditions: Unit test for invalid input exists
    Steps:
      1. Run: `cargo test -q doi_extract::invalid_input -- --nocapture > .sisyphus/evidence/task-2-doi-invalid.txt`
      2. Assert: exit code 0
    Expected Result: Error variant returned for invalid input
    Evidence: `.sisyphus/evidence/task-2-doi-invalid.txt`

- [x] 3. Build Crossref client with middleware + public API

  **What to do**:
  - Build a shared async `CrossrefClient` using `reqwest-middleware`.
  - Rate limit with `reqwest-ratelimit` + `governor` (defaults: 10 req/s, concurrency 3).
  - Retry using `astral-reqwest-retry` with exponential backoff + jitter; retry 429/5xx.
  - Respect `Retry-After` header for 429 if present.
  - Add request builder: base URL (configurable), `mailto` query param, optional User-Agent formatted as `{app_name} {mailto}`.
  - Expose public API functions in `lib.rs`.
  - Add mocked HTTP tests: verify headers, rate limit wiring, retry on 429, no retry on 4xx.

  **Must NOT do**:
  - Do not call real Crossref in tests
  - Do not add caching or alternate metadata sources

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Middleware wiring + async HTTP + retry logic
  - **Skills**: [x]
    - No special skills required
  - **Skills Evaluated but Omitted**:
    - `rust-error-snafu`: Sticking to `thiserror`

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (after Tasks 1 & 2)
  - **Blocks**: None
  - **Blocked By**: Tasks 1, 2

  **References**:
  - `Cargo.toml` - dependency and feature flags
  - `src/lib.rs` - public API surface
  - `https://docs.rs/astral-reqwest-middleware/0.4.2`
  - `https://docs.rs/astral-reqwest-retry/0.8.0`
  - `https://docs.rs/reqwest-ratelimit/0.5.0`

  **Acceptance Criteria**:
- [x] `cargo test -q crossref_client` exits 0
- [x] Mocked tests verify 429 retry and optional User-Agent behavior
- [x] 4xx (except 429) does NOT retry
- [x] Fetch returns full typed `CrossrefResponse`

  **Agent-Executed QA Scenarios**:

  Scenario: Retry on 429 with Retry-After
    Tool: Bash
    Preconditions: Mock server configured to return 429 then 200
    Steps:
      1. Run: `cargo test -q crossref_client::retry_on_429 -- --nocapture > .sisyphus/evidence/task-3-retry-429.txt`
      2. Assert: exit code 0
    Expected Result: Client retries once and succeeds
    Evidence: `.sisyphus/evidence/task-3-retry-429.txt`

  Scenario: Optional User-Agent header
    Tool: Bash
    Preconditions: Mock server asserts request headers
    Steps:
      1. Run: `cargo test -q crossref_client::user_agent_header -- --nocapture > .sisyphus/evidence/task-3-ua-header.txt`
      2. Assert: exit code 0
    Expected Result: Header present only when app_name provided
    Evidence: `.sisyphus/evidence/task-3-ua-header.txt`

  Scenario: 404 does not retry
    Tool: Bash
    Preconditions: Mock server returns 404 and counts request attempts
    Steps:
      1. Run: `cargo test -q crossref_client::no_retry_on_404 -- --nocapture > .sisyphus/evidence/task-3-no-retry-404.txt`
      2. Assert: exit code 0
    Expected Result: Single request attempt, error returned
    Evidence: `.sisyphus/evidence/task-3-no-retry-404.txt`

- [x] 4. Add usage example

  **What to do**:
  - Add `examples/basic.rs` showing `extract_doi_from_url` + `CrossrefClient::fetch_metadata` usage.
  - Use an async runtime (`tokio::main`) and print a small field (e.g., title or DOI).

  **Must NOT do**:
  - Do not call real Crossref in tests (example is not a test).
  - Do not add new dependencies unless required by the example.

  **Acceptance Criteria**:
  - [x] `examples/basic.rs` exists and demonstrates DOI extraction + metadata fetch.
  - [x] `cargo check --example basic` exits 0.

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 2 | `feat(doi): add doi parsing and models` | src/*, Cargo.toml | `cargo test -q` |
| 3 | `feat(crossref): add client with middleware` | src/*, tests/* | `cargo test -q` |

---

## Success Criteria

### Verification Commands
```bash
cargo test
```

### Final Checklist
- [x] DOI extraction handles percent-encoded URLs and trailing punctuation
- [x] Crossref requests include `mailto=icoderdev@outlook.com`
- [x] Optional User-Agent applied when app_name provided
- [x] Retry policy handles 429/5xx with backoff
- [x] All tests pass without real network access
