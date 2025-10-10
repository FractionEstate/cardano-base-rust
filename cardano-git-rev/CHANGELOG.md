# Changelog

All notable changes to `cardano-git-rev` are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Expanded the README with the Haskell module mapping, build-script workflow,
	troubleshooting guidance, and end-to-end testing instructions to make the
	crate distribution ready.
- Adopted Keep a Changelog structure for this file so future releases can be
	tracked consistently.

## [0.1.0] - 2025-10-08

### Added
- Initial Rust port of `cardano-git-rev`, mirroring the behaviour of the
	Haskell `Cardano.Git.Rev` module while remaining pure Rust.
- Build script that surfaces the Git revision through the `CARDANO_GIT_REV`
	environment variable and embeds the `_cardano_git_rev` symbol for Nix
	patching.
- Runtime API (`git_rev`) with fallbacks that match the Haskell implementation
	plus test-only guards for overriding the embedded revision and Git command.
