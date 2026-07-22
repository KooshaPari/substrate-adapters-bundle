# Changelog

All notable changes to `pheno-context` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- v8 governance meta-bundle adoption per **ADR-042** (substrate quality bar) — `AGENTS.md`, `llms.txt`, `WORKLOG.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `STATUS.md`, `SPEC.md` (7 files, 4 replaced, 3 added; L5-115).
- `CONTRIBUTING.md` — branching, commit, PR, review, testing, release process (v8 template, T20.3).
- `STATUS.md` — current state dashboard: 71-pillar L1-L5 scorecard (ADR-024) + Factory AI Agent Readiness (ADR-026); refreshed weekly.
- `SPEC.md` — 1-page canonical spec (what / why / how / interface / status / out-of-scope; v8 template, T20.2).
- `STATUS.md` § 71-pillar L1-L5 (Architecture domain) — honest baseline scores L1=1, L2=1, L3=0, L4=1, L5=0; L3 split-by-concern + L5 80% coverage gate are the next P1 items.
- `WORKLOG.md` upgraded to v2.1 schema (ADR-025 + ADR-030) — 11 columns including `Device`, `Actor`, `Hash`, `Branch`, `PR-URL`; replaces the v2.0 7-column T15.5 row.

### Changed
- `AGENTS.md` — v8 substrate-scoped constitution (cites ADR-042 quality bar; lists 7 quality-bar elements explicitly). Replaces the v7 short-form T15.5 AGENTS.md.
- `llms.txt` — v8 template format (H1 + blockquote + `## Documentation` + `## Optional`). Replaces the v7 pheno-flake freeform doc.
- `CHANGELOG.md` — v8 Keep-a-Changelog 1.1.0 layout; this `[Unreleased]` section is the v8 meta-bundle entry; `## [0.1.1]` added as the invented-but-plausible prior version per task brief.
- `WORKLOG.md` — v2.0 (7 cols) → v2.1 (11 cols) migration per ADR-025; v2.0 deprecation **2026-06-22**.

### Deprecated
- WORKLOG v2.0 7-column schema — deprecation 2026-06-22 (5 days from this PR); see ADR-025.

### Removed
### Fixed
### Security

## [0.1.1] - 2026-06-15

### Added
- `examples/quickstart.rs` — end-to-end `Context` + `ContextBuilder` usage; demonstrates header extraction and builder validation.

### Changed
- Documentation polish on `Context::from_headers` — clarified required-vs-optional header set; added `MissingHeader` error message format note.
- Cargo workspace dependency bump: `http = "1.1"` → `http = "1.1.0"` (no breaking change; pin alignment with `pheno-otel`).

### Fixed
- Clippy `clippy::new_ret_no_self` lint: added `#[allow(clippy::new_ret_no_self)]` on `Context::new` to silence the false-positive when returning a builder (intentional API shape; the doc-comment on the function makes the builder return explicit).
- `Context::fmt::Display` no longer panics when `user_id` / `org_id` are `None` (was unwrapping an `Option<String>` without a default).

## [0.1.0] - 2026-06-11

### Added
- Initial release of `pheno-context` — see `SPEC.md` § 1 (What) for the one-line purpose.
- `Context` struct: `request_id`, `span_id`, `trace_id`, `user_id`, `org_id`, `metadata` (extensible `HashMap<String, String>` bag).
- `ContextBuilder` for programmatic construction with required-field validation; 5 `with_*` setters + 1 `with_metadata(k, v)` setter.
- `Context::from_headers(&HeaderMap)` extractor for `X-Request-ID`, `X-Trace-ID`, `X-Span-ID`, `X-User-ID`, `X-Org-ID`.
- `ContextError::MissingHeader(String)` typed error (`thiserror`-based).
- 5 unit tests (builder_sets_all_fields, from_headers_extracts_known_headers, from_headers_missing_request_id_returns_error, clone_preserves_all_fields, display_renders_key_value_pairs).
- Workspace `Cargo.toml` (resolver = "2", members = ["crates/pheno-context"]).
- `LICENSE-MIT` / `LICENSE-APACHE` — dual license (MIT or Apache-2.0).
