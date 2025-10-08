# Phase 11 – Deriving-Via Crates Parity

**Status:** ☐ Not started  \
**Primary owners:** _Unassigned_  \
**Supporting crates:** `base-deriving-via`, `orphans-deriving-via`

---

## Objective

Port and validate the deriving-via pattern crates from Haskell to Rust, providing ergonomic derive macros and newtype wrappers that enable code reuse and reduce boilerplate when implementing common traits.

## Success Criteria

- Derive macros provide equivalent functionality to Haskell deriving-via
- Newtype wrappers enable trait reuse via composition
- Integration with Cardano types (Semigroup, Monoid, NFData, NoThunks, etc.)
- Property-based tests validate derived instances
- Documentation includes usage examples and best practices
- Minimal compilation overhead

## Scope

### Core Functionality

1. **base-deriving-via** - Core deriving patterns
   - Semigroup, Monoid derivations
   - Generic trait implementations
   - InstantiatedAt pattern (tagging types with phantom parameters)

2. **orphans-deriving-via** - Orphan instance patterns
   - Provide implementations for foreign types
   - Enable trait impls for types from other crates
   - Type aliases and newtype wrappers

### Out of Scope (Future Phases)

- Custom derive macros for crypto traits (handled in crypto crates)
- Advanced type-level programming (const generics, GATs)

---

## Milestone Checklist

### 1. Base-Deriving-Via Crate

#### 1.1 Audit & Analysis
- [ ] Compare against Haskell `base-deriving-via`
  - `Cardano.Prelude.Base.Deriving.Via` module
  - Generic deriving strategies
  - InstantiatedAt pattern

- [ ] Document Rust equivalents
  - Derive macros vs Haskell deriving-via
  - Trait coherence (no orphan instances in Rust)
  - Newtype pattern

- [ ] Identify target traits
  - Semigroup (append operation)
  - Monoid (identity + append)
  - Generic (derive Debug, Clone, etc.)
  - NFData / NoThunks (from helper crates)

#### 1.2 Semigroup & Monoid Derivation
- [ ] Define `Semigroup` trait
  ```rust
  trait Semigroup {
      fn append(&self, other: &Self) -> Self;
  }
  ```

- [ ] Define `Monoid` trait
  ```rust
  trait Monoid: Semigroup {
      fn empty() -> Self;
  }
  ```

- [ ] Implement for stdlib types
  - `Vec<T>` - concatenation
  - `String` - concatenation
  - `Option<T>` - First, Last, or inner monoid
  - `()` - unit monoid
  - Numbers (via Sum, Product wrappers)

- [ ] Newtype wrappers
  - `Sum<T>` - addition monoid
  - `Product<T>` - multiplication monoid
  - `First<T>` - first Some wins
  - `Last<T>` - last Some wins

#### 1.3 InstantiatedAt Pattern
- [ ] Define `InstantiatedAt<T, Tag>` wrapper
  ```rust
  struct InstantiatedAt<T, Tag>(T, PhantomData<Tag>);
  ```

- [ ] Use cases
  - Tag types with context (e.g., `Hash<Blake2b256>`)
  - Enable different trait impls for same type
  - Type-safe phantom parameters

- [ ] Trait implementations
  - Forward traits to inner type
  - Preserve tagging in operations

#### 1.4 Generic Derivation
- [ ] Derive macro for `Semigroup`
  ```rust
  #[derive(Semigroup)]
  struct MyData {
      field1: Vec<u8>,
      field2: String,
  }
  // Derives append by appending each field
  ```

- [ ] Derive macro for `Monoid`
  ```rust
  #[derive(Monoid)]
  struct MyData { ... }
  // Derives empty() by using Monoid::empty() for each field
  ```

- [ ] Testing
  - Unit tests for derived instances
  - Property tests (monoid laws)

#### 1.5 Documentation
- [ ] API docs for all traits and wrappers
- [ ] Usage examples
  - Deriving Semigroup/Monoid
  - Using InstantiatedAt
  - Newtype wrappers (Sum, Product, First, Last)

- [ ] Haskell → Rust migration guide
- [ ] Update CHANGELOG

---

### 2. Orphans-Deriving-Via Crate

#### 2.1 Audit & Analysis
- [ ] Compare against Haskell `orphans-deriving-via`
  - Orphan instances for external types
  - Newtype wrappers to enable trait impls
  - Type aliases

- [ ] Document Rust equivalents
  - Rust orphan rules (cannot impl external trait for external type)
  - Newtype pattern as workaround
  - Re-export strategy

- [ ] Identify needed orphan instances
  - Serialization for foreign types
  - NoThunks for stdlib types
  - Custom traits for external crates

#### 2.2 Newtype Wrappers
- [ ] Create wrappers for common external types
  ```rust
  #[derive(Debug, Clone, Serialize)]
  pub struct SerializableU64(pub u64);

  impl From<u64> for SerializableU64 { ... }
  impl From<SerializableU64> for u64 { ... }
  ```

- [ ] Implement needed traits
  - `Serialize` / `Deserialize` for types missing them
  - `NoThunks` for stdlib types
  - `Semigroup` / `Monoid` for external types

#### 2.3 Type Aliases
- [ ] Define common aliases
  ```rust
  pub type Bytes = Vec<u8>;
  pub type Text = String;
  ```

- [ ] Re-export patterns
  - Provide unified interface for external types
  - Enable switching implementations (e.g., `bytes` crate)

