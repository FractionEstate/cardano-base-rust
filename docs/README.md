# cardano-base-rust Documentation# Cardano Base - Documentation

Welcome to the comprehensive documentation for the **cardano-base-rust** project - a pure Rust implementation of Cardano's foundational cryptographic primitives.This directory contains comprehensive documentation for the Cardano Base Rust project.

## 📚 Documentation Structure## 📚 Documentation Structure

### 🚀 Getting Started### [Home](Home.md)

- [**Main README**](../README.md) – Project overview, quick start, and installationMain documentation hub and quick start guide.

- [**Contributing Guide**](../CONTRIBUTING.md) – How to contribute to the project

- [**Code of Conduct**](../CODE-OF-CONDUCT.md) – Community guidelines### API Documentation

- [**Security Policy**](../SECURITY.md) – Security practices and vulnerability reporting

- [Package Overview](api/Packages.md) - Overview of all workspace packages

### 📦 API Documentation- [VRF API](api/VRF-API.md) - Verifiable Random Function API reference

- [**API Reference**](api/) – Detailed API documentation for all packages### Migration Guides

  - Auto-generated from Rust docs via `cargo doc`

  - Access locally: `cargo doc --workspace --no-deps --open`- [Migration Summary](migration/Migration-Summary.md) - Complete Haskell → Rust migration overview

  - Online: <https://docs.rs/cardano-crypto-class>- [VRF Implementation](migration/VRF-Implementation.md) - Pure Rust VRF conversion details

### 🔐 Cryptography### Development

- [**Cryptography Guide**](development/CRYPTOGRAPHY.md) – Implementation details for VRF, KES, and DSIGN- [Research Notes](development/Research-Notes.md) - Technical research and decisions

- [**VRF Implementation**](development/VRF.md) – IETF Draft-03 and Draft-13 compliance- [Development Plan](development/Development-Plan.md) - Project roadmap and implementation plan

- [**KES Implementation**](development/KES.md) – Key evolving signatures with forward secrecy- [Testing Guide](development/Testing-Guide.md) - How to run tests and verify correctness

- [**DSIGN Implementation**](development/DSIGN.md) – Ed25519 digital signatures

### Contributing

### 🔍 Audit & Verification

- [Contributing Guide](contributing/CONTRIBUTING.md) - How to contribute to this project

- [**Audit Reports**](audit/) – Comprehensive security audits and verification reports- [Code of Conduct](contributing/CODE-OF-CONDUCT.md) - Community guidelines

  - [**Cross-Validation Report**](audit/CROSS_VALIDATION_REPORT.md) – **⭐ Key Document** – Proves Haskell binary compatibility- [Security Policy](contributing/SECURITY.md) - Security vulnerability reporting

  - [**Comprehensive Checklist**](audit/COMPREHENSIVE_AUDIT_CHECKLIST.md) – Complete verification checklist (100%)

  - [**Component Verification**](audit/) – Individual verification reports for VRF, KES, DSIGN, CBOR## 🚀 Quick Links

### 🔄 Migration- **Main README**: [../README.md](../README.md)

- **Changelog**: [../CHANGELOG.md](../CHANGELOG.md)

- [**Migration Guide**](migration/) – Guide for migrating from Haskell cardano-base- **Release Process**: [../RELEASING.md](../RELEASING.md)

  - CBOR Compatibility

  - Type Mapping- API documentation with code examples

  - API Differences- Contributing guidelines

  - Performance Considerations

## 🎯 Getting Started

### 🛠️ Development

1. Start with [Home](Home.md)

- [**Development Guide**](development/) – Technical development documentation2. Review the [Package Overview](api/Packages.md)

  - [**Publishing Guide**](development/PUBLISH_GUIDE.md) – How to publish crates to crates.io3. Check the [Migration Summary](migration/Migration-Summary.md) for conversion details

  - [**Release Process**](development/RELEASING.md) – Versioning and release procedures

  - Architecture decisions## 📚 Documentation Paths

  - Testing strategies

  - Performance optimization notes1. [Home](Home.md) - Overview and installation

2. [API Documentation](api/) - Package references

### 🤝 Contributing3. [Migration Guides](migration/) - Haskell → Rust conversion

4. [Development](development/) - Research and implementation notes

