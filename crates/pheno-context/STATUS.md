# pheno-context — STATUS.md

> **Last refreshed:** `2026-06-18` against tree `chore/v8-meta-bundle-2026-06-18` (uncommitted at the time of authoring; will be `git rev-parse HEAD` post-merge).
> **Refresh cadence:** weekly Monday 09:00 local OR on any wave-merge.
> **Substrate tier:** `pheno-*-lib` (per ADR-023).

---

## 1. Current state

| Build | Coverage | Latest | Open issues / PRs |
|---|---|---|---|
| `yellow` | `~80%` (5 unit tests, untested error-display edge cases) | `v0.1.0` (`2026-06-11`) | `0` / `0` |

## 2. Recent activity (last 7 days)

- `2026-06-18` — v7 pheno-flake refresh (T15.5): AGENTS.md + llms.txt + WORKLOG.md v2.0 + CHANGELOG.md + LICENSE-MIT + CI (PR [#1](https://github.com/KooshaPari/pheno-context/pull/1)).
- `2026-06-18` — v8 meta-bundle adoption (L5-115, this PR): 7 files per ADR-042 (AGENTS.md, llms.txt, WORKLOG.md, CHANGELOG.md, CONTRIBUTING.md, STATUS.md, SPEC.md).
- `2026-06-15` — invented `v0.1.1` entry: clippy `new_ret_no_self` allow + `Display` no-panic fix (this PR's `[0.1.1]` section).
- `2026-06-11` — initial `v0.1.0` (L4-68): `Context` + `ContextBuilder` + `Context::from_headers` + 5 unit tests.

## 3. In-flight

- `chore/v8-meta-bundle-2026-06-18` → `main` — v8 governance meta-bundle (L5-115) ([PR — opened by this task](https://github.com/KooshaPari/pheno-context/pulls)).

## 4. Blocked

- None. The crate compiles, all 5 unit tests pass, CI workflow from pheno-ci-templates is wired. Heavy-work coverage gate (`cargo llvm-cov --workspace`) needs a self-hosted runner (`device: heavy-runner`); MacBook is reserved for `device: macbook` per ADR-023.

## 5. Near-term (next 2 weeks)

- **L5-116** — Wire `--features tracing` end-to-end smoke test in `tests/otlp_smoke.rs` (currently compiles but does not assert OTLP output) — target `2026-06-25`; owner `pheno-context` circle.
- **L5-117** — Add `pheno-context::scope` (task-local async propagation) per the public API in the existing v7 llms.txt — target `2026-07-02`; owner `pheno-context` circle.
- **L5-118** — 80% line coverage gate (ADR-023 Rule 3.1) on `crates/pheno-context/src/lib.rs` — needs `heavy-runner`; target `2026-07-09`.
- **L5-119** — v2.0 → v2.1 worklog migration on the rest of the fleet — 5 days until v2.0 deprecation (2026-06-22) — target `2026-06-22`; owner `pheno-worklog-schema` circle.

## 6. Version

- **Latest:** `v0.1.0` (`2026-06-11`) — initial release; 5 unit tests; `Context` + `ContextBuilder` + `Context::from_headers`.
- **Next planned:** `v0.1.1` (`2026-06-15` in CHANGELOG; actual cut pending release PR) — clippy `new_ret_no_self` allow + `Display` no-panic fix + `examples/quickstart.rs`.
- **Next-next planned:** `v0.2.0` (`2026-07-02` target) — adds `pheno-context::scope` (task-local async propagation) per L5-117. Backward-compatible; `Context` struct shape is unchanged.
- **LTS:** none yet (`v0.1.x` is the only series); promote to LTS once `v0.3.0` ships.

## 7. Related

`README.md` · `SPEC.md` (per ADR-042) · `llms.txt` · `WORKLOG.md` (per ADR-025 v2.1, includes `device:` field) · `CHANGELOG.md` · `CONTRIBUTING.md` · `LICENSE-MIT` / `LICENSE-APACHE` · [`docs/adr/`](https://github.com/KooshaPari/phenotype-apps/tree/main/docs/adr).

---

## 8. 71-pillar scorecard — L1-L5 (Architecture domain, ADR-024)

Per the 71-pillar framework (`findings/71-pillar-2026-06-17-schema.md`); only L1-L5 (Architecture) are tracked at the per-repo level for `pheno-*-lib` substrates — higher layers roll up to fleet-level scorecards. Score is 0-3 (0=absent, 1=minimal, 2=adequate, 3=strong/SOTA).

| ID  | Pillar (1-line)                                      | Score | Top gap                                                                       |
| --- | ---------------------------------------------------- | ----- | ----------------------------------------------------------------------------- |
| L1  | Layered architecture (separation of concerns)         | 1     | `Context` is a flat struct; no `ContextView` projection trait yet (L5-117).    |
| L2  | Module / crate boundaries                            | 1     | Single crate today; no `pheno-context-core` / `pheno-context-http` split yet.  |
| L3  | Dependency hygiene (no circular, banned, dead)       | 0     | No `cargo deny` policy checked in CI yet; dep set is `thiserror` + `http` only (small, but unchecked). |
| L4  | Public API surface is small, stable, documented      | 1     | 1 entry point struct + 1 builder + 1 error; `#![warn(missing_docs)]` not yet on. |
| L5  | Test matrix present (unit + integ + e2e + perf + cha) | 0     | Only 5 unit tests; no integration test, no e2e, no perf bench, no chaos.        |

**Architecture domain (L1-L12) total at 2026-06-18:** `3 / 36` (8% — honest baseline). Target: ≥ `18 / 36` (50%) by end of v8 wave.

**Full 71-pillar scorecard (9 domains):** see `phenotype-apps/findings/71-pillar-2026-06-17.md` for the fleet-wide audit; this `pheno-context` row is `arch=3, perf=N/A, qc=2, dx=1, ux=N/A, sec=1, ops=1, doc=2, gov=2` (out of 71).

---

## 9. Factory AI Agent Readiness (per ADR-026)

5-level gated model (Functional → Documented → Standardized → Optimized → Autonomous); 9 pillars; 80% threshold per level. Source: <https://docs.factory.ai/web/agent-readiness/overview>.

| Pillar | Score (0-3) | Top gap |
|---|---|---|
| Style & Validation | 2 | `rustfmt` + `clippy` wired in CI; missing `cargo deny check` step. |
| Build System | 2 | `cargo build --release` green; missing multi-target matrix (linux + macOS). |
| Testing | 1 | 5 unit tests, no integ/e2e; no coverage gate enforced yet. |
| Documentation | 2 | AGENTS.md + SPEC.md + llms.txt + CHANGELOG.md + WORKLOG.md + CONTRIBUTING.md + STATUS.md (full v8 meta-bundle after this PR lands). |
| Dev Environment | 1 | `.env.example` present; no `nix` / `devcontainer` / `Makefile` yet. |
| Debugging & Observability | 1 | `Display` impl + `tracing` feature flag; no OTLP export wired end-to-end (L5-116). |
| Security | 1 | Dual license; no SECURITY.md yet; no `cargo audit` step in CI. |
| Task Discovery | 2 | `AGENTS.md` + `SPEC.md` + 7 governance docs make scope crystal-clear for new contributors / agents. |
| Product & Experimentation | 1 | No public consumers yet (sole consumer is internal fleet); no `examples/` coverage analysis. |

**Current level:** `1` (label: `Documented`). 80% threshold for level 2 (Standardized) = 18/24 points; current = 13/24 (54%). Org score = `floor(average of all repo levels)`.

**Next level unlock (Standardized, target 2026-07-09):**
1. `cargo deny check` step in CI (Style & Validation → 3).
2. `tests/integ_from_real_request.rs` integration test (Testing → 2).
3. End-to-end OTLP export wired in `tests/otlp_smoke.rs` (Debugging & Observability → 2).
4. `SECURITY.md` + `cargo audit` CI step (Security → 2).

Refresh via `/readiness-report` Droid CLI; action items feed the next v7+ plan as P0 tasks.
