# substrate-adapters-bundle

[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)]()
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)]()
[![crates.io](https://img.shields.io/badge/crates.io-coming--soon-blue.svg)]()

> **Meta-repo of standalone [substrate](https://github.com/KooshaPari/substrate) adapter crates for OSS consumption.**

This bundle republishes the substrate adapter crates as a single Cargo workspace with shared release cadence and a unified `crates.io` publishing surface. The source of truth lives in [`KooshaPari/substrate`](https://github.com/KooshaPari/substrate); this repo is a **mirror** that imports the same crates via path dependencies and bundles them for downstream consumers who want one version pin.

## Crates

| Crate | Role | Upstream substrate crate |
|---|---|---|
| [`engine-agentapi`](crates/engine-agentapi) | HTTP/PTY client over the agentapi-plusplus binary family (Go). Implements `EnginePort`. | [substrate/crates/engine-agentapi](https://github.com/KooshaPari/substrate/tree/main/crates/engine-agentapi) |
| [`engine-claude`](crates/engine-claude) | Native Anthropic Claude Agent SDK adapter. | [substrate/crates/engine-claude](https://github.com/KooshaPari/substrate/tree/main/crates/engine-claude) |
| [`engine-codex`](crates/engine-codex) | OpenAI Codex CLI adapter. | [substrate/crates/engine-codex](https://github.com/KooshaPari/substrate/tree/main/crates/engine-codex) |
| [`engine-forge`](crates/engine-forge) | Generic `forge` (substrate-fleet) engine: spawn a CLI subprocess, capture logs, expose `EnginePort`. | [substrate/crates/engine-forge](https://github.com/KooshaPari/substrate/tree/main/crates/engine-forge) |
| [`omniroute-adapter`](crates/omniroute-adapter) | `RoutingPort` adapter over OmniRoute's HTTP gateway (50+ providers). | [substrate/crates/omniroute-adapter](https://github.com/KooshaPari/substrate/tree/main/crates/omniroute-adapter) |
| [`cliproxy-adapter`](crates/cliproxy-adapter) | `EnginePort` adapter over CLIProxyAPI's OpenAI-compat HTTP gateway. | [substrate/crates/cliproxy-adapter](https://github.com/KooshaPari/substrate/tree/main/crates/cliproxy-adapter) |
| [`context-budget`](crates/context-budget) | Per-conversation token-budget middleware on `EnginePort`. | [substrate/crates/context-budget](https://github.com/KooshaPari/substrate/tree/main/crates/context-budget) |
| [`substrate-trace`](crates/substrate-trace) | Trace adapters: recording, OTLP/HTTP via pheno-otel, multi-fan-out. | [substrate/crates/substrate-trace](https://github.com/KooshaPari/substrate/tree/main/crates/substrate-trace) |

## Install

This bundle is currently a **Cargo workspace**, not a published crate. To use any of the adapters, depend on them directly from the canonical substrate repo:

```toml
[dependencies]
engine-agentapi = { git = "https://github.com/KooshaPari/substrate.git" }
cliproxy-adapter = { git = "https://github.com/KooshaPari/substrate.git" }
context-budget   = { git = "https://github.com/KooshaPari/substrate.git" }
```

Once the substrate release pipeline is wired up (planned for `substrate@0.2.0`), each crate will be published individually to `crates.io` with the same version pin.

## Workspace layout

```
substrate-adapters-bundle/
├── Cargo.toml              # workspace manifest (this file)
├── README.md               # you are here
├── LICENSE-MIT
├── LICENSE-APACHE
└── crates/
    ├── engine-agentapi/
    ├── engine-claude/
    ├── engine-codex/
    ├── engine-forge/
    ├── omniroute-adapter/
    ├── cliproxy-adapter/
    ├── context-budget/
    └── substrate-trace/
```

Each `crates/<name>/` is currently a thin re-export shim around the canonical substrate crate. Once substrate switches to per-crate versioning, this bundle will be replaced by direct `crates.io` dependencies.

## Architecture role

The bundle is one of the three **distribution surfaces** of substrate:

| Surface | Audience | Source |
|---|---|---|
| `KooshaPari/substrate` | internal + integrators | monolith, 28 crates, hexagonal spine |
| `KooshaPari/substrate-adapters-bundle` (this repo) | OSS consumers who want one version pin for all adapters | mirror, single workspace |
| `KooshaPari/phenotype-router-spec` | implementers of substrate-compatible routers | protocol spec, markdown + JSON schema |

## Related repos

- [`KooshaPari/substrate`](https://github.com/KooshaPari/substrate) — the canonical substrate workspace
- [`KooshaPari/phenotype-router-spec`](https://github.com/KooshaPari/phenotype-router-spec) — router protocol spec
- [`KooshaPari/phenotype-registry`](https://github.com/KooshaPari/phenotype-registry) — registry of phenotype-related projects
- [`KooshaPari/cliproxyapi-plusplus`](https://github.com/KooshaPari/cliproxyapi-plusplus) — canonical Plus fork of `router-for-me/CLIProxyAPI`
- [`KooshaPari/OmniRoute`](https://github.com/KooshaPari/OmniRoute) — fork of `diegosouzapw/OmniRoute`

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Open a PR against [`KooshaPari/substrate`](https://github.com/KooshaPari/substrate). The bundle mirrors the canonical repo.
