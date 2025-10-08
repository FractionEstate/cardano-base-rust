# Haskell → Rust Module Mapping

This document maps the Haskell `cardano-binary` modules to their Rust equivalents in the `cardano-binary` crate.

## Module Overview

| Haskell Module | Rust Module | Status | Notes |
|----------------|-------------|--------|-------|
| `Cardano.Binary.Serialize` | `serialize` | ✅ Complete | Core serialization functions |
| `Cardano.Binary.Deserialize` | `deserialize` | ✅ Complete | Core deserialization functions |
| `Cardano.Binary.FromCBOR` | Serde traits | ✅ Complete | Uses `serde::Deserialize` |
| `Cardano.Binary.ToCBOR` | Serde traits | ✅ Complete | Uses `serde::Serialize` |

## Function Mapping

### Serialization Functions

| Haskell Function | Rust Function | Notes |
|------------------|---------------|-------|
| `serialize` | `serialize()` | Produces canonical CBOR |
| `serializeWith` | `serialize_into_writer()` | Writes to custom IO |
| `toStrictByteString` | `serialize_strict()` | Strict variant |
| `serialize'` (with capacity) | `serialize_with_capacity()` | Pre-allocates buffer |
| N/A | `serialize_into_vec()` | Rust-specific: reuses Vec allocation |

### Deserialization Functions

| Haskell Function | Rust Function | Notes |
|------------------|---------------|-------|
| `decodeFull` | `decode_full()` | Ensures no leftover bytes |
| `decodeFull'` | `decode_full_owned()` | Takes owned Vec |
| `unsafeDeserialize` | `unsafe_deserialize()` | **Deprecated** - panics on error |
| `unsafeDeserialize'` | `unsafe_deserialize_owned()` | **Deprecated** - panics on error |

### Nested CBOR (Tag 24)

| Haskell Function | Rust Function | Notes |
|------------------|---------------|-------|
| `encodeNestedCbor` | `encode_nested_cbor()` | Wraps value in tag 24 |
| `encodeNestedCborBytes` | `encode_nested_cbor_bytes()` | Wraps raw bytes in tag 24 |
| `decodeNestedCbor` | `decode_nested_cbor()` | Unwraps and deserializes |
| `decodeNestedCborBytes` | `decode_nested_cbor_bytes()` | Unwraps to raw bytes |

## Type Class Mapping

### Haskell: ToCBOR
```haskell
class ToCBOR a where
  toCBOR :: a -> Encoding
```

### Rust: serde::Serialize
```rust
impl Serialize for MyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    { ... }
}
```

Or derive automatically:
```rust
#[derive(Serialize)]
struct MyType { ... }
```

### Haskell: FromCBOR
```haskell
class FromCBOR a where
  fromCBOR :: Decoder s a
```

### Rust: serde::Deserialize
```rust
impl<'de> Deserialize<'de> for MyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    { ... }
}
```

Or derive automatically:
```rust
#[derive(Deserialize)]
struct MyType { ... }
```

## Error Handling

| Haskell Error | Rust Error | Notes |
|---------------|------------|-------|
| `DeserialiseFailure` | `BinaryError::Deserialization` | CBOR decode failure |
| `LeftOverBytes` | `BinaryError::LeftoverBytes` | Trailing data after decode |
| `NestedCBORTagMismatch` | `BinaryError::NestedCborTagMismatch` | Expected tag 24 |
| `SerialiseFailure` | `BinaryError::Serialization` | CBOR encode failure |

## Canonical Encoding Rules

Both Haskell and Rust implementations enforce CBOR canonical encoding (RFC 8949 §4.2):

1. **Definite Lengths**: Use smallest possible encoding for lengths
2. **Map Key Ordering**: Keys sorted by byte-level comparison of their encodings
3. **No Duplicate Keys**: Maps must have unique keys
4. **Integer Encoding**: Smallest representation (major type 0/1 for small values)
5. **Floating Point**: Not used in Cardano (integers/bignums preferred)

### Implementation

- **Haskell**: Uses `cborg` library with canonical encoding
- **Rust**: Uses `ciborium` library which enforces canonical form by default

## Testing Strategy

### Haskell Cross-Validation

The `tests/haskell_cross_validation.rs` file contains 30+ tests that verify:
- Identical hex output for primitive types (bool, int, string, bytes)
- Matching encoding for collections (lists, tuples, maps)
- Nested CBOR (tag 24) handling
- Canonical map key ordering
- Deterministic encoding across multiple serializations

### Golden Vectors

The `tests/golden_tests.rs` file locks specific byte patterns:
- Empty collections
- Small integers (0, 42, u64::MAX)
- Negative integers (i64::MIN)
- Strings and byte arrays
- Nested structures

### Property Testing

The `tests/proptest_roundtrip.rs` file uses property-based testing:
- Random primitive values roundtrip correctly
- Complex nested structures preserve data
- Options and enums serialize deterministically

### CBOR Compatibility

The `tests/cbor_compatibility.rs` file validates:
- All major CBOR types (0-7)
- Semantic tags
- Definite vs indefinite lengths
- Canonical encoding rules
- Edge cases (max values, empty structures)

## Known Differences from Haskell

### 1. Serde-based API

Rust uses the `serde` framework instead of custom type classes. This provides:
- Automatic derivation for most types
- Integration with the broader Rust ecosystem
- Compile-time guarantees about serialization structure

### 2. Error Handling

Rust uses `Result<T, BinaryError>` instead of Haskell's exception-based approach:
- Forces explicit error handling
- No silent panics in production code
- Deprecated `unsafe_*` functions for migration compatibility

### 3. Buffer Management

Rust provides explicit buffer reuse functions:
- `serialize_into_vec()` - reuses Vec allocation
- `serialize_with_capacity()` - pre-allocates with hint
- Zero-copy deserialization where possible via `serde` framework

### 4. Type Safety

Rust's ownership system enforces:
- No accidental sharing of serialization buffers
- Lifetime tracking for borrowed data
- Compile-time prevention of use-after-free

## Migration Guide

### From Haskell to Rust

**Haskell:**
```haskell
import Cardano.Binary

data MyType = MyType
  { field1 :: Int
  , field2 :: Text
  } deriving (Generic, ToCBOR, FromCBOR)

bytes = serialize myValue
result = decodeFull @MyType bytes
```

**Rust:**
```rust
use cardano_binary::{serialize, decode_full};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyType {
    field1: i64,
    field2: String,
}

let bytes = serialize(&my_value)?;
let result: MyType = decode_full(&bytes)?;
```

### Key Changes

1. Replace `ToCBOR`/`FromCBOR` with `Serialize`/`Deserialize`
2. Use `?` operator for error propagation instead of exceptions
3. Add `&` for borrows when serializing
4. Specify type with `:` or turbofish `::` when deserializing

## Future Work

- [ ] Incremental decoding API (if needed for streaming)
- [ ] Custom CBOR semantic tags beyond tag 24
- [ ] Size calculation before serialization (for pre-allocation)
- [ ] Canonical encoding verification tool
- [ ] Performance benchmarks vs Haskell implementation

## References

- **Haskell Source**: `cardano-base/cardano-binary/src/Cardano/Binary/*.hs`
- **CBOR RFC 8949**: https://datatracker.ietf.org/doc/html/rfc8949
- **Canonical CBOR (§4.2)**: https://datatracker.ietf.org/doc/html/rfc8949#section-4.2
- **Ciborium Documentation**: https://docs.rs/ciborium/
- **Serde Documentation**: https://serde.rs/

---

*Last updated: 2025-10-08*
