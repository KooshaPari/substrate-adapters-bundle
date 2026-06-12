# Functional Requirements

## Overview
PhenoUtils is the shared Rust utility workspace for the Phenotype ecosystem.
It provides foundational crates for CLI shells, filesystem abstractions,
cryptographic operations, network utilities, and testing helpers.

## Requirements

| ID | Title | Description | Priority | Status |
|----|-------|-------------|----------|--------|
| FR-001 | CLI shell framework | Interactive shell builder with command parsing, completions, and history management. | High | Backlog |
| FR-002 | Filesystem abstractions | Async file I/O, recursive operations, atomic writes, and path helpers. | High | Backlog |
| FR-003 | Cryptographic operations | Hashing, encryption, signing, HMAC, and key derivation helpers. | High | Backlog |
| FR-004 | Network utilities | TCP/UDP helpers, connection pooling, DNS resolution, and TLS setup. | Medium | Backlog |
| FR-005 | Testing helpers | Fixtures, temporary directories, mock implementations, and property generators. | Medium | Backlog |
| FR-006 | Error handling | Rich error types with context and consistent `?`-friendly propagation. | Low | Backlog |
| FR-007 | Performance baseline | Zero-dependency where possible, efficient string handling, and benchmark coverage. | Low | Backlog |

## Test Traceability

| FR | Test File | Test Name | Status |
|----|-----------|-----------|--------|
| FR-001 | `crates/pheno-shell/src/lib.rs` | not yet mapped | Pending |
| FR-002 | `crates/pheno-fs/src/lib.rs` | `atomic_write_replaces_contents_and_leaves_no_temp_file`, `compute_hash_returns_sha256_hex_digest` | Covered |
| FR-003 | `crates/pheno-crypto/src/lib.rs` | `hmac_sha256_matches_known_vector`, `base64_decode_rejects_invalid_input`, `random_bytes_returns_requested_length` | Covered |
| FR-004 | `crates/pheno-net/src/lib.rs` | not yet mapped | Pending |
| FR-005 | `crates/pheno-testing/src/lib.rs` | not yet mapped | Pending |
| FR-006 | `tests/smoke_test.rs` | `smoke_test_loads` | Covered |
| FR-007 | `docs/BENCHMARKS.md` | not yet mapped | Pending |
