//! Shim crate for `substrate::context-budget`.
//!
//! This crate re-exports the canonical adapter from the substrate workspace
//! so it can be consumed via the [`substrate-adapters-bundle`] workspace as a
//! single-version-pin unit. See the canonical source at:
//! <https://github.com/KooshaPari/substrate/tree/main/crates/context-budget>
//!
//! [`substrate-adapters-bundle`]: https://github.com/KooshaPari/substrate-adapters-bundle

pub use context_budget::*;
