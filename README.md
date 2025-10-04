# cardano-base-rust

[![Pure Rust](https://img.shields.io/badge/Pure_Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-234_Passing-brightgreen.svg)](https://github.com/FractionEstate/cardano-base-rust/actions)
[![Zero C Dependencies](https://img.shields.io/badge/C_Dependencies-0-blue.svg)](Cargo.toml)
[![License](https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE)
[![Security Hardened](https://img.shields.io/badge/Security-Audited-green.svg)](docs/audit/)

> **Pure Rust implementation of Cardano's foundational cryptographic primitives**

A complete, production-ready Rust port of [Haskell cardano-base](https://github.com/IntersectMBO/cardano-base), providing cryptographic foundations for Cardano blockchain applications with 100% memory safety and zero unsafe code in critical paths.

## ✨ Features

- 🦀 **100% Pure Rust** – No C or Haskell dependencies, fully native Rust implementation
- 🔐 **Production-Grade Crypto** – VRF (IETF Draft-03/13), KES, DSIGN, and more
- ✅ **Haskell-Compatible** – Binary-compatible with original Haskell implementation
- 🧪 **234 Tests Passing** – Comprehensive test coverage including cross-validation with Haskell
- 🔒 **Security Audited** – Multiple security audits and hardening passes completed
- �� **13 Modular Packages** – Use only what you need

## 🚀 Quick Start

**Prerequisites:**

- Rust 1.70 or later ([install via rustup](https://rustup.rs/))
- Cargo (comes with Rust)

**Installation:**

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
cardano-crypto-class = "0.1"
cardano-binary = "0.1"
cardano-vrf-pure = "0.1"
```

**Example Usage:**

```rust
use cardano_crypto_class::hash::Blake2b256;
use cardano_binary::{Serialize, Deserialize};

// Blake2b hashing
let data = b"Hello, Cardano!";
let hash = Blake2b256::hash(data);

// CBOR serialization (Haskell-compatible)
let encoded = my_data.serialize()?;
let decoded = MyType::deserialize(&encoded)?;
```

**Building from Source:**

```bash
git clone https://github.com/FractionEstate/cardano-base-rust.git
cd cardano-base-rust
cargo build --workspace --release
cargo test --workspace
cargo doc --workspace --no-deps --open
```

## 📚 Documentation

- [**📖 API Documentation**](docs/api/) – Comprehensive API reference for all packages
- [**🔐 Cryptography Guide**](docs/development/CRYPTOGRAPHY.md) – VRF, KES, and DSIGN implementations
- [**🔄 Migration Guide**](docs/migration/) – Migrating from Haskell cardano-base
- [**🛡️ Security Practices**](SECURITY.md) – Security policies and vulnerability reporting
- [**🤝 Contributing**](CONTRIBUTING.md) – How to contribute to this project

## 📦 Workspace Packages

### Core Cryptographic Libraries

| Package | Description | Version |
|---------|-------------|---------|
| [**cardano-crypto-class**](cardano-crypto-class/) | Main cryptographic primitives (VRF, KES, DSIGN, hashing) | 0.1.0 |
| [**cardano-vrf-pure**](cardano-vrf-pure/) | Pure Rust VRF (IETF Draft-03 & Draft-13 compliant) | 0.1.0 |

### Core Data & Serialization

| Package | Description | Version |
|---------|-------------|---------|
| [**cardano-base**](cardano-base/) | Base types and common utilities | 0.1.0 |
| [**cardano-binary**](cardano-binary/) | CBOR encoding/decoding with Haskell compatibility | 0.1.0 |
| [**cardano-slotting**](cardano-slotting/) | Time and slot management for blockchain | 0.1.0 |
| [**cardano-strict-containers**](cardano-strict-containers/) | Strict evaluation containers | 0.1.0 |

### Utility Libraries

| Package | Description | Version |
|---------|-------------|---------|
| [**base-deriving-via**](base-deriving-via/) | Generic deriving helpers | 0.1.0 |
| [**deepseq**](deepseq/) | Deep evaluation utilities | 0.1.0 |
| [**nothunks**](nothunks/) | Thunk detection for space leak prevention | 0.1.0 |
| [**heapwords**](heapwords/) | Heap allocation tracking | 0.1.0 |
| [**measures**](measures/) | Measurement abstractions | 0.1.0 |
| [**orphans-deriving-via**](orphans-deriving-via/) | Orphan instance helpers | 0.1.0 |
| [**cardano-git-rev**](cardano-git-rev/) | Git revision tracking for builds | 0.1.0 |

## 🔐 Cryptographic Primitives

### VRF (Verifiable Random Functions)

Pure Rust implementation of IETF VRF standards:

- **ECVRF-ED25519-SHA512-ELL2** (Draft-03) – 80-byte proofs
- **ECVRF-ED25519-SHA512-TAI** (Draft-13) – 128-byte proofs with batch compatibility
- Powered by `curve25519-dalek` v4.1 for security and performance
- 14 IETF test vectors validated + 20+ property-based tests

### KES (Key Evolving Signatures)

Forward-secure signatures with period-based key evolution:

- Binary tree structure for efficient key evolution
- Multiple hash algorithms: Blake2b-256, Blake2b-512, SHA-256
- 194 property-based tests ensuring correctness
- Haskell-compatible serialization

### DSIGN (Digital Signatures)

Ed25519 signatures via `ed25519-dalek`:

- RFC 8032 compliant
- Batch verification support
- Zero-copy operations where possible

### Hashing

Multiple cryptographic hash functions:

- **Blake2b** (256-bit and 512-bit variants)
- **Blake2s** (256-bit)
- **SHA-256**
- **Keccak-256**

See [CRYPTOGRAPHY.md](docs/development/CRYPTOGRAPHY.md) for detailed implementation notes.

## 🛠️ Development

### Building and Testing

```bash
cargo check --workspace
cargo test --workspace
cargo test --workspace -- --nocapture
cargo test --package cardano-crypto-class
cargo build --workspace --release
```

### Code Quality and Linting

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo audit
cargo deny check
```

### Documentation

```bash
cargo doc --workspace --no-deps --open
cargo doc --package cardano-vrf-pure --open
```

### Quality Standards

The workspace enforces strict quality standards:

- **Clippy lints**: `correctness=deny`, `unwrap_used=warn`, `panic=warn`
- **Format checking**: rustfmt configuration enforced
- **Security scanning**: Automated vulnerability detection via `cargo audit`
- **License compliance**: Dependency license validation via `cargo deny`
- **Pre-commit checks**: See [PRE_COMMIT_CHECKLIST.md](PRE_COMMIT_CHECKLIST.md)

See [SECURITY_PRACTICES.md](SECURITY_PRACTICES.md) for detailed security guidelines.

## 🤝 Contributing

We welcome contributions! Before contributing, please:

1. Read our [Contributing Guide](CONTRIBUTING.md)
2. Review the [Code of Conduct](CODE-OF-CONDUCT.md)
3. Check our [Security Policy](SECURITY.md) for security-related contributions
4. Follow the [Pre-Commit Checklist](PRE_COMMIT_CHECKLIST.md)

### Reporting Issues

- **Security vulnerabilities**: See [SECURITY.md](SECURITY.md) for responsible disclosure
- **Bug reports**: Use GitHub Issues with detailed reproduction steps
- **Feature requests**: Open a discussion in GitHub Discussions first

## 📄 License

This project is dual-licensed under:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- **MIT License** ([LICENSE-MIT](LICENSE) or <http://opensource.org/licenses/MIT>)

You may choose either license for your use. See individual package `LICENSE` files for details.

## 🎯 Project Status

| Aspect | Status |
|--------|--------|
| **Migration** | ✅ 100% Complete (Haskell → Rust) |
| **Tests** | ✅ 234/234 Passing (100%) |
| **Security** | ✅ Multiple audits completed |
| **Haskell Compatibility** | ✅ Binary-compatible CBOR |
| **Production Ready** | ✅ Yes |

## 🔗 Links

- **GitHub Repository**: [FractionEstate/cardano-base-rust](https://github.com/FractionEstate/cardano-base-rust)
- **Original Haskell Implementation**: [IntersectMBO/cardano-base](https://github.com/IntersectMBO/cardano-base)
- **Issues & Discussions**: [GitHub Issues](https://github.com/FractionEstate/cardano-base-rust/issues)
- **Security Policy**: [SECURITY.md](SECURITY.md)

## 🙏 Acknowledgments

This project is a pure Rust port of the Haskell [`cardano-base`](https://github.com/IntersectMBO/cardano-base) library maintained by [Intersect MBO](https://github.com/IntersectMBO). We maintain binary compatibility with the original implementation while providing the safety and performance benefits of Rust.

---

Built with ❤️ in pure Rust for the Cardano ecosystem
