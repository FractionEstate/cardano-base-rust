# cardano-base

[![100% Pure Rust](https://img.shields.io/badge/Pure_Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-148_Passing-brightgreen.svg)]()
[![Zero C Dependencies](https://img.shields.io/badge/C_Dependencies-0-blue.svg)]()

**Complete Haskell → Rust Migration: ✅ DONE**

This repository contains the pure Rust foundations that back Cardano's consensus components. The migration from Haskell to Rust is **100% complete**, with all C and Haskell code removed.

## 📚 Documentation

**Comprehensive documentation is available in the [GitHub Wiki](../../wiki)**

Quick links:
- [**📦 All Packages**](../../wiki/API-Packages) - Overview of all 13 Rust packages
- [**🔐 VRF API Reference**](../../wiki/API-VRF-API) - Pure Rust VRF implementation guide
- [**🚀 Migration Summary**](../../wiki/Migration-Summary) - Complete migration journey
- [**🛠️ Development Guide**](../../wiki/Development-Research-Notes) - Technical research and decisions
- [**🤝 Contributing**](../../wiki/Contributing-CONTRIBUTING) - How to contribute

## 🎯 Pure Rust Achievement

This project has achieved **100% pure Rust** with:
- ✅ **0 C files** (removed 26 files, 9,716 lines)
- ✅ **0 Haskell files** (100% migrated)
- ✅ **148 tests passing** (100% success rate)
- ✅ **Pure Rust VRF** (IETF Draft-03 and Draft-13 compliant)
- ✅ **Zero unsafe code** in critical paths

## 📦 Workspace Packages

### Core Cryptographic
- **cardano-crypto-class** – Main cryptographic library (VRF, hashing, signatures)
- **cardano-vrf-pure** – Pure Rust VRF implementation (curve25519-dalek)

### Core Data & Serialization
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

Each package includes comprehensive documentation, unit tests, and integration tests.

## 🚀 Quick Start

### Building

Install a stable Rust toolchain ([rustup](https://rustup.rs/)) and build:

```bash
# Build all packages
cargo build --workspace

# Build with optimizations
cargo build --workspace --release
```

### Testing

Run the comprehensive test suite (148 tests):

```bash
# All tests
cargo test --workspace

# Specific package
cargo test --package cardano-vrf-pure

# With output
cargo test --workspace -- --nocapture
```

### Documentation

Generate and view documentation:

```bash
# Generate docs for all packages
cargo doc --workspace --no-deps --open

# Specific package
cargo doc --package cardano-crypto-class --open
```

## 🔐 VRF Implementation

The pure Rust VRF implementation (`cardano-vrf-pure`) provides:

- **IETF VRF Draft-03** (ECVRF-ED25519-SHA512-ELL2, 80-byte proofs)
- **IETF VRF Draft-13** (ECVRF-ED25519-SHA512-TAI, 128-byte proofs, batch-compatible)
- **100% Pure Rust** using curve25519-dalek v4.1
- **Cryptographically Verified** with 9 internal security tests

See [VRF API Documentation](../../wiki/API-VRF-API) for detailed usage.

## 🛠️ Development

All packages target Rust edition 2021 and follow standard Rust conventions:

```bash
# Check code
cargo check --workspace

# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace --all-targets
```

## 🤝 Contributing

We welcome contributions! Please see:
- [Contributing Guide](../../wiki/Contributing-CONTRIBUTING)
- [Code of Conduct](../../wiki/Contributing-CODE-OF-CONDUCT)
- [Security Policy](../../wiki/Contributing-SECURITY)

## 📄 License

This project is dual-licensed under Apache-2.0 and MIT licenses. See `LICENSE` files in individual packages for details.

## 🎉 Migration Complete

The Haskell → Rust migration is **complete**! All functionality has been successfully ported to pure Rust with enhanced type safety, memory safety, and performance.