#### 2.4 Testing
- [ ] Unit tests for newtype wrappers
- [ ] Conversion tests (From/Into impls)
- [ ] Integration with base-deriving-via

#### 2.5 Documentation
- [ ] API docs for all wrappers
- [ ] Usage examples
  - When to use newtypes
  - Orphan rule workarounds

- [ ] Update CHANGELOG

---

### 3. Integration with Cardano Types

#### 3.1 Crypto Types
- [ ] Implement `Semigroup` for crypto newtypes
  - `SignedDSIGN<V>` - not a semigroup (no append)
  - `VerKeyDSIGN<V>` - not a semigroup (no append)
  - Document why some types don't impl traits

#### 3.2 Container Types
- [ ] Implement `Semigroup`/`Monoid` for `StrictSeq`
  ```rust
  impl<T> Semigroup for StrictSeq<T> {
      fn append(&self, other: &Self) -> Self {
          self.concat(other)
      }
  }
  ```

- [ ] Implement for `StrictMap`
  - Left-biased union for append
  - Empty map for identity

#### 3.3 Slot/Epoch Types
- [ ] Consider `Semigroup` for `SlotNo`
  - Addition semigroup (but not monoid, no identity)
  - Or use `InstantiatedAt` for specific contexts

#### 3.4 Testing
- [ ] Property tests for derived instances
  - Monoid laws: `empty.append(a) == a`, `a.append(empty) == a`, `(a.append(b)).append(c) == a.append(b.append(c))`
  - Semigroup laws: Associativity

- [ ] Integration tests with real Cardano types

---

### 4. Property-Based Testing

- [ ] Monoid law tests
  - Left identity: `empty.append(a) == a`
  - Right identity: `a.append(empty) == a`
  - Associativity: `(a.append(b)).append(c) == a.append(b.append(c))`

- [ ] Semigroup law tests
  - Associativity only

- [ ] InstantiatedAt tests
  - Tagging preserves underlying behavior
  - Type safety (cannot mix different tags)

---

### 5. Documentation

- [ ] Per-crate README updates
  - `base-deriving-via/README.md` - usage, examples, Haskell comparison
  - `orphans-deriving-via/README.md` - orphan rules, newtype patterns

- [ ] Workspace README
  - Link to deriving-via docs
  - Overview of trait derivation strategy

- [ ] Examples directory
  - `examples/semigroup_monoid.rs` - basic usage
  - `examples/instantiated_at.rs` - phantom tagging
  - `examples/orphan_workaround.rs` - newtype pattern

- [ ] Update CHANGELOGs

---

## Verification Checklist

- [ ] `cargo test -p base-deriving-via` green
- [ ] `cargo test -p orphans-deriving-via` green
- [ ] Property tests pass (monoid/semigroup laws)
- [ ] Integration with other crates works
- [ ] Derive macros compile without errors
- [ ] Documentation complete

---

## Dependencies & References

### Haskell Source
- `cardano-base/base-deriving-via/src/Data/Monoid/*.hs`
- `cardano-base/orphans-deriving-via/src/*.hs`
- Test suites in respective test directories

### Specifications
- **Haskell Deriving-Via**: GHC extension for deriving instances
- **Rust Orphan Rules**: Coherence and trait implementation restrictions
- **Newtype Pattern**: Zero-cost abstraction in Rust

### Rust Implementation
- `base-deriving-via/src/semigroup.rs`
- `base-deriving-via/src/instantiated_at.rs`
- `base-deriving-via/src/macros.rs` (derive macros)
- `orphans-deriving-via/src/lib.rs`

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Orphan rules prevent needed impls | Cannot integrate external types | Newtype wrappers, re-export strategy |
| Derive macros too complex | Compilation errors, maintainability | Start with simple hand-written impls, expand later |
| Monoid laws not enforced by type system | Incorrect instances | Property-based tests, documentation |
| InstantiatedAt pattern unclear | Misuse, bugs | Clear docs, examples, type aliases |

---

## Estimated Effort

### Per-Crate Estimates
- **base-deriving-via**:
  - Semigroup/Monoid traits: 1 day
  - InstantiatedAt pattern: 1 day
  - Derive macros: 2-3 days
  - Testing: 1 day
  - Documentation: 1 day
  - **Subtotal**: 6-7 days

- **orphans-deriving-via**:
  - Newtype wrappers: 1 day
  - Trait implementations: 1 day
  - Testing: 0.5 days
  - Documentation: 0.5 days
  - **Subtotal**: 3 days

### Total
- **Implementation**: 9-10 days
- **Integration with Cardano types**: 1-2 days
- **Property testing**: 1 day
- **Documentation**: 1 day
- **Total**: 12-14 days (approximately 2-3 weeks)

---

## Reporting Cadence

Update the **Status** line and tick checkboxes as work progresses.
- (YYYY-MM-DD) INIT: Phase 11 created.
- (YYYY-MM-DD) PROGRESS: Completed [base-deriving-via/orphans-deriving-via].
- (YYYY-MM-DD) COMPLETE: All checklist items finished.

---

## Related Work

- **Completed**: Phase 03 (VRF), Phase 04 (DSIGN), Phase 05 (KES), Phase 06 (Hash), Phase 07 (CBOR)
- **In Progress**: Phase 08 (Slotting), Phase 09 (Strict Containers), Phase 10 (Helper Crates)
- **Upcoming**: Phase 12 (Cardano Base)

---

_This document lives at `.github/tasks/phase-11-deriving-via-parity.md`. Update it after every meaningful change._
