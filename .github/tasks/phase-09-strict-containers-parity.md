# Phase 09 – Cardano Strict Containers Parity

**Status:** ☐ Not started  \
**Primary owners:** _Unassigned_  \
**Supporting crates:** `cardano-strict-containers`

---

## Objective

Port and validate the strict container types (StrictSeq, StrictMap, StrictMaybe) from Haskell to Rust, ensuring strict evaluation semantics that prevent memory leaks and space leaks in long-running blockchain applications.

## Success Criteria

- Strict evaluation semantics match Haskell behavior
- Memory usage patterns identical to Haskell (no thunks)
- Performance characteristics meet or exceed Haskell
- Integration with `nothunks` validation
- CBOR serialization roundtrips correctly
- Property-based tests validate container laws
- Documentation includes usage patterns and performance characteristics

## Scope

### Core Functionality

1. **StrictSeq** - Strict sequence
   - Forces evaluation of elements on insertion
   - Efficient append, index, slice operations
   - Finger-tree or similar balanced structure

2. **StrictMap** - Strict key-value map
   - Strict evaluation of both keys and values
   - Ordered by key (for canonical CBOR)
   - Efficient lookup, insert, delete

3. **StrictMaybe** - Strict optional value
   - Forces evaluation on construction
   - Alternative to lazy Option for hot paths

4. **Nothunks Integration**
   - Validate no lazy thunks remain
   - Runtime checks in debug builds
   - Property tests for strictness

### Out of Scope (Future Phases)

- Concurrent containers (Arc/Mutex variants)
- Persistent data structures (structural sharing)
- Custom allocators

---

## Milestone Checklist

### 1. Audit & Analysis

- [ ] Compare Rust implementation against Haskell `cardano-strict-containers`
  - `Data.Sequence.Strict` - Strict sequences
  - `Data.Map.Strict` - Strict maps
  - `Data.Maybe.Strict` - Strict Maybe

- [ ] Document differences and missing features
- [ ] Identify strictness invariants
- [ ] Map out nothunks integration points

### 2. StrictSeq Implementation

- [ ] Core data structure
  - [ ] Choose backing structure (Vec, VecDeque, or finger-tree)
  - [ ] Ensure strict evaluation on push/insert
  - [ ] Implement Index trait

- [ ] Operations
  - [ ] `push_back`, `push_front` - O(1) or O(log n)
  - [ ] `pop_back`, `pop_front` - O(1) or O(log n)
  - [ ] `get`, `get_mut` - O(1) or O(log n)
  - [ ] `split_at`, `append` - efficient splitting/joining
  - [ ] `iter`, `iter_mut` - standard iterators

- [ ] Properties
  - [ ] Length tracking
  - [ ] Empty sequence checks
  - [ ] Conversion from/to Vec

### 3. StrictMap Implementation

- [ ] Core data structure
  - [ ] Choose backing structure (BTreeMap for ordering)
  - [ ] Ensure strict evaluation on insert
  - [ ] Maintain key ordering for CBOR

- [ ] Operations
  - [ ] `insert` - O(log n), strict evaluation
  - [ ] `get`, `get_mut` - O(log n) lookup
  - [ ] `remove` - O(log n)
  - [ ] `keys`, `values`, `iter` - ordered iterators
  - [ ] `union`, `intersection`, `difference` - set operations

- [ ] Properties
  - [ ] Key uniqueness
  - [ ] Ordered iteration
  - [ ] Conversion from/to HashMap/BTreeMap

### 4. StrictMaybe Implementation

- [ ] Core data structure
  - [ ] Wrapper around Option with strict semantics
  - [ ] Force evaluation on construction
  - [ ] Implement Debug, Clone, etc.

- [ ] Operations
  - [ ] `SomeStrict(value)` - forces evaluation
  - [ ] `NothingStrict` - explicit nothing
  - [ ] `map`, `and_then` - strict transformations
  - [ ] `unwrap_or`, `unwrap_or_else` - strict defaults

- [ ] Properties
  - [ ] No lazy thunks
  - [ ] Interop with Option

### 5. Nothunks Integration

- [ ] Implement `NoThunks` trait for all containers
  - [ ] Validate strict evaluation recursively
  - [ ] Debug mode assertions
  - [ ] Test harness for thunk detection

- [ ] Property tests
  - [ ] `prop_no_thunks_after_insert`
  - [ ] `prop_no_thunks_after_operations`
  - [ ] `prop_strict_evaluation`

### 6. CBOR Serialization

- [ ] Implement `Serialize` for StrictSeq
  - [ ] Array encoding
  - [ ] Definite length

- [ ] Implement `Serialize` for StrictMap
  - [ ] Map encoding
  - [ ] Canonical key ordering
  - [ ] Definite length

- [ ] Implement `Serialize` for StrictMaybe
  - [ ] Optional encoding (null or value)

