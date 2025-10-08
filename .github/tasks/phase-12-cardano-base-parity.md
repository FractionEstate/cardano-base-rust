# Phase 12 – Cardano Base Crate Parity

**Status:** ☐ Not started  \
**Primary owners:** _Unassigned_  \
**Supporting crates:** `cardano-base`

---

## Objective

Port and validate the core `cardano-base` crate from Haskell to Rust, providing the unified interface and re-exports that tie together all other crates in the workspace. This crate serves as the primary entry point for downstream applications.

## Success Criteria

- All sub-crates correctly re-exported
- Module structure mirrors Haskell layout
- Documentation provides clear entry points
- Examples demonstrate common use cases
- Integration tests validate cross-crate interactions
- Performance benchmarks establish baseline for full stack

## Scope

### Core Functionality

1. **Module Re-exports**
   - `cardano_base::crypto` → `cardano-crypto-class`
   - `cardano_base::binary` → `cardano-binary`
   - `cardano_base::slotting` → `cardano-slotting`
   - `cardano_base::containers` → `cardano-strict-containers`
   - `cardano_base::vrf` → `cardano-vrf-pure`
   - And all helper crates

2. **Prelude Module**
   - Common imports for downstream users
   - Standard traits and types
   - Ergonomic re-exports

3. **Utilities**
   - Common helper functions
   - Type aliases
   - Error types

### Out of Scope (Future Phases)

- Network protocol implementation
- Ledger state machine
- Consensus layer

---

## Milestone Checklist

### 1. Audit & Analysis

- [ ] Compare Rust implementation against Haskell `cardano-base`
  - `Cardano.Crypto.*` modules
  - `Cardano.Binary.*` modules
  - `Cardano.Slotting.*` modules
  - Re-export structure

- [ ] Document module hierarchy
  - Which Haskell modules map to which Rust crates
  - Re-export strategy (flat vs hierarchical)
  - Breaking changes from Haskell

- [ ] Identify missing pieces
  - Modules not yet ported
  - Functionality gaps
  - Deferred features

### 2. Module Structure

- [ ] Define top-level module hierarchy
  ```rust
  pub mod crypto {
      pub use cardano_crypto_class::*;
      // Additional re-exports or aliases
  }

  pub mod binary {
      pub use cardano_binary::*;
  }

  pub mod slotting {
      pub use cardano_slotting::*;
  }

  pub mod containers {
      pub use cardano_strict_containers::*;
  }

  pub mod vrf {
      pub use cardano_vrf_pure::*;
  }

  // Helper crates
  pub mod deepseq { ... }
  pub mod heapwords { ... }
  pub mod measures { ... }
  pub mod nothunks { ... }

  // Deriving-via
  pub mod deriving_via {
      pub use base_deriving_via::*;
      pub use orphans_deriving_via::*;
  }
  ```

- [ ] Consider re-export granularity
  - Re-export everything (easy but pollutes namespace)
  - Re-export selectively (more control, more maintenance)
  - Hybrid approach (prelude + full modules)

### 3. Prelude Module

- [ ] Create `cardano_base::prelude` module
  ```rust
  pub mod prelude {
      // Common traits
      pub use crate::crypto::{Hash, DSIGN, KES, VRF};
      pub use crate::binary::{Serialize, Deserialize};
      pub use crate::deriving_via::{Semigroup, Monoid};

      // Common types
      pub use crate::slotting::{SlotNo, EpochNo};
      pub use crate::containers::{StrictSeq, StrictMap};

      // Error types
      pub use crate::error::*;

      // Utilities
      pub use crate::util::*;
  }
  ```

- [ ] Document prelude usage
  - When to use `use cardano_base::prelude::*;`
  - What's included and why
  - How to opt out of specific imports

### 4. Error Handling

- [ ] Define unified error type
  ```rust
  #[derive(Debug, thiserror::Error)]
  pub enum CardanoBaseError {
      #[error("Cryptographic error: {0}")]
      Crypto(#[from] cardano_crypto_class::Error),

      #[error("Serialization error: {0}")]
      Binary(#[from] cardano_binary::Error),

      #[error("Slotting error: {0}")]
      Slotting(#[from] cardano_slotting::Error),

      // ... other variants
  }
  ```

- [ ] Conversion traits
  - `From` impls for sub-crate errors
  - `Into<anyhow::Error>` for flexibility

### 5. Utilities

- [ ] Common helper functions
  - Hex encoding/decoding
  - Base64 encoding/decoding
  - Pretty printing for crypto types

- [ ] Type aliases
  ```rust
  pub type Bytes = Vec<u8>;
  pub type ByteString = bytes::Bytes;
  pub type Text = String;
  ```

- [ ] Traits
  - `ToHex`, `FromHex`
  - `Display` impls for pretty printing

### 6. Integration Tests

- [ ] Cross-crate interaction tests
  - Serialize crypto keys with `cardano-binary`
  - Hash serialized data with `cardano-crypto-class`
  - Slot arithmetic with `cardano-slotting`
  - Store in `StrictMap` from `cardano-strict-containers`

- [ ] Real-world scenarios
  - Create and verify VRF proof
  - Sign data with DSIGN, serialize with CBOR
  - Calculate slot for timestamp
  - Validate NoThunks for strict containers

- [ ] Error propagation tests
  - Errors from sub-crates convert correctly
  - Error messages are clear
  - Stack traces preserved

### 7. Examples

