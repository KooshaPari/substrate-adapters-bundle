# pheno-context — SPEC.md

> **Spec status:** `implemented` — this document reflects the current codebase (`v0.1.0` + `[Unreleased]` v8 meta-bundle).
> **Last audited:** `2026-06-18` against tree `chore/v8-meta-bundle-2026-06-18` (post-merge: `git rev-parse main`).
> **Substrate tier:** `pheno-*-lib` (per ADR-023 Rule 3).

---

## 1. What (1 paragraph)

`pheno-context` is a **pure, single-crate Rust library that defines the canonical request-scoped `Context` value** that **every HTTP-handling substrate in the pheno-* fleet reads and writes**. It is consumed by `pheno-otel`, `pheno-tracing`, `pheno-config`, and the federated services (`phenoMCP`, `phenoObservability`, `phenoEvents`) to propagate `request_id` / `span_id` / `trace_id` / `user_id` / `org_id` and a free-form `metadata` bag through the call graph.

## 2. Why (1 paragraph)

Before this crate, every service re-implemented a slightly different `request_id` + `trace_id` struct, leading to cross-service trace correlation drift. ADR-013 + ADR-035 converge on a single canonical `Context` shape so that trace propagation, OTLP export, audit-log correlation, and `pheno-tracing` span linkage all read from the same value. Existing alternatives (hand-rolled structs, `tracing::Span` extensions, `tokio::task_local!` shims) either pull in an async runtime, couple to a transport, or do not survive `serde` round-trips.

## 3. How (architecture, 3 sentences)

`Context` is a flat `Clone + Debug + PartialEq` struct of 3 required `String` IDs + 2 optional `String` IDs + a `HashMap<String, String>` bag. `ContextBuilder` is a fluent builder that validates required-field presence at `build()` time and returns `ContextError::MissingHeader(String)` on the first missing field. `Context::from_headers(&HeaderMap)` extracts the same shape from a `http::HeaderMap` (or any compatible type), mapping `X-Request-ID` / `X-Trace-ID` / `X-Span-ID` (required) and `X-User-ID` / `X-Org-ID` (optional).

```
       ┌──────────────────┐
HTTP   │  X-Request-ID    │  required  ┌──────────────────┐
header │  X-Trace-ID      │ ────────▶  │                  │
 ────▶ │  X-Span-ID       │            │  Context         │
       │  X-User-ID (opt) │            │   { request_id,  │
       │  X-Org-ID  (opt) │            │     span_id,     │
       └──────────────────┘            │     trace_id,    │
                                       │     user_id?,    │
       ┌──────────────────┐            │     org_id?,     │
code   │  ContextBuilder  │ ────────▶  │     metadata{} } │
  ───▶ │   .with_*(...)   │            │                  │
       │   .build()?      │            └──────────────────┘
       └──────────────────┘                      │
                                                ▼
                                       ┌──────────────────┐
                                       │  fmt::Display    │  for log lines
                                       │  + Eq            │  for assertions
                                       │  + Clone         │  for fan-out
                                       └──────────────────┘
```

## 4. Interface (API surface)

```rust
// crates/pheno-context/src/lib.rs
pub struct Context {
    pub request_id: String,         // required, non-empty (validated in builder)
    pub span_id: String,            // required, non-empty
    pub trace_id: String,           // required, non-empty
    pub user_id: Option<String>,
    pub org_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

pub struct ContextBuilder { /* private fields */ }
impl ContextBuilder {
    pub fn with_request_id(self, v: impl Into<String>) -> Self;
    pub fn with_span_id(self, v: impl Into<String>) -> Self;
    pub fn with_trace_id(self, v: impl Into<String>) -> Self;
    pub fn with_user_id(self, v: impl Into<String>) -> Self;
    pub fn with_org_id(self, v: impl Into<String>) -> Self;
    pub fn with_metadata(self, k: impl Into<String>, v: impl Into<String>) -> Self;
    pub fn build(self) -> Result<Context, ContextError>;
}

impl Context {
    pub fn new() -> ContextBuilder;
    pub fn from_headers(headers: &HeaderMap) -> Result<Self, ContextError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("missing header: {0}")]
    MissingHeader(String),
}
```

