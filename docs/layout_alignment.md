# Workspace Layout Alignment

This document captures how the Rust workspace maps onto the upstream
[`IntersectMBO/cardano-base`](https://github.com/IntersectMBO/cardano-base)
Haskell tree. The goal is to keep crate boundaries, naming, and supporting
assets aligned with the canonical repository so that cross-language porting
remains straightforward.

## Top-level package mapping

| Haskell package (path) | Purpose (Haskell) | Rust crate / folder | Alignment status |
| --- | --- | --- | --- |
| `base-deriving-via/` | DerivingVia helpers used across packages | `base-deriving-via/` | ✅ Names and scope match |
| `cardano-base/` | Misc base utilities (`Cardano.Base.*` modules) | `cardano-base/` | ✅ Direct port; ensure module subtrees mirror `Cardano/Base` hierarchy |
| `cardano-binary/` | CBOR serialization/deserialization | `cardano-binary/` | ✅ Structure parallels Haskell `src/Cardano/Binary` |
| `cardano-crypto-class/` | Shared crypto class typeclasses | `cardano-crypto-class/` | ✅ Crate layout mirrors upstream; tests live under `tests/` analogue to Haskell `cardano-crypto-tests` |
| `cardano-git-rev/` | Git revision embedding | `cardano-git-rev/` | ✅ Same responsibility |
| `cardano-slotting/` | Slotting / time utilities | `cardano-slotting/` | ✅ Keep module tree consistent with `Cardano.Slotting.*` |
| `cardano-strict-containers/` | Strict container wrappers | `cardano-strict-containers/` | ✅ Matching namespace |
| `cardano-crypto-praos/` | VRF/KES primitives + C bindings | _no direct crate_ | ⚠️ TODO – currently replaced by `cardano-vrf-pure`; need to mirror structure or add FFI crate |
| `cardano-crypto-tests/` | Integration/property tests | (shared across `cardano-crypto-class/tests`) | ⚠️ Parts folded into crate-level tests; consider dedicated workspace member |
| `measures/` | Memory measures utilities | `measures/` | ✅ Maintained |
| `orphans-deriving-via/` | Additional DerivingVia instances | `orphans-deriving-via/` | ✅ Maintained |
| `heapwords/` | HeapWords class derivations | `heapwords/` | ✅ Maintained |
| `deepseq/` | Deep evaluation helpers | `deepseq/` | ✅ Maintained |
| `scripts/` | Dev utilities (formatting, CI helpers) | `scripts/` | ✅ Ported with Rust specific tooling |
| `docs/` | High-level documentation | `docs/` | ✅ Additional Rust-specific notes live alongside |

### Additional Rust-only crates

| Rust crate | Origin | Notes |
| --- | --- | --- |
| `cardano-vrf-pure/` | New | Pure-Rust VRF port; upstream functionality resides in `cardano-crypto-praos/cbits/vrf03` and `cardano-crypto-class` modules |
| `nothunks/` | (Not present in upstream repo root) | Derived from other IOHK repos; ensure namespace matches future upstream moves |

## Structural differences & follow-up actions

1. **VRF/KES boundary**
   - Upstream: VRF and KES implementations live in `cardano-crypto-praos` with C sources under `cbits/` and Haskell bindings.
   - Rust: implementation currently lives in `cardano-vrf-pure` (work-in-progress pure Rust port).
   - _Action_: maintain folder layout mirroring `cardano-crypto-praos` (e.g., `cardano-vrf-pure/src/cardano_compat`) and consider introducing an FFI crate if we need byte-identical compatibility.

2. **Centralized tests**
   - Upstream collects crypto property tests under `cardano-crypto-tests`.
   - Rust workspace scatters equivalent checks into crate-local `tests/` directories. For maintainers familiar with upstream layout, providing an aggregated `cardano-crypto-tests` crate (even as a wrapper that depends on the existing tests) would reduce friction.

3. **Module namespace parity**
   - Ensure Rust module paths track the Haskell module hierarchy. Example: Haskell `Cardano.Base.Types` → Rust `cardano_base::types` (snake_case).
   - Audit each crate’s `src/` tree to confirm filenames and module names correspond to the Haskell modules they ported.

4. **Documentation parity**
   - Haskell repo stores architecture notes under `docs/`. Rust repo reuses that directory; cross-link to the upstream markdown files to show provenance and explain Rust deviations.

5. **Build orchestrators**
   - Upstream uses `cabal.project`/`flake.nix`. Rust workspace top-level `Cargo.toml` should keep crate membership ordering consistent with upstream’s `cabal.project` entries to make syncing easier.

## Next steps

- [ ] Expand this document with per-crate module maps (Haskell module ↔ Rust module) as the port matures.
- [ ] Evaluate adding a `cardano-crypto-praos` crate (either wrapping C FFI or re-exporting `cardano-vrf-pure`) to maintain naming continuity.
- [ ] Mirror the upstream test harness layout by introducing a dedicated workspace member for cross-crate tests.

Maintaining this alignment will simplify future diffing against the Haskell codebase and help contributors navigate both repositories interchangeably.
