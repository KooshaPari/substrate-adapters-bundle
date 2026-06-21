# pheno-context — CONTRIBUTING.md

> **Repo tier:** `pheno-*-lib` (per ADR-023).
> **Sibling artifact:** see [`SPEC.md`](./SPEC.md) for the companion spec.
> **Meta-bundle target:** `AGENTS.md` + `llms.txt` + `WORKLOG.md` + `CHANGELOG.md` + `CONTRIBUTING.md` + `STATUS.md` + `SPEC.md` + `LICENSE-MIT` (per ADR-042 substrate quality bar).

## 1. Quickstart (5 lines)

```bash
gh repo fork pheno-context && cd pheno-context
git checkout -b feat/<req-id>-<slug>-$(date +%Y-%m-%d)   # or chore/<req-id>-...
git commit -m "feat(scope): description"                 # Conventional Commits
gh pr create --base main --head <branch> --fill
```

## 2. Branching strategy

| Type    | Prefix                          | When                                            |
| ------- | ------------------------------- | ----------------------------------------------- |
| Feature | `feat/<req-id>-<slug>-<date>`   | New user-facing capability, API addition        |
| Chore   | `chore/<req-id>-<slug>-<date>`  | Refactor, governance, docs, CI, deps            |
| Fix     | `fix/<req-id>-<slug>-<date>`    | Bug fix on a shipped path                       |
| Spike   | `spike/<req-id>-<slug>-<date>`  | Time-boxed investigation; do NOT merge code     |

`<req-id>` = fleet DAG level `L<n>-<seq>` (e.g. `L5-115`, `T15.5`); `<date>` = `YYYY-MM-DD`.

## 3. Commit conventions

Use [Conventional Commits](https://www.conventionalcommits.org/) with a scope. Subject ≤ 72 chars, imperative mood.

- `feat(scope):` / `fix(scope):` / `refactor(scope):` — capability, bug fix, no behavior change
- `perf(scope):` / `test(scope):` / `docs(scope):` — perf, tests, docs only
- `build(scope):` / `ci(scope):` / `chore(scope):` — build, CI, non-src maintenance

## 4. PR template

```markdown
## What / Why / How
<!-- 1-3 sentences each. Link the issue / ADR / DAG level L<n>-<seq>. -->

## Test plan
<!-- [ ] unit  [ ] integ  [ ] e2e  [ ] manual
     [ ] coverage >= 80% on touched paths  [ ] lint clean  [ ] WORKLOG.md updated (v2.1)
     [ ] labels: governance | L<n>-#<n>  +  area:<scope> -->

## Risk
<!-- Blast radius + rollback plan. -->
```

## 5. Review process

- **Reviewers:** 1 CODEOWNER on the touched path + 1 cross-area reviewer. SLA: first review ≤ 1 business day, re-review ≤ 4 hours of push.
- **Merge:** squash-merge by default; rebase-merge only for multi-commit feature branches.
- **Self-merge:** permitted for `governance` + `L<n>-#<n>` + `area:docs` + `area:ci` PRs (Track 8 post-mortem). All others need explicit human approval.
- **No force-push** to `main` or to PRs after review has started.

## 6. Testing gates

A PR is mergeable only when **all** of the following pass:

- [ ] Tests + coverage: `cargo test --workspace` green, ≥ 80% line coverage on touched paths (ADR-023 Rule 3.1, lib/SDK gate).
- [ ] Lint: `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all -- --check`.
- [ ] `cargo deny check` advisories clear (no new RUSTSEC, no banned-license drift).
- [ ] `WORKLOG.md` updated per [ADR-025 v2.1](https://github.com/KooshaPari/phenotype-apps/blob/main/docs/adr/2026-06-15/ADR-025-worklog-v2-1.md) — 11-column schema including `device:` field.
- [ ] No new `unwrap()` / `panic!` in lib crates (allowed in `#[cfg(test)] mod tests` + `bin/` only).
- [ ] All 7 ADR-042 quality-bar elements present (spec, README, test matrix, observability, coverage, CI gate, worklog v2.1). For governance PRs that only touch docs, this reduces to "no element is removed".

## 7. Release process

1. Bump version in `crates/pheno-context/Cargo.toml` (workspace-relative `version.workspace = true` is updated at the workspace root `Cargo.toml`) per SemVer; move unreleased `CHANGELOG.md` entries under the new version.
2. Open release PR: `chore/<req-id>-release-v<X>.<Y>.<Z>-<date>`; after merge, tag `v<X>.<Y>.<Z>` and push to trigger release workflow. Hotfixes cut a `fix/...` branch from the released tag and back-merge to `main` immediately.
3. Publish to crates.io as `pheno-context`; verify docs.rs renders the new version.

## 8. Support

- **Questions:** open a [Discussion](https://github.com/KooshaPari/pheno-context/discussions) — not an issue.
- **Bugs:** open an [Issue](https://github.com/KooshaPari/pheno-context/issues) with the PR template above.
- **Security:** see `SECURITY.md` — do NOT file public issues for vulns.