- [ ] Roundtrip tests
  - [ ] Serialize → deserialize → compare
  - [ ] Test with nested containers

### 7. Property-Based Testing

- [ ] StrictSeq properties
  - [ ] `prop_length_after_push`
  - [ ] `prop_order_preserved`
  - [ ] `prop_split_append_identity`
  - [ ] `prop_iter_order`

- [ ] StrictMap properties
  - [ ] `prop_insert_lookup`
  - [ ] `prop_remove_absent`
  - [ ] `prop_union_size`
  - [ ] `prop_ordered_keys`

- [ ] StrictMaybe properties
  - [ ] `prop_map_preserves_some`
  - [ ] `prop_and_then_associativity`

### 8. Haskell Cross-Validation

- [ ] Extract test vectors from Haskell test suite
  - [ ] Known sequences and their operations
  - [ ] Maps with ordered keys
  - [ ] Edge cases (empty, singleton)

- [ ] Create JSON test vector files
  - [ ] `strict_seq_operations.json`
  - [ ] `strict_map_operations.json`

- [ ] Implement validation harness
  - [ ] Parse Haskell-generated vectors
  - [ ] Compare Rust outputs
  - [ ] Report any discrepancies

### 9. Performance Benchmarks

- [ ] StrictSeq benchmarks
  - [ ] Push/pop operations (front and back)
  - [ ] Indexing
  - [ ] Splitting/appending
  - [ ] Iteration

- [ ] StrictMap benchmarks
  - [ ] Insert/lookup/remove
  - [ ] Iteration
  - [ ] Set operations (union, intersection)

- [ ] Memory usage
  - [ ] Size of empty containers
  - [ ] Memory overhead per element
  - [ ] Allocation patterns

### 10. Documentation

- [ ] API documentation for all types
  - [ ] StrictSeq, StrictMap, StrictMaybe
  - [ ] All public methods
  - [ ] Usage examples

- [ ] Performance characteristics
  - [ ] Big-O notation for operations
  - [ ] Memory usage
  - [ ] Comparison with Haskell

- [ ] Haskell → Rust migration guide
  - [ ] Type mapping table
  - [ ] Common patterns
  - [ ] Strictness semantics

- [ ] Update CHANGELOG

---

## Verification Checklist

- [ ] `cargo test -p cardano-strict-containers` green
- [ ] Property tests pass (quickcheck/proptest)
- [ ] Haskell cross-validation tests green
- [ ] Nothunks validation passes
- [ ] CBOR roundtrip tests green
- [ ] No panics on edge cases
- [ ] Benchmarks establish baseline
- [ ] Documentation complete

---

## Dependencies & References

### Haskell Source
- `cardano-base/cardano-strict-containers/src/Data/*.hs`
- Test suite: `cardano-base/cardano-strict-containers/test/*.hs`

### Specifications
- **Finger Trees**: Efficient functional sequences (Hinze/Paterson paper)
- **Persistent Data Structures**: Okasaki's thesis
- **NoThunks**: Space leak detection in Haskell

### Rust Implementation
- `cardano-strict-containers/src/strict_seq.rs`
- `cardano-strict-containers/src/strict_map.rs`
- `cardano-strict-containers/src/strict_maybe.rs`
- `nothunks` crate integration

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Strictness semantics differ from Haskell | Memory leaks, performance issues | Property tests, nothunks validation |
| CBOR serialization incompatible | Chain validation failures | Cross-validation with Haskell |
| Performance regression vs Haskell | Node slowdown | Benchmarks, profiling |
| Incorrect ordering in StrictMap | Canonical CBOR failures | Explicit ordering tests |

---

## Estimated Effort

- **Audit & Planning**: 0.5 days
- **StrictSeq**: 2 days
- **StrictMap**: 2 days
- **StrictMaybe**: 0.5 days
- **Nothunks Integration**: 1 day
- **CBOR Serialization**: 1 day
- **Property Tests**: 1-2 days
- **Cross-Validation**: 1 day
- **Benchmarks**: 1 day
- **Documentation**: 1 day
- **Total**: 10-12 days (approximately 2 weeks)

---

## Reporting Cadence

Update the **Status** line and tick checkboxes as work progresses.
- (YYYY-MM-DD) INIT: Phase 09 created.
- (YYYY-MM-DD) PROGRESS: [Brief status update]
- (YYYY-MM-DD) COMPLETE: All checklist items finished.

---

## Related Work

- **Completed**: Phase 03 (VRF), Phase 04 (DSIGN), Phase 05 (KES), Phase 06 (Hash), Phase 07 (CBOR)
- **In Progress**: Phase 08 (Slotting)
- **Upcoming**: Phase 10 (Helper Crates)

---

_This document lives at `.github/tasks/phase-09-strict-containers-parity.md`. Update it after every meaningful change._
