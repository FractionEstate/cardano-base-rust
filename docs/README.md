# cardano-base-rust Documentation

Welcome to the comprehensive documentation for the **cardano-base-rust** project - a pure Rust implementation of Cardano's foundational cryptographic primitives.

## 📚 Documentation Structure

### 🚀 Getting Started

- [**Main README**](../README.md) – Project overview, quick start, and installation
- [**Contributing Guide**](../CONTRIBUTING.md) – How to contribute to the project
- [**Code of Conduct**](../CODE-OF-CONDUCT.md) – Community guidelines
- [**Security Policy**](../SECURITY.md) – Security practices and vulnerability reporting

### 📦 API Documentation

- [**API Reference**](api/) – Detailed API documentation for all packages
  - Auto-generated from Rust docs via `cargo doc`
  - Access locally: `cargo doc --workspace --no-deps --open`
  - Online: <https://docs.rs/cardano-crypto-class>

### 🔐 Cryptography

- [**Cryptography Guide**](development/CRYPTOGRAPHY.md) – Implementation details for VRF, KES, and DSIGN
- [**VRF Implementation**](development/VRF.md) – IETF Draft-03 and Draft-13 compliance
- [**KES Implementation**](development/KES.md) – Key evolving signatures with forward secrecy
- [**DSIGN Implementation**](development/DSIGN.md) – Ed25519 digital signatures

### 🔍 Audit & Verification

- [**KES Implementation Status**](audit/KES_STATUS.md) – **✨ Current Status** – KES implementation state (October 2025)
- [**Audit Status Update**](audit/AUDIT_STATUS_UPDATE.md) – Comparison of audit claims vs current reality
- [**Audit Reports**](audit/) – Comprehensive security audits and verification reports
  - [**Cross-Validation Report**](audit/CROSS_VALIDATION_REPORT.md) – **⭐ Key Document** – Proves Haskell binary compatibility
  - [**Comprehensive Checklist**](audit/COMPREHENSIVE_AUDIT_CHECKLIST.md) – Complete verification checklist (100%)
  - [**Component Verification**](audit/) – Individual verification reports for VRF, KES, DSIGN, CBOR
  - [**Historical Documents**](audit/README.md) – Outdated audit documents (January 2025) kept for reference

### 🔄 Migration

- [**Migration Guide**](migration/) – Guide for migrating from Haskell cardano-base
  - CBOR Compatibility
  - Type Mapping
  - API Differences
  - Performance Considerations

### 🛠️ Development

- [**Development Guide**](development/) – Technical development documentation
  - [**Publishing Guide**](development/PUBLISH_GUIDE.md) – How to publish crates to crates.io
  - [**Release Process**](development/RELEASING.md) – Versioning and release procedures
  - Architecture decisions
  - Testing strategies
  - Performance optimization notes

### 🤝 Contributing

- [**Contributing Guidelines**](contributing/) – Detailed contribution workflow
  - Setup development environment
  - Code style and standards
  - Pull request process
  - Issue reporting

## 📖 Quick Links

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

1. [Main README](../README.md) – Start here for project overview
2. [Cross-Validation Report](audit/CROSS_VALIDATION_REPORT.md) – Proof of Haskell compatibility
3. [Security Policy](../SECURITY.md) – Security practices and reporting
4. [Contributing Guide](../CONTRIBUTING.md) – How to contribute

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
cargo doc --workspace --no-deps --open
cargo doc --package cardano-crypto-class --open
cargo doc --workspace --document-private-items --open
```

### View Documentation Locally

```bash
cd target/doc
python3 -m http.server 8000
```

Then open browser to <http://localhost:8000>

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

## �� Acknowledgments

This documentation structure is inspired by the Rust community's best practices and the original Haskell cardano-base documentation.

---

*For questions or suggestions about documentation, please [open an issue](https://github.com/FractionEstate/cardano-base-rust/issues).*