- [ ] Basic usage examples
  - `examples/crypto_basics.rs` - Hash, sign, verify
  - `examples/cbor_encoding.rs` - Serialize/deserialize
  - `examples/slot_calculations.rs` - Time/slot conversions
  - `examples/strict_containers.rs` - Using StrictSeq/StrictMap

- [ ] Advanced examples
  - `examples/full_workflow.rs` - Complete use case
  - `examples/error_handling.rs` - Ergonomic error handling
  - `examples/custom_types.rs` - Deriving traits for custom types

### 8. Performance Benchmarks

- [ ] Full-stack benchmarks
  - End-to-end workflow (create data → sign → serialize → hash)
  - Memory usage
  - Allocation patterns

- [ ] Comparison with Haskell
  - Equivalent operations
  - Throughput metrics
  - Latency measurements

### 9. Documentation

- [ ] Comprehensive README
  - Overview of cardano-base
  - Quick start guide
  - Link to sub-crate docs
  - Common use cases
  - Migration from Haskell

- [ ] API documentation
  - All public modules, functions, types
  - Usage examples in doc comments
  - Links to relevant specs

- [ ] Architecture doc
  - Module hierarchy diagram
  - Re-export strategy rationale
  - Design decisions

- [ ] Haskell → Rust migration guide
  - Module mapping table
  - Common patterns
  - Breaking changes
  - Performance notes

- [ ] Update workspace README
  - Link to cardano-base as main entry point
  - Overview of all crates
  - Getting started section

- [ ] Update CHANGELOG

### 10. Release Preparation

- [ ] Version alignment
  - Ensure all sub-crates have compatible versions
  - Set cardano-base version appropriately
  - Update Cargo.toml dependencies

- [ ] Feature flags
  - Optional dependencies (e.g., `serde` support)
  - Debug/release optimizations
  - Test-only features

- [ ] Cargo.toml metadata
  - Authors, license, repository
  - Keywords, categories
  - README link

- [ ] CI/CD
  - Run all tests
  - Check documentation builds
  - Validate examples compile and run
  - Verify benchmarks run

---

## Verification Checklist

- [ ] `cargo test -p cardano-base` green
- [ ] All examples compile and run
- [ ] Integration tests validate cross-crate interactions
- [ ] Benchmarks establish baseline
- [ ] Documentation complete and renders correctly
- [ ] No warnings in `cargo clippy`
- [ ] `cargo fmt` passes
- [ ] `cargo doc --open` shows comprehensive docs

---

## Dependencies & References

### Haskell Source
- `cardano-base/cardano-base/src/Cardano/*.hs`
- Module re-export structure
- Test suite: `cardano-base/cardano-base/test/*.hs`

### Specifications
- None specific (this crate is organizational)

### Rust Implementation
- `cardano-base/src/lib.rs` - Main entry point
- `cardano-base/src/prelude.rs` - Prelude module
- `cardano-base/src/error.rs` - Unified error type
- `cardano-base/src/util.rs` - Helper utilities
- `cardano-base/examples/*.rs` - Usage examples

### Related Crates (All Dependencies)
- `cardano-crypto-class` (Phase 03-06)
- `cardano-binary` (Phase 07)
- `cardano-slotting` (Phase 08)
- `cardano-strict-containers` (Phase 09)
- `cardano-vrf-pure` (Phase 03)
- `deepseq`, `heapwords`, `measures`, `nothunks` (Phase 10)
- `base-deriving-via`, `orphans-deriving-via` (Phase 11)

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Sub-crate APIs change during integration | Breaking changes, rework | Establish stable APIs early, semantic versioning |
| Re-export strategy too permissive | Namespace pollution, confusion | Use prelude for common items, selective re-exports |
| Unified error type too complex | Hard to use, noisy | Keep error variants minimal, use `source()` for details |
| Examples out of sync with API | Misleading documentation | Include examples in CI, test as part of build |
| Performance regression in full stack | User-facing slowdowns | Benchmark early, profile hot paths |

---

## Estimated Effort

- **Audit & Planning**: 0.5 days
- **Module Structure**: 0.5 days
- **Prelude & Error Handling**: 1 day
- **Utilities**: 0.5 days
- **Integration Tests**: 2 days
- **Examples**: 2 days
- **Benchmarks**: 1 day
- **Documentation**: 2-3 days
- **Release Preparation**: 1 day
- **Total**: 10-11 days (approximately 2 weeks)

**Note**: This assumes all sub-crates (Phase 03-11) are complete. If any are pending, this phase cannot proceed.

---

## Reporting Cadence

Update the **Status** line and tick checkboxes as work progresses.
- (YYYY-MM-DD) INIT: Phase 12 created.
- (YYYY-MM-DD) PROGRESS: Completed [module structure/prelude/examples].
- (YYYY-MM-DD) COMPLETE: All checklist items finished, cardano-base ready for release.

---

## Related Work

- **Completed**: Phase 03 (VRF), Phase 04 (DSIGN), Phase 05 (KES), Phase 06 (Hash), Phase 07 (CBOR)
- **In Progress**: Phase 08 (Slotting), Phase 09 (Strict Containers), Phase 10 (Helper Crates), Phase 11 (Deriving-Via)
- **Blocks**: This phase cannot start until Phase 08-11 are complete

---

_This document lives at `.github/tasks/phase-12-cardano-base-parity.md`. Update it after every meaningful change._
