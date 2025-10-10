# Changelog

All notable changes to `heapwords` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Ported the Haskell `Cardano.HeapWords` module to safe, stable Rust.
- Implemented `HeapWords` for core standard library types and Cardano-specific
	wrappers.
- Added helpers `heap_words0` .. `heap_words9`, plus `heap_size_kb` and
	`heap_size_mb` for reporting memory usage.
- Published documentation examples and unit tests mirroring the upstream
	semantics.
- Expanded README with highlights, integration notes, crate layout, and testing
	instructions to match the workspace distribution standard.

## 0.1.0.3

### Changed
- Placeholder for last upstream Haskell release (no Rust equivalent published
	yet).

## 0.1.0.2

### Changed
- Remove `development` flag (upstream ref: IntersectMBO/cardano-base#372).

## 0.1.0.1

### Added
- Initial release.

