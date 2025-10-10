# Changelog

All notable changes to `cardano-strict-containers` are documented here. The
format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the
crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Expanded README with distribution guidance, finger tree examples, and
	Haskell↔Rust lookup table.
- Documented feature flag configuration (`serde` default) and cross-crate
	integration hooks with `nothunks`/`heapwords`.

## 0.1.0 – 2025-10-02

### Added
- Reimplemented the original Haskell package as a Rust crate while preserving
	parity for `StrictMaybe`, `StrictSeq`, and `StrictFingerTree`.
- Ported measurement/search helpers (`add_measure`, `bin_measure`, `search`,
	`split`, `ViewL`, `ViewR`) and basic algebraic traits (`Measured`,
	`Semigroup`, `Monoid`).
- Brought over unit tests and crate-level documentation to ensure behavioural
	compatibility.
