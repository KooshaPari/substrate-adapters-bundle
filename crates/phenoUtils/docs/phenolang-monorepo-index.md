# PhenoLang monorepo extraction index

`KooshaPari/PhenoLang` (private archived, ~5,200 paths) was the org monorepo. It is **not** fully superseded by [phenoUtils](https://github.com/KooshaPari/phenoUtils) alone.

## phenoUtils scope (utility crates only)

| Crate | Status |
|-------|--------|
| pheno-crypto | ✅ extracted |
| pheno-fs | ✅ extracted |
| pheno-net | ✅ extracted |
| pheno-shell | ✅ extracted |
| pheno-testing | ✅ extracted |

## Major extractions (see standalone repos)

| PhenoLang path | Successor repo |
|----------------|----------------|
| `crates/*` (21+ infrakit crates) | HexaKit |
| `AgilePlus`, `agileplus-*` | AgilePlus |
| `thegent-*`, `thegent` | thegent |
| `phenotype-*` packages | respective phenotype-* repos |
| `helios-cli`, `heliosApp` | HeliosCLI, phenotype-tooling |
| `nanovms`, `devenv-abstraction` | nanovms |
| `KodeVibeGo` | KodeVibe `engine/` |
| `Profila` | phenotype-tooling `packages/profila/` |
| `Traceon` | phenotype-otel (partial) |
| `PlatformKit` / `phenotype-go-kit` | phenotype-go-sdk |
| `worktree-manager` | PhenoVCS |

## PhenoLang branch triage (Wave H8 — 28 branches)

All `*/feat/docs-site` lanes are **docs-site sprawl** from the monorepo era — content already extracted to successor repos. Close without merge.

| Branch pattern | Count | Disposition |
|----------------|-------|-------------|
| `*/feat/docs-site` | 22 | **CLOSE** — stale docs lanes (agileplus, apps, artifacts, bifrost, clikit, crates, koosha-portfolio, packages, phench, pheno-cli, phenotype-infrakit, phenotype-router-monitor, plans, platforms, prompts, proto, python, repos, rust, scripts) |
| `chore/*` | 4 | **CLOSE** — hygiene absorbed in successor CI (cargo-deny, codeowners, pre-commit, scorecard) |
| `fix/*` | 3 | **CLOSE** — fixes landed in HexaKit / phenoUtils (`fix/phenotype-test-infra`, `fix/gitignore-v2`, `cve-residual-fix`) |
| `main` | 1 | **KEEP** archived — read-only reference |

No further git operations on PhenoLang until fleet-wide extraction map validation completes.

## Policy

- Do **not** delete PhenoLang until fleet-wide extraction map is validated.
- Do **not** unarchive — use successor repos.

Source inventory: `all_repos.txt` in archived PhenoLang tree.
