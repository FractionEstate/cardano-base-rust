# cardano-strict-containers changelog

## 0.1.0 (2025-10-02)

- Reimplemented the library as a Rust crate while preserving the public
	surface area of the original Haskell package.
- Added `StrictMaybe`, `StrictSeq`, and `StrictFingerTree` modules with parity
	helper utilities and serde support.
- Ported unit tests to assert behavioural compatibility and added crate-level
	documentation.
