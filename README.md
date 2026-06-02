> **Work state:** ACTIVE · **Progress:** `█████████░ 85%`
> Five Rust primitive crates (crypto/fs/net/shell/testing); the primitives tier under the SDKs. · updated 2026-06-02

> **Pinned references (Phenotype-org)**
> - MSRV: see rust-toolchain.toml
> - cargo-deny config: see deny.toml
> - cargo-audit: rustsec/audit-check@v2 weekly
> - Branch protection: 1 reviewer required, no force-push
> - Authority: phenotype-org-governance/SUPERSEDED.md

# PhenoUtils

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![FR Coverage](https://github.com/KooshaPari/phenoUtils/actions/workflows/fr-coverage.yml/badge.svg)](https://github.com/KooshaPari/phenoUtils/actions/workflows/fr-coverage.yml)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

Essential utilities and foundational crates for the Phenotype ecosystem. Provides CLI shells, filesystem abstractions, cryptographic operations, network utilities, and testing helpers used across all Phenotype services and tools.

## Overview

**PhenoUtils** is the foundational utilities library for the Phenotype platform, providing battle-tested implementations of common patterns: interactive shells, filesystem abstractions with async support, cryptographic operations, network utilities, and comprehensive testing helpers. All crates are zero-dependency where possible, thoroughly tested, and designed for high-performance, production use.

**Core Mission**: Eliminate utility boilerplate across Phenotype services by providing reusable, well-tested foundational crates that handle complexity transparently.

## Technology Stack

- **Language**: Rust (edition 2021)
- **Async Runtime**: Tokio for async filesystem and network operations
- **Cryptography**: ring for cryptographic primitives, argon2 for password hashing
- **Testing**: criterion for benchmarking, proptest for property testing
- **Build**: Cargo workspace with shared dependency versions

## Key Features

- **CLI Shell Framework**: Interactive shell builder with command parsing, completions, history
- **Filesystem Utilities**: Async file I/O, recursive operations, atomic writes, path utilities
- **Cryptographic Operations**: Hashing, encryption, signing, HMAC, key derivation
- **Network Utilities**: TCP/UDP helpers, connection pooling, DNS resolution, TLS support
- **Testing Helpers**: Fixtures, temporary files/directories, mock implementations, property generators
- **Error Handling**: Rich error types with context, automatic `?` operator support
- **Performance**: Zero-copy where possible, memory pooling, efficient string handling

## Quick Start

```bash
# Clone and explore
git clone <repo-url>
cd phenoUtils

# Review governance and architecture
cat CLAUDE.md          # Project governance
cat AGENTS.md          # Agent operating contract

# Build all crates
cargo build --workspace

# Run comprehensive test suite
cargo test --workspace

# Run benchmarks
cargo bench --workspace

# Lint and format
cargo clippy --workspace -- -D warnings
cargo fmt --check

# Explore crate examples
ls -la crates/
```

## Project Structure

```
phenoUtils/
├── crates/
│   ├── pheno-shell/            # Interactive CLI shell framework
│   │   ├── src/
│   │   │   ├── shell.rs        # Shell builder and REPL
│   │   │   ├── parser.rs       # Command parsing
│   │   │   ├── completions.rs  # Tab completion engine
│   │   │   └── history.rs      # Command history management
│   │   └── examples/
│   ├── pheno-fs/               # Async filesystem abstractions
│   │   ├── src/
│   │   │   ├── file.rs         # Async file operations
│   │   │   ├── dir.rs          # Directory traversal
│   │   │   ├── atomic.rs       # Atomic write semantics
│   │   │   └── permissions.rs  # Fine-grained access control
│   │   └── tests/
│   ├── pheno-crypto/           # Cryptographic operations
│   │   ├── src/
│   │   │   ├── hash.rs         # Hashing (SHA-256, BLAKE3)
│   │   │   ├── encrypt.rs      # Symmetric encryption (AES-256)
│   │   │   ├── sign.rs         # Digital signatures (Ed25519)
│   │   │   ├── kdf.rs          # Key derivation (Argon2)
│   │   │   └── hmac.rs         # Message authentication
│   │   └── benches/
│   ├── pheno-net/              # Network utilities
│   │   ├── src/
│   │   │   ├── tcp.rs          # TCP connection helpers
│   │   │   ├── udp.rs          # UDP utilities
│   │   │   ├── pool.rs         # Connection pooling
│   │   │   ├── dns.rs          # DNS resolution
│   │   │   └── tls.rs          # TLS configuration
│   │   └── tests/
│   ├── pheno-testing/          # Testing utilities
│   │   ├── src/
│   │   │   ├── fixtures.rs     # Test fixture builders
│   │   │   ├── tempdir.rs      # Temporary directories
│   │   │   ├── mocks.rs        # Mock implementations
│   │   │   └── generators.rs   # Property test generators
│   │   └── tests/
│   └── Cargo.toml              # Workspace manifest
├── docs/
│   ├── ARCHITECTURE.md         # Design and patterns
│   ├── CRATE_GUIDE.md          # Per-crate usage guide
│   └── BENCHMARKS.md           # Performance characteristics
├── benches/
│   ├── crypto_perf.rs
│   ├── fs_perf.rs
│   └── network_perf.rs
└── Cargo.toml                  # Root workspace config
```

## Crate Reference

| Crate | Purpose | Stability |
|-------|---------|-----------|
| **pheno-shell** | Interactive CLI shell builder | Stable |
| **pheno-fs** | Async filesystem abstractions | Stable |
| **pheno-crypto** | Cryptographic operations | Stable |
| **pheno-net** | Network utilities and pooling | Stable |
| **pheno-testing** | Testing helpers and mocks | Stable |

## Related Phenotype Projects

- **PhenoLibs**: Shared data structures and algorithms
- **phenotype-tooling**: CLI tools built on pheno-shell
- **Tracera**: Observability (uses pheno-net for metrics export)
- **phenotype-ops-mcp**: MCP server (uses pheno-crypto for token management)

## License

MIT — see [LICENSE](./LICENSE).