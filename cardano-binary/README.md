# cardano-binary

Rust helpers for Cardano CBOR serialisation and deserialisation. The crate
wraps `serde`/`serde_cbor` with convenience functions that mimic the original
Haskell API:

- `serialize`, `serialize_strict`, `serialize_into_vec`, and
  `serialize_with_capacity` turn values into CBOR byte vectors while allowing
  buffer reuse.
- `decode_full` consumes an entire CBOR payload, reporting trailing bytes.
- `encode_nested_cbor` / `decode_nested_cbor` work with semantic tag 24 for
  CBOR-in-CBOR structures.
- `deserialise_decoder` provides incremental decoding when a caller needs to
  retain the leftover bytes.

All functions return the `BinaryError` type, exposing trailing data, nested tag
mismatches, and underlying `serde_cbor` failures.

## Usage

Add the crate to your workspace and rely on the helper functions:

```rust
use cardano_binary::{serialize, decode_full, encode_nested_cbor, decode_nested_cbor};

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
let nested_decoded: Demo = decode_nested_cbor(&nested)?;
assert_eq!(nested_decoded, value);
```

## Testing

Run the crate tests with:

```bash
cargo test -p cardano-binary
```

The repository container currently lacks a Rust toolchain; install one (for
example via `rustup`) before running the command.
