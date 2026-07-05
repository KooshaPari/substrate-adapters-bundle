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
| `phenotype-manifest` | `odin.nvms` v0.2 schema + validator + JSON Schema emission (Rust → Draft 7 / 2020-12) | BytePort, OmniRoute, NanoVMS |

## Layout

```
phenotype-shared/
├── Cargo.toml                  # workspace root
├── Cargo.lock                  # committed
├── .gitignore                  # excludes /target/ + per-crate Cargo.lock
├── README.md                   # this file
├── schemas/
│   └── odin.nvms.schema.json   # generated artifact (cargo run --bin dump-schema)
└── crates/
    └── phenotype-manifest/
        ├── Cargo.toml
        ├── src/
        │   ├── lib.rs          # re-exports
        │   ├── schema.rs       # types + validate() + schema_json()
        │   └── bin/
        │       └── dump_schema.rs  # CLI: write schema to file or stdout
        ├── examples/
        │   └── web-service.nvms.json
        └── tests/
            ├── validate.rs        # 4 integration tests
            └── schema_validation.rs # 5 jsonschema-based tests
```

## Building & Testing

```bash
cd phenotype-shared
cargo test --workspace       # 9 integration tests + 3 doc-tests
cargo build --release        # release build of lib + dump-schema binary
cargo run --bin dump-schema  # regenerate schemas/odin.nvms.schema.json
```

The workspace requires Rust **1.75+** (`rust-version = "1.75"`).

## Adoption Recipe (downstream repo)

**Rust** (BytePort, OmniRoute Rust tools, NanoVMS):

```toml
# In BytePort/Cargo.toml (or OmniRoute/Cargo.toml):
[dependencies]
phenotype-manifest = { path = "../phenotype-shared/crates/phenotype-manifest" }
```

```rust
use phenotype_manifest::{validate, Manifest, ManifestError};

fn load(path: &std::path::Path) -> Result<Manifest, ManifestError> {
    let json = std::fs::read_to_string(path)?;
    validate(&json)
}
```

**Go** (BytePort backend, NVMS service):

```go
import "github.com/xeipuuv/gojsonschema"

// Load the committed schema artifact (or fetch from `cargo run --bin dump-schema`)
schemaBytes, _ := os.ReadFile("../phenotype-shared/schemas/odin.nvms.schema.json")
schemaLoader := gojsonschema.NewBytesLoader(schemaBytes)
documentLoader := gojsonschema.NewGoLoader(manifest)
result, _ := gojsonschema.Validate(schemaLoader, documentLoader)
if !result.Valid() { /* reject */ }
```

**TypeScript** (OmniRoute, BytePort UI):

```typescript
import Ajv2020 from "ajv/dist/2020";
import schema from "../../phenotype-shared/schemas/odin.nvms.schema.json";

const ajv = new Ajv2020();
const validate = ajv.compile(schema);
if (!validate(manifest)) throw new Error("invalid odin.nvms");
```

## Roadmap

- **v0.1 ✅** `phenotype-manifest` — JSON-only, manual semantic pass. 4 integration tests green.
- **v0.2 ✅** `phenotype-manifest` — JSON Schema emission via `schemars`; `dump-schema` CLI; jsonschema validation round-trip (9 tests + 3 doc-tests green); committed `schemas/odin.nvms.schema.json`.
- **v0.3 (next):** `phenotype-port-adapter-shim` re-exporting `pheno-port-adapter` traits (when adoption begins).
- **v0.4:** `phenotype-tracing-shim` re-exporting `pheno-tracing` OTel init.
- **v0.5:** `phenotype-errors-shim` re-exporting `pheno-errors` error types.
- **v0.6:** `phenotype-nvms-adapter` — Rust shim wrapping NanoVMS APIs for the BytePort control plane.

See `plans/2026-07-04-ecosystem-unified-vision-v1.md` §6 for the broader context.