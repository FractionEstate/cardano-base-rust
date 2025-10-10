# cardano-base (Rust)

`cardano-base` provides the strongly typed representation of experimental
Cardano protocol feature flags. The crate mirrors the behaviour of the Haskell
module [`Cardano.Base.FeatureFlags`](https://github.com/IntersectMBO/cardano-base/blob/master/cardano-base/src/Cardano/Base/FeatureFlags.hs)
and keeps the API intentionally small: parse flag names coming from on-chain or
configuration sources, serialise them back to JSON, and inspect which features
are enabled inside your node or tooling.

## Module map

| Rust path | Purpose | Haskell source |
| --- | --- | --- |
| `cardano_base` (crate root) | Declares the `CardanoFeatureFlag` enum, parsing helpers, and error types. | `Cardano.Base.FeatureFlags` |

## Quick start

```rust
use cardano_base::{parse_flags, CardanoFeatureFlag};

let flags = parse_flags(["Leios", "Phalanx"])?;
assert_eq!(flags, vec![CardanoFeatureFlag::Leios, CardanoFeatureFlag::Phalanx]);
```

### Flag catalogue

| Flag | Historical alias | Description |
| --- | --- | --- |
| `Leios` | `LeiosFlag` | Higher-throughput Ouroboros schedule tuned for pipelining. |
| `Peras` | `PerasFlag` | Faster settlement variant designed to reduce finality latency. |
| `Phalanx` | `PhalanxFlag` | Anti-grinding improvements that resist adaptive adversaries. |

Additions follow the order above to maintain compatibility with
golden-tests from the Haskell implementation.

## Parsing and validation

- ✅ `CardanoFeatureFlag::from_str` accepts either the short or historical
  alias.
- ✅ `parse_flags` consumes any iterator of strings and returns a `Vec` in the
  original order.
- ✅ `parse_flag_case_insensitive` is available for lenient user input (CLI,
  config files). The helper falls back to the strict parser so unknown values
  still yield a `ParseFeatureFlagError::UnknownFlag`.

The error type implements `std::error::Error`, `Clone`, `Eq`, and
`Display`, making it ergonomic to bubble up through larger parsing pipelines.

## Serialisation

The enum derives `serde::Serialize`/`Deserialize`, so feature sets round-trip
cleanly through JSON payloads:

```rust
use cardano_base::CardanoFeatureFlag;

let json = serde_json::to_string(&CardanoFeatureFlag::Peras)?;
assert_eq!(json, "\"Peras\"");
```

If you need case-insensitive inputs when decoding, use
`parse_flag_case_insensitive` before deserialising into downstream types to
retain explicit error handling.

## Integration tips

- 🧰 The crate has no features; include `cardano-base = "0.1"` and the default
  serde support is ready to go.
- 🧩 For configuration files, parse the user-facing strings into
  `CardanoFeatureFlag` values, then enable the matching protocol behaviour.
- 🛡️ When validating API requests, expose the error message from
  `ParseFeatureFlagError` verbatim so operators can spot typos quickly.

## Migration notes

The Rust API mirrors the original Haskell surface, so migrating consumers from
`Cardano.Base.FeatureFlags` is mostly mechanical:

1. **Replace imports** – swap Haskell module references for the crate root
  (`use cardano_base::{CardanoFeatureFlag, parse_flags};`). All enum
  constructors keep the same name and case as the Haskell data constructors.
2. **Preserve parsing semantics** – `parse_flags` accepts iterators of string
  types (`&str`, `String`, `Vec<&str>`). It yields flags in the same order as
  the input, matching the `FromJSON` instance. Unknown values still produce a
  descriptive error (`ParseFeatureFlagError::UnknownFlag`).
3. **Handle historic aliases** – the Rust parser recognises both the modern
  (`"Leios"`) and legacy (`"LeiosFlag"`) spellings. Downstream code that
  expected the alias to parse in Haskell will continue to do so.
4. **JSON round-trips** – replace `aeson` encoding/decoding with `serde_json`.
  The derived serde implementations use the same canonical casing, so existing
  golden tests translate directly.
5. **Config loaders** – when porting CLI or configuration readers, call
  `parse_flag_case_insensitive` to keep the “accept lowercase keywords” UX
  Cardano operators relied on. The helper still funnels unknown values into
  the strict parser, maintaining identical error text.

For cross-language assurance, compare the outputs of the Haskell
`featureFlagsExamples` property tests with the Rust unit tests in
`tests/parse.rs`. Both suites cover permutation, duplicate, and alias cases.

## Testing

Run unit tests locally with:

```bash
cargo test -p cardano-base
```

The suite covers serde round-trips, strict vs case-insensitive parsing, and the
list iterator helper. Add extra tests when extending the enum to guarantee
behaviour stays aligned with the original Haskell implementation.

## Related crates

- [`cardano-binary`](../cardano-binary/README.md) for canonical CBOR
  serialisation
- [`cardano-slotting`](../cardano-slotting/README.md) when connecting flags to
  slot scheduling primitives

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE](./LICENSE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE](./LICENSE) or <http://opensource.org/licenses/MIT>)

at your option.
