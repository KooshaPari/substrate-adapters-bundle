//! `phenotype-manifest` — minimal v0.1 schema + validator for the
//! `odin.nvms` manifest format shared by BytePort, OmniRoute, and
//! NanoVMS/PhenoCompose.
//!
//! Scope of this crate (v0.1):
//!
//! - [`Manifest`] — Rust struct with `serde::Deserialize` covering the
//!   six top-level fields required by v0.1
//!   (`app`, `infra`, `network`, `observability`, `agent`, `portfolio`).
//! - [`validate`] — one-call `&str -> Result<Manifest, ManifestError>`
//!   using `serde_json` for structural validation and a small manual
//!   pass for semantic checks (e.g. `app.name` non-empty).
//!
//! Out of scope for v0.1 (deferred to v0.2):
//!
//! - JSON Schema emission via `schemars` (stretch goal — skip if not
//!   trivial).
//! - Strict jsonschema-based validation.
//! - YAML ingestion (JSON only for v0.1).
//! - Schema versioning / `version` discriminator on `Manifest`.

#![forbid(unsafe_code)]

mod schema;

pub use schema::{
    validate, Agent, App, Infra, Manifest, ManifestError, Network, Observability, Portfolio,
    Resources,
};
