# Changelog for `cardano-binary`

## 0.1.0 (2025-10-02)

- Replaced the legacy Haskell package with a Rust crate powered by
  `serde_cbor`.
- Added helper functions for serialisation (`serialize`,
  `serialize_into_writer`, `serialize_into_vec`, `serialize_with_capacity`,
  `encode_nested_cbor`) and deserialisation
  (`decode_full`, `deserialise_decoder`, `decode_nested_cbor`).
- Introduced the `BinaryError` type capturing trailing bytes, nested tag
  mismatches, and underlying CBOR errors.