Full API: see `llms.txt` and `crates/pheno-context/src/lib.rs`.

## 5. Status (current state)

| Component                                | Status     | Notes                                                                              |
| ---------------------------------------- | ---------- | ---------------------------------------------------------------------------------- |
| `Context` struct                         | `shipped`  | 5 public fields, `Clone + Debug + PartialEq`.                                       |
| `ContextBuilder`                         | `shipped`  | Fluent builder, `build()` validates required fields.                                |
| `Context::from_headers`                  | `shipped`  | 3 required + 2 optional HTTP headers.                                                |
| `ContextError::MissingHeader`            | `shipped`  | `thiserror`-based typed error.                                                      |
| `Display` impl                           | `shipped`  | Renders all 6 fields; no-panic on `None` (fixed in `[0.1.1]`).                     |
| `--features tracing` integration         | `partial`  | Compiles; OTLP smoke test does not yet assert output (L5-116).                      |
| `pheno-context::scope` (task-local)      | `blocked`  | L5-117 deliverable; target `2026-07-02`. Requires async runtime decision (L7-L8).    |
| CI / tests                               | `yellow`   | 5/5 unit tests pass on MacBook; coverage gate pending heavy-runner (L5-118).         |
| Observability                            | `partial`  | `tracing` feature flag present; OTLP export not yet wired (L5-116).                 |

## 6. Out of scope (explicit deferrals)

- **Async runtime** — `tokio::task_local!` (L5-117) deferred until an async-using consumer is on the runway. ADR-023 Rule 3 forbids pulling an async runtime into a `pheno-*-lib`.
- **OTLP export** — deferred to `pheno-otel`; `pheno-context` only holds the IDs that `pheno-otel` reads.
- **`serde::Serialize` / `Deserialize`** — deferred to L5-120. Headers and `metadata` are both `String`-typed today, so adding serde is a pure additive change.
- **W3C `traceparent` parsing** — deferred to `pheno-tracing` (the canonical tracing substrate); `pheno-context` accepts the IDs as opaque strings and does not parse the W3C header shape.
- **Per-tenant key rotation** — `org_id` is a single string; multi-tenant key rotation is a `pheno-tenant-*` concern, not a `pheno-context` concern.

## 7. References

- `README.md` — quickstart, when/when NOT, install.
- `llms.txt` — full API reference (LLM-friendly index).
- `WORKLOG.md` — change history (per ADR-025 v2.1, includes `device:` field).
- `CHANGELOG.md` — release notes (Keep-a-Changelog 1.1.0 + SemVer 2.0.0).
- `CONTRIBUTING.md` — how to contribute.
- `STATUS.md` — current state dashboard (71-pillar L1-L5 + Factory AI readiness).
- `LICENSE-MIT` / `LICENSE-APACHE` — dual license.
- ADR-013 — `pheno-mcp-router` substrate pattern (sibling substrate, same hex layout).
- ADR-023 — substrate placement rules; this is `pheno-*-lib`.
- ADR-024 — 71-pillar audit framework (this `STATUS.md` § 8).
- ADR-025 — WORKLOG v2.1 schema (this `WORKLOG.md`).
- ADR-030 — `pheno-worklog-schema` v2.1 pins the 4 `device:` values.
- ADR-042 — substrate quality bar (the 7 elements: spec + README + test matrix + observability + coverage + CI gate + worklog v2.1).

---

## Template usage notes

- **Length:** target ≤ 1 page (≤ 80 lines). If you cannot fit, your scope is too large — split the substrate.
- **Status discipline:** mark every component with one of `shipped | scaffold | partial | blocked`. No "TBD".
- **Tone:** declarative, present tense. "X is Y" not "X will be Y".
- **Update cadence:** re-audit every quarter OR on any major architectural change, whichever comes first. Mark "Last audited" date.
- **Per ADR-042 (Substrate quality bar):** this SPEC.md is element 1 of the 7-element bar. CI lint (`pheno-ci-templates/quality-bar.yml`) fails PRs missing it.
