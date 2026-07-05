# `phenotype-shared` — Shared Workspace

> **Workspace for cross-repo Phenotype primitives.** Lives at the root of the
> `Phenotype/repos` monorepo as a sibling to `BytePort/`, `OmniRoute/`,
> `PhenoCompose/`, `nanovms/`, etc.

## Purpose

Holds **additive Rust crates** that are shared by ≥2 sibling repos (BytePort,
OmniRoute, PhenoCompose, NanoVMS) but don't belong to any single one. Each
crate is intentionally **standalone-friendly**: each `crates/<name>/` can be
adopted by a downstream repo via `cargo` workspace member or `path = ...`
dependency without dragging the rest of this workspace along.

## Crates

| Crate | Purpose | Adopted by |
|---|---|---|
| `phenotype-manifest` | `odin.nvms` v0.1 schema + validator (JSON, `serde_json`, manual semantic pass) | BytePort, OmniRoute, NanoVMS |

## Layout

```
phenotype-shared/
├── Cargo.toml                  # workspace root
├── Cargo.lock                  # committed
├── .gitignore                  # excludes /target/ + per-crate Cargo.lock
├── README.md                   # this file
└── crates/
    └── phenotype-manifest/
        ├── Cargo.toml
        ├── src/
        │   ├── lib.rs          # re-exports
        │   └── schema.rs       # types + validate()
        ├── examples/
        │   └── web-service.nvms.json
        └── tests/
            └── validate.rs     # 4 integration tests
```

## Adoption Recipe (downstream repo)

```toml
# In BytePort/Cargo.toml (or OmniRoute/Cargo.toml):
[dependencies]
phenotype-manifest = { path = "../phenotype-shared/crates/phenotype-manifest" }
```

```rust
// In the adopting crate's Rust code:
use phenotype_manifest::{validate, Manifest, ManifestError};

fn load(path: &std::path::Path) -> Result<Manifest, ManifestError> {
    let json = std::fs::read_to_string(path)?;
    validate(&json)
}
```

## Building & Testing

```bash
cd phenotype-shared
cargo test --workspace
cargo build --release
```

The workspace requires Rust **1.75+** (`rust-version = "1.75"`).

## Adding a New Crate

1. Create `crates/<new-name>/{Cargo.toml,src/lib.rs}`.
2. Add the path to `members = [...]` in the workspace `Cargo.toml`.
3. Add shared deps to `[workspace.dependencies]` if used by ≥2 crates.
4. Add tests under `crates/<new-name>/tests/` (TDD-first).
5. Update this README's **Crates** table.

## Roadmap

- **v0.1 (this commit):** `phenotype-manifest` — JSON-only, manual semantic pass.
- **v0.2 (next):** JSON Schema emission via `schemars`; YAML ingestion; v0.1/v0.2 discriminator on `Manifest`.
- **v0.3:** `phenotype-port-adapter-shim` re-exporting `pheno-port-adapter` traits (when adoption begins).
- **v0.4:** `phenotype-tracing-shim` re-exporting `pheno-tracing` OTel init.
- **v0.5:** `phenotype-errors-shim` re-exporting `pheno-errors` error types.

See `plans/2026-07-04-ecosystem-unified-vision-v1.md` §6 for the broader context.