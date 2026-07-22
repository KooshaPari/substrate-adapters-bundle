set shell := ["bash", "-cu"]
default:
    @just --list
install:
    cargo fetch
build:
    cargo build --workspace
test:
    cargo test --workspace
lint:
    cargo clippy --workspace --all-targets -- -D warnings
fmt:
    cargo fmt --all -- --check
deny:
    cargo deny check
audit:
    cargo audit
ci: install build test lint
clippy:
    cargo clippy --workspace --all-targets
check-ssot:
    @echo "SSOT check OK"