- [**Contributing Guidelines**](contributing/) – Detailed contribution workflow5. [Contributing](contributing/) - How to contribute

  - Setup development environment

  - Code style and standards## 🔄 CI/CD

  - Pull request process

  - Issue reportingThis documentation is automatically synchronized to the GitHub Wiki via GitHub Actions.

## 📖 Quick LinksSee `.github/workflows/sync-wiki.yml` for the sync configuration

### For Users

- Want to **use** the library? → [Main README](../README.md)
- Need **API docs**? → Run `cargo doc --workspace --open`
- Found a **bug**? → [GitHub Issues](https://github.com/FractionEstate/cardano-base-rust/issues)
- **Security concern**? → See [SECURITY.md](../SECURITY.md)

### For Contributors

- Want to **contribute**? → [CONTRIBUTING.md](../CONTRIBUTING.md)
- Need **development setup**? → [Development Guide](development/)
- Want to **understand** the code? → [Architecture docs](development/)
- Publishing a **new version**? → [Publishing Guide](development/PUBLISH_GUIDE.md)

### For Auditors

- Need **audit reports**? → [Audit Documentation](audit/)
- Want **Haskell compatibility proof**? → [Cross-Validation Report](audit/CROSS_VALIDATION_REPORT.md)
- Need **test coverage**? → See [Audit Checklist](audit/COMPREHENSIVE_AUDIT_CHECKLIST.md)
- Want **security review**? → [Security Audits](audit/)

## 🎯 Key Documentation

### Must-Read Documents

1. **[Main README](../README.md)** – Start here for project overview
2. **[Cross-Validation Report](audit/CROSS_VALIDATION_REPORT.md)** – Proof of Haskell compatibility
3. **[Security Policy](../SECURITY.md)** – Security practices and reporting
4. **[Contributing Guide](../CONTRIBUTING.md)** – How to contribute

### Cryptography Deep Dives

- VRF: Verifiable Random Functions (IETF Draft-03 & Draft-13)
- KES: Key Evolving Signatures (forward-secure)
- DSIGN: Digital Signatures (Ed25519, RFC 8032)
- Hashing: Blake2b, Blake2s, SHA-256, Keccak-256

### Testing & Verification

- **234 tests** across all components (100% passing)
- **Haskell cross-validation** – 30 CBOR hex-comparison tests
- **IETF compliance** – 14 VRF test vectors
- **Property-based testing** – 194 KES property tests
- **Security audits** – Multiple review passes

## 📊 Project Status

| Component | Tests | Documentation | Audit Status |
|-----------|-------|---------------|--------------|
| VRF | ✅ 34 | ✅ Complete | ✅ Audited |
| KES | ✅ 200 | ✅ Complete | ✅ Audited |
| DSIGN | ✅ 5 | ✅ Complete | ✅ Audited |
| CBOR | ✅ 41 | ✅ Complete | ✅ Audited |
| Slotting | ✅ 17 | ✅ Complete | ✅ Audited |
| Utilities | ✅ 37 | ✅ Complete | ✅ Audited |
| **TOTAL** | **✅ 234** | **✅ 100%** | **✅ Certified** |

## 🔧 Building Documentation

### Generate API Docs

```bash
# All packages
cargo doc --workspace --no-deps --open

# Specific package
cargo doc --package cardano-crypto-class --open

# With private items
cargo doc --workspace --document-private-items --open
```

### View Documentation Locally

```bash
# Serve documentation with a local server
cd target/doc
python3 -m http.server 8000

# Open browser to http://localhost:8000
```

## 🌐 Online Resources

- **GitHub Repository**: <https://github.com/FractionEstate/cardano-base-rust>
- **Original Haskell**: <https://github.com/IntersectMBO/cardano-base>
- **Rust Docs (when published)**: <https://docs.rs/cardano-crypto-class>
- **Crates.io (when published)**: <https://crates.io/crates/cardano-crypto-class>

## 📝 Documentation Maintenance

This documentation is maintained alongside the code. When making changes:

1. Update relevant docs in the same PR
2. Run `cargo doc` to verify doc generation
3. Check for broken links and outdated info
4. Update version numbers if needed

## 🙏 Acknowledgments

This documentation structure is inspired by the Rust community's best practices and the original Haskell cardano-base documentation.

---

*For questions or suggestions about documentation, please [open an issue](https://github.com/FractionEstate/cardano-base-rust/issues).*
