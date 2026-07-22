# pheno-context — WORKLOG.md

> **Schema:** `v2.1` (ADR-025; supersedes v2.0 on 2026-06-22). Validator: `pheno-worklog-schema` Python lib (`SPEC-v2.1.md`).
> **Location:** repo root, alongside `SPEC.md` + `CHANGELOG.md` + `AGENTS.md`.

---

## 1. Schema (v2.1, 11 columns, canonical order)

| Date | Task ID | Layer | Action | Files | Notes | Device | Actor | Hash | Branch | PR-URL |
|---|---|---|---|---|---|---|---|---|---|---|
| `YYYY-MM-DD` | `L<n>-<id>` | `L<n>` | `verb` | `path:line` | text | enum | enum | `sha256:8` | `branch` | `https://.../pull/n` |

One task per row. Header row is mandatory.

## 2. Columns

- **Date** — ISO 8601 day work landed. **Task ID** — fleet DAG id (`L1-001`, `L5-104.1`, `T15.5`).
- **Layer** — DAG layer (L1 top → L9 infra); mirrors Task ID prefix. **Action** — past-tense verb: `add` / `fix` / `refactor` / `absorb` / `archive` / `migrate` / `docs` / `chore`.
- **Files** — primary path with `path:startLine-endLine` per AGENTS.md citation rule. **Notes** — one line, no markdown.
- **Device** — see § 3. **Actor** — `human` / `forge` / `codex` / `droid` / `ci-bot`.
- **Hash** — `sha256:<8 hex>` of concatenated column values (optional). **Branch** — git branch the work landed on.
- **PR-URL** — full PR URL, or empty if not yet opened.

## 3. Device field (ADR-025 + ADR-030, 4 valid values)

- **`macbook`** — planning, ADRs, small focused PRs, code review, dogfooding (ADR-023 device-fit gate).
- **`heavy-runner`** — full `cargo test --workspace`, iOS sim, DinD, Unity editor, > 10 min single build/test on MacBook.
- **`subagent`** — work dispatched via forge / codex / Codex to a worker tier.
- **`ci`** — work performed by GitHub Actions or other CI bot.

## 4. Recent work

| Date | Task ID | Layer | Action | Files | Notes | Device | Actor | Hash | Branch | PR-URL |
|---|---|---|---|---|---|---|---|---|---|---|
| 2026-06-18 | L5-115 | L5 | chore | AGENTS.md:1,llms.txt:1,WORKLOG.md:1,CHANGELOG.md:1,CONTRIBUTING.md:1,STATUS.md:1,SPEC.md:1 | Apply v8 governance meta-bundle per ADR-042 — 7 files (4 replaced, 3 added); 0 Rust code changes | `macbook` | `human` | | chore/v8-meta-bundle-2026-06-18 | (this PR) |
| 2026-06-18 | T15.5 | L0 | docs | AGENTS.md:1,llms.txt:1,WORKLOG.md:1,CHANGELOG.md:1,LICENSE-MIT:1,.github/workflows/ci.yml:1 | v7 pheno-flake refresh — 5 governance docs + CI workflow (test + clippy + fmt + 80% coverage gate per ADR-023); OTLP smoke test wired via pheno-otel/pheno-tracing (ADR-012) | `macbook` | `human` | | chore/t15-5-pheno-flake-refresh-2026-06-18 | <https://github.com/KooshaPari/pheno-context/pull/1> |
| 2026-06-18 | L5-110-L5-119 | L0 | chore | .github/workflows/ci.yml:1 | Add CI workflow from pheno-ci-templates (test + clippy + fmt + 80% coverage gate per ADR-023 Rule 3.1) | `macbook` | `human` | | chore/t15-5-pheno-flake-refresh-2026-06-18 | <https://github.com/KooshaPari/pheno-context/pull/1> |
| 2026-06-11 | L4-68 | L4 | add | crates/pheno-context/Cargo.toml:1,crates/pheno-context/src/lib.rs:1 | Initial implementation: Context struct, ContextBuilder, Context::from_headers, ContextError::MissingHeader; 5 unit tests (builder, headers, missing-required, clone, display) | `macbook` | `human` | | feat/l4-68-pheno-context-2026-06-11 | <https://github.com/KooshaPari/pheno-context/commit/651b5b2> |

## 5. Validation

`pheno-worklog-schema` v2.1 enforces: header row matches 11-column schema exactly (order, casing, dashes); `Date` is ISO 8601; `Task ID` matches `^[A-Z]?[0-9]+([.-][0-9]+)*$`; `Device` ∈ {`macbook`, `heavy-runner`, `subagent`, `ci`}; `Actor` ∈ {`human`, `forge`, `codex`, `droid`, `ci-bot`}; `PR-URL` is a valid `https://github.com/.../pull/<n>` URL or empty; one task per row. CLI: `pheno-worklog-schema validate ./WORKLOG.md`.

## 6. Related

- **AGENTS.md** § 6 — Authority, § 4 — Conventions (worklog v2.1).
- **ADR-015** — v2.0 10-column schema (superseded 2026-06-22). **ADR-025** — v2.1 bump, adds `device:` column.
- **ADR-030** — `pheno-worklog-schema v2.1`, pins the 4 `device:` values. **ADR-023** — device-fit gate.
- **pheno-worklog-schema** — `SPEC-v2.1.md`, validator CLI, `migrate_v2_to_v2_1.py`.
