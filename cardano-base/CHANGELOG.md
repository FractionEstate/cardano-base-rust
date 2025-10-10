# Changelog

All notable changes to `cardano-base` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Distribution-grade README with module mapping, parsing guidance, serde
	examples, and integration tips linked to the Haskell origin.
- Migration notes section outlining how downstream crates can migrate from the
	Haskell feature flag helpers to the Rust implementation.

### Changed
- Converted this changelog to Keep a Changelog format and documented the
	alignment work for feature flag primitives.

## 0.1.0

### Added
- Initial Rust port of `Cardano.Base.FeatureFlags`; exposes the
	`CardanoFeatureFlag` enum, strict/case-insensitive parsing helpers, serde
	support, and error reporting identical to the Haskell implementation.
