# Cardano Base Packages

This document provides an overview of all 13 pure Rust packages in the cardano-base workspace.

## Core Cryptographic Packages

### cardano-crypto-class
**Location**: `cardano-crypto-class/`

The main cryptographic library providing:
- VRF (Verifiable Random Functions) implementations
- Hash functions (Blake2b, Blake2s, SHA-256, etc.)
- KES (Key Evolving Signatures)
- Digital signatures
- Cryptographic primitives

**Key modules**:
- `vrf::mock` - Mock VRF for testing
- `vrf::never` - Never VRF (always fails)
- `vrf::simple` - Simple VRF implementation
- `vrf::praos` - Praos VRF (Draft-03)
- `vrf::praos_batch` - Batch-compatible Praos VRF (Draft-13)

### cardano-vrf-pure
**Location**: `cardano-vrf-pure/`

Pure Rust VRF implementation with zero dependencies on C code:
- IETF VRF Draft-03 (ECVRF-ED25519-SHA512-ELL2)
- IETF VRF Draft-13 (ECVRF-ED25519-SHA512-TAI)
- Uses curve25519-dalek v4.1 for elliptic curve operations
- Cryptographically verified with 9 internal tests

**Features**:
- Elligator2 hash-to-curve
- Prove/Verify operations
- Secure key generation
- Output hash computation

## Core Data Packages

### cardano-base
**Location**: `cardano-base/`

Base types and utilities:
- Core type definitions
- Common utilities
- Foundation abstractions

### cardano-binary
**Location**: `cardano-binary/`

Binary serialization library:
- CBOR encoding/decoding
- Custom serialization formats
- Compatibility with Cardano binary formats

### cardano-slotting
**Location**: `cardano-slotting/`

Time and slot management:
- Slot arithmetic
- Epoch calculations
- Time conversions
- Slot validation

### cardano-strict-containers
**Location**: `cardano-strict-containers/`

Strict evaluation container types:
- Strict maps
- Strict sets
- Performance-optimized collections
- Memory-efficient data structures

## Utility Packages

### base-deriving-via
**Location**: `base-deriving-via/`

Generic deriving utilities:
- `InstantiatedAt` wrapper for deriving via
- Generic representation helpers
- Borrowed representation support

### deepseq
**Location**: `deepseq/`

Deep evaluation utilities:
- `NFData` trait for deep strict evaluation
- `NFData1`, `NFData2` for higher-kinded types
- Generic evaluation helpers
- WHNF forcing

### nothunks
**Location**: `nothunks/`

Thunk detection for preventing space leaks:
- `NoThunks` trait for thunk checking
- `OnlyCheckWhnf` wrapper for shallow checks
- Generic thunk detection
- Memory safety validation

### heapwords
**Location**: `heapwords/`

Heap allocation tracking:
- Memory footprint estimation
- 64-bit heap word counting
- Type-specific size heuristics
- Support for Rust standard types

### measures
**Location**: `measures/`

Measurement abstractions:
- Generic measure traits
- Bounded measurements
- Arithmetic operations on measures
- Size and length abstractions

### orphans-deriving-via
**Location**: `orphans-deriving-via/`

Orphan instance helpers:
- Compatibility wrappers
- Test harness for deriving-via
- Bridge between traits and types

## Infrastructure Packages

### cardano-git-rev
**Location**: `cardano-git-rev/`

Git revision tracking:
- Embed git commit hash
- Build-time version information
- Source tracking

## Package Dependencies

```
cardano-crypto-class
├── cardano-vrf-pure (pure Rust VRF)
├── deepseq
├── nothunks
└── base-deriving-via

cardano-binary
├── deepseq
└── nothunks

cardano-slotting
├── cardano-base
├── deepseq
└── measures

cardano-strict-containers
├── deepseq
└── nothunks
```

## Building Packages

```bash
# Build all packages
cargo build --workspace

# Build specific package
cargo build --package cardano-vrf-pure

# Build with optimizations
cargo build --workspace --release

# Run tests for all packages
cargo test --workspace

# Run tests for specific package
cargo test --package cardano-crypto-class
```

## Documentation

Generate documentation for all packages:

```bash
# Generate and open docs
cargo doc --workspace --no-deps --open

# Generate docs for specific package
cargo doc --package cardano-vrf-pure --open
```

## Testing

Each package includes comprehensive tests:

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --package cardano-vrf-pure test_prove_verify_roundtrip
```

## Package Versioning

All packages follow semantic versioning (SemVer):
- **cardano-crypto-class**: v0.1.0
- **cardano-vrf-pure**: v0.1.0
- **cardano-base**: v0.1.0
- Others: See individual `Cargo.toml` files

## License

All packages are dual-licensed under Apache-2.0 and MIT licenses unless otherwise specified.
