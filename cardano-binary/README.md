# cardano-binary (Rust)

`cardano-binary` is the canonical CBOR toolkit used across the Cardano stack.
It mirrors the behaviour of the Haskell
[`cardano-binary`](https://github.com/IntersectMBO/cardano-base/tree/master/cardano-binary)
package so that ledger types, networking payloads, and test fixtures encode and
decode byte-for-byte the same way in Rust.

## Module map

| Rust path | Purpose | Haskell source |
| --- | --- | --- |
| `cardano_binary` (crate root) | Re-exports the high-level API surface (`serialize`, `decode_full`, nested helpers) | [`Cardano.Binary`](https://github.com/IntersectMBO/cardano-base/blob/master/cardano-binary/src/Cardano/Binary.hs) |
| `serialize` | Canonical CBOR encoders, buffer reuse, semantic tag 24 helpers | [`Cardano.Binary.Serialize`](https://github.com/IntersectMBO/cardano-base/blob/master/cardano-binary/src/Cardano/Binary/Serialize.hs) |
| `deserialize` | Total decoders, leftover detection, nested tag 24 decoders, legacy unsafe helpers | [`Cardano.Binary.Decode`](https://github.com/IntersectMBO/cardano-base/blob/master/cardano-binary/src/Cardano/Binary/Decode.hs) |
| `error` | Error type equivalent to Haskell `DecoderError`, capturing leftovers, tag mismatches, and IO failures | [`Cardano.Binary.Decoder.Error`](https://github.com/IntersectMBO/cardano-base/blob/master/cardano-binary/src/Cardano/Binary/Decoder/Error.hs) |

Refer to `HASKELL_MAPPING.md` for the full symbol-by-symbol translation.

## Quick start

```rust
use cardano_binary::{decode_full, encode_nested_cbor, serialize};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
struct Demo {
        id: u64,
        tag: String,
}

let value = Demo { id: 7, tag: "hello".into() };
let bytes = serialize(&value)?;
let decoded: Demo = decode_full(&bytes)?;
assert_eq!(decoded, value);

let nested = encode_nested_cbor(&value)?;
let nested_decoded: Demo = cardano_binary::decode_nested_cbor(&nested)?;
assert_eq!(nested_decoded, value);
```

## Feature highlights

- **Canonical CBOR output** – Deterministic encoding that satisfies
    RFC 8949 §4.2 (smallest integer form, definite lengths, sorted map keys,
    no duplicates).
- **Nested CBOR helpers** – Encode/Decode semantic tag 24 payloads used by the
    ledger and networking protocols.
- **Leftover-aware decoding** – `decode_full` reports trailing bytes through
    `BinaryError::Leftover` so deserialisation boundaries stay explicit.
- **Allocation-aware APIs** – `serialize_into_vec` and
    `serialize_with_capacity` reuse buffers for tight loops or pre-sizing.
- **Extensive parity testing** – 86 tests covering golden vectors, Haskell
    cross-validation, property-based roundtrips, and fuzzed CBOR fragments.

## Canonical encoding contract

The encoder enforces the canonical rules automatically, but collections need to
arrive in the right order. Prefer deterministic containers (`BTreeMap`, sorted
vectors) when serialising maps:

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert("zebra", 1);
map.insert("apple", 2);
map.insert("mango", 3);

let bytes = serialize(&map)?; // Keys encode in byte-order: "apple", "mango", "zebra"
```

If you work with `Vec<(K, V)>`, make sure to sort before encoding:

```rust
let mut entries = vec![("zebra", 1), ("apple", 2), ("mango", 3)];
entries.sort_by_key(|(key, _)| *key);
let canonical = serialize(&entries)?;
```

## Nested CBOR payloads

Some protocol messages embed CBOR inside CBOR (tag 24). The helper functions
wrap and unwrap the tagged payload while validating the structure:

```rust
let inner_bytes = vec![0x01, 0x02, 0x03];
let tagged = cardano_binary::encode_nested_cbor_bytes(&inner_bytes)?;
let roundtrip = cardano_binary::decode_nested_cbor_bytes(&tagged)?;
assert_eq!(roundtrip, inner_bytes);
```

Errors differentiate between the wrong tag (`BinaryError::NestedTag`) and an
unexpected payload type (`BinaryError::NestedPayload`).

## Error handling

All APIs return `Result<_, BinaryError>`. Besides serialization/deserialization
wrappers around `ciborium`, notable cases include:

- `BinaryError::Leftover` – exposes the label, leftover slice, and length so
    higher-level decoders can surface actionable messages.
- `BinaryError::NestedTag` – carries both the expected and observed tag IDs.
- `BinaryError::NestedPayload` – signals that the inner CBOR object was not a
    byte string.

Deprecated helpers (`unsafe_deserialize*`) mirror the historical Haskell API and
will be removed once downstream code migrates to fallible decoding.

## Testing and verification

```bash
cargo test -p cardano-binary
```

The suite spans:

- `tests/cbor_compatibility.rs` – coverage over CBOR major/minor types.
- `tests/golden_tests.rs` – fixed hex fixtures tied to the Haskell repository.
- `tests/haskell_cross_validation.rs` – roundtrips against Haskell outputs.
- `tests/proptest_roundtrip.rs` – property tests for structural types.

CI executes these alongside the rest of the workspace to guard byte-level
parity.

## Benchmarks

```bash
cargo bench -p cardano-binary --bench cbor_bench
```

Criterion benchmarks report throughput for representative payloads (small and
large structs, vector-heavy data, large maps). HTML reports live under
`target/criterion/cbor_bench/`.

## Related crates

- [`cardano-base`](../cardano-base/README.md) – feature flags and base types
    that often serialise through this crate.
- [`cardano-slotting`](../cardano-slotting/README.md) – slot/epoch primitives
    with CBOR instances backed by `cardano-binary`.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE](./LICENSE) or
    <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE](./LICENSE) or <http://opensource.org/licenses/MIT>)

at your option.
