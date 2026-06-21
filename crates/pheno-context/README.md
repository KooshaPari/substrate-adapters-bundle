# pheno-context

Canonical request context for the pheno-* fleet. Builder, header extraction, and structured metadata.

## What

`pheno_context::Context` carries the canonical identifiers every service in the pheno-* fleet needs to thread through a request:

- `request_id` — inbound request correlation
- `span_id`, `trace_id` — observability IDs (compatible with `pheno-otel`)
- `user_id`, `org_id` — multi-tenant context
- `metadata` — extensible `HashMap<String, String>` bag

## When

Use when your service needs to:
- Parse a `Context` from incoming HTTP headers (`X-Request-ID`, `X-Trace-ID`, `X-Span-ID`, optional `X-User-ID`, `X-Org-ID`).
- Build a `Context` programmatically with the fluent `ContextBuilder`.
- Carry a `Context` through layers, tasks, or async boundaries.

## When **not**

- If you only need tracing, use `pheno-tracing` directly.
- If you need full per-request state with async cancellation, use `tower` or `axum` extractors.
- If you need distributed trace propagation (W3C `traceparent`), wait for the L4 follow-up.

## Quickstart (5 lines)

```rust
use pheno_context::Context;

let ctx = Context::from_headers(&headers)?;        // 1
let _ = ctx.request_id;                            // 2
let bag = ctx.metadata;                            // 3
// pass ctx into your service layer
```

Or builder-style:

```rust
use pheno_context::Context;

let ctx = Context::new()
    .with_request_id("req-1")
    .with_span_id("span-1")
    .with_trace_id("trace-1")
    .with_user_id("user-1")
    .build()?;
```

## Status

Recovered 2026-06-17 from the stranded `chore/l4-68-pheno-context-2026-06-11` worktree (commit `d8960dfd80`). Original author: Koosha Pari. Substrate per ADR-023 (`pheno-*-lib`).

## Substrate placement

This crate is a `pheno-*-lib` per ADR-023: a single-concern, language-specific, pure reusable library. It depends only on `thiserror` and `http` (the de-facto Rust HTTP types crate). No `tokio`, no async runtime, no I/O.

## License

MIT OR Apache-2.0
