---
layout: default
title: Home
---

![100% Pure Rust](https://img.shields.io/badge/Pure_Rust-100%25-orange.svg)
![Tests](https://img.shields.io/badge/Tests-227_Passing-brightgreen.svg)
![Zero C Dependencies](https://img.shields.io/badge/C_Dependencies-0-blue.svg)
![Security Hardened](https://img.shields.io/badge/Security-Hardened-green.svg)

## Complete Pure Rust implementation of Cardano base libraries

This project provides the foundational libraries for Cardano blockchain applications, fully migrated from Haskell to pure Rust with zero C dependencies.

## 🎯 Key Achievements

- ✅ **227 tests passing** (100% success rate)
- ✅ **Zero C code** (removed 26 files, 9,716 lines)
- ✅ **Zero Haskell dependencies** (100% Rust)
- ✅ **Pure Rust VRF** (IETF Draft-03 & Draft-13 compliant)
- ✅ **Cryptographically verified** (cross-validated against Haskell implementation)
- ✅ **Fully audited** (comprehensive security review completed)

## 📦 Packages

### Core Cryptographic

- **cardano-crypto-class** – Cryptographic primitives (VRF, hashing, signatures)
- **cardano-vrf-pure** – Pure Rust VRF implementation using curve25519-dalek

### Data & Serialization

- **cardano-base** – Base types and utilities
- **cardano-binary** – CBOR encoding/decoding
- **cardano-slotting** – Time and slot management
- **cardano-strict-containers** – Strict evaluation containers

### Utilities

- **base-deriving-via** – Generic deriving helpers
- **deepseq** – Deep evaluation utilities
- **nothunks** – Thunk detection for space leak prevention
- **heapwords** – Heap allocation tracking
- **measures** – Measurement abstractions
- **orphans-deriving-via** – Orphan instance helpers
- **cardano-git-rev** – Git revision tracking

## 🚀 Quick Start

```bash

# Add to Cargo.toml

[dependencies]
cardano-binary = "0.1.0"
cardano-crypto-class = "0.1.0"
cardano-vrf-pure = "0.1.0"

# Build and test

cargo build --workspace
cargo test --workspace

```

## 📚 Documentation

### For Users

- [Getting Started Guide](guides/getting-started/) - Installation and basic usage
- [API Documentation](api/) - Comprehensive API reference
- [Migration Guide](guides/migration/) - Migrating from Haskell libraries

### For Contributors

- [Contributing Guide](contributing/CONTRIBUTING) - How to contribute
- [Development Guide](guides/development/) - Development setup and workflow
- [Testing Guide](guides/testing/) - Running and writing tests

### Audit & Verification

- [Audit Reports](audit/) - Comprehensive security and correctness audits
- [Cross-Validation Results](audit/cross-validation-summary/) - Haskell compatibility verification
- [CBOR Compatibility](audit/cbor-compatibility/) - Serialization format verification

## 🔬 Technical Highlights

### VRF Implementation

Pure Rust implementation of Verifiable Random Functions:

- IETF Draft-03 (ECVRF-ED25519-SHA512-ELL2)
- IETF Draft-13 (ECVRF-ED25519-SHA512-TAI)
- Uses curve25519-dalek v4.1 for elliptic curve operations
- Cross-validated against Haskell libsodium implementation
- Zero unsafe code in critical cryptographic paths

### CBOR Serialization

Standards-compliant CBOR implementation:

- RFC 8949 compliant using ciborium
- Byte-exact compatibility with Haskell cborg
- 30 cross-validation tests passing
- Support for canonical serialization

## 📊 Project Statistics

- **Total Packages**: 13
- **Total Tests**: 227
- **Lines of Rust**: ~15,000+
- **C Code Removed**: 9,716 lines (26 files)
- **Test Success Rate**: 100%
- **Production Readiness**: 95%

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](contributing/CONTRIBUTING) for details.

## 📝 License

This project is licensed under Apache 2.0. See [LICENSE](https://github.com/FractionEstate/cardano-base-rust/blob/master/LICENSE) for details.

## 🔗 Links

- [GitHub Repository](https://github.com/FractionEstate/cardano-base-rust)
- [Issue Tracker](https://github.com/FractionEstate/cardano-base-rust/issues)
- [Changelog](https://github.com/FractionEstate/cardano-base-rust/blob/master/CHANGELOG.md)
- [Security Policy](contributing/SECURITY)
