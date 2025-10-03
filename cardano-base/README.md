# cardano-base (Rust)

`cardano-base` is now a Rust crate that provides the strongly typed
representation of experimental Cardano protocol feature flags.

```rust
use cardano_base::{parse_flags, CardanoFeatureFlag};

let flags = parse_flags(["Leios", "Phalanx"])?;
assert_eq!(flags, vec![CardanoFeatureFlag::Leios, CardanoFeatureFlag::Phalanx]);
```

The enum derives `serde::Serialize`/`Deserialize`, making it trivial to embed in
configuration files or pass across FFI boundaries. Parsing accepts both the
historic names (`"LeiosFlag"`) and the modern short aliases (`"Leios"`). A
case-insensitive helper is also available:

```rust
use cardano_base::parse_flag_case_insensitive;

let flag = parse_flag_case_insensitive("peras")?;
assert_eq!(flag, CardanoFeatureFlag::Peras);
```

## Testing

Run the crate's unit tests with:

```bash
cargo test -p cardano-base
```

> **Note:** The shared development container currently lacks a Rust toolchain.
> Install Rust (e.g. via `rustup`) before executing the command.
