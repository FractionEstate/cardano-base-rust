# Changelog

All notable changes to `cardano-slotting` are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Refreshed README with slot/epoch examples, Haskell↔Rust lookup table, and
  integration notes for JSON/serde consumers.

### Changed
- Documented existing epoch-info debug behaviour and testing coverage.

## 0.2.0.2

### Added
- Updated `EpochInfo` debug representation stub to align with the Haskell
  `Show` instance.
- Added unit tests covering `fixed_epoch_info` and
  `unsafe_linear_extend_epoch_info` helper behaviour.

## 0.2.0.1

### Changed
- Dropped support for GHC ≤ 9.4 in the upstream codebase (no-op for Rust port).

## 0.2.0.0

### Added
- Ported `EpochInterval` and `add_epoch_interval` from the ledger code.
- Introduced `bin_op_epoch_no` to unify arithmetic on `EpochNo`.
- Extracted `Test.Cardano.Slotting.TreeDiff`-equivalent utilities into the
  `testlib` module (Rust tests replicate the behaviour).

### Removed
- Numeric instances (`Num`, `Integral`, `Real`) for `EpochNo` and `EpochSize`
  in favour of explicit helpers; legacy support remains in the `testlib`
  feature scope.

## 0.1.1.1

### Added
- GHC 9.6 compatibility work mirrored in the Rust port (no API changes).

## 0.1.1.0

### Added
- JSON instances for `WithOrigin`, `BlockNo`, `SystemStart`, `RelativeTime`,
  and `SlotLength` to match the Haskell package.

### Changed
- Removed the legacy `development` Cabal flag (not applicable to Rust, noted
  for parity).

## 0.1.0.1

### Added
- Initial release.
