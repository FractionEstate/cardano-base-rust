# Phase 10 – Helper Crates Parity

**Status:** ☐ Not started  \
**Primary owners:** _Unassigned_  \
**Supporting crates:** `deepseq`, `heapwords`, `measures`, `nothunks`

---

## Objective

Port and validate the helper/utility crates from Haskell to Rust, ensuring critical debugging, profiling, and validation tools are available for the Cardano ecosystem. These crates provide deep strictness evaluation, memory footprint analysis, performance metrics, and thunk detection.

## Success Criteria

- All helper crates provide equivalent functionality to Haskell versions
- Integration with main crates (crypto, containers, slotting) works correctly
- Performance overhead is minimal (especially for nothunks checks)
- Documentation includes usage examples and best practices
- Debug mode validations can be disabled in release builds

## Scope

### Crates to Implement

1. **deepseq** - Deep strictness evaluation
   - Force evaluation of nested structures
   - Prevent space leaks in long-running processes

2. **heapwords** - Memory footprint calculation
   - Calculate size of data structures in words/bytes
   - Identify memory bloat

3. **measures** - Performance metrics
   - Time measurements
   - Allocation tracking
   - Custom metrics

4. **nothunks** - Thunk detection
   - Runtime validation that no lazy thunks exist
   - Integration with strict containers
   - Debug mode assertions

### Out of Scope (Future Phases)

- Advanced profiling tools (heap profiling, call graphs)
- Real-time monitoring dashboards
- Distributed tracing

---

## Milestone Checklist

### 1. Deepseq Crate

#### 1.1 Audit & Analysis
- [ ] Compare against Haskell `deepseq` package
  - `Control.DeepSeq` - NFData type class
  - `force`, `deepseq` functions
  - Strictness semantics

- [ ] Document Rust equivalents
  - Rust is strict by default (no thunks)
  - Identify use cases (async evaluation, lazy iterators)
  - Determine if crate is needed or placeholder

#### 1.2 Implementation (if needed)
- [ ] Define `DeepSeq` trait
  - `fn deep_seq(&self)` - force evaluation
  - Derive macros for common types

- [ ] Implement for stdlib types
  - Vec, HashMap, BTreeMap
  - Option, Result
  - Tuples, arrays

- [ ] Implement for Cardano types
  - Crypto keys, signatures
  - Containers (StrictSeq, StrictMap)
  - Slot/epoch types

#### 1.3 Testing
- [ ] Unit tests for trait implementations
- [ ] Property tests (deepseq is idempotent)
- [ ] Integration with strict containers

#### 1.4 Documentation
- [ ] API docs
- [ ] Usage examples
- [ ] Haskell comparison (why Rust differs)
- [ ] Update CHANGELOG

---

### 2. Heapwords Crate

#### 2.1 Audit & Analysis
- [ ] Compare against Haskell `heapwords` package
  - `heapWords` function - calculate size
  - GHC heap object representation
  - Memory profiling integration

- [ ] Document Rust equivalents
  - `std::mem::size_of`, `size_of_val`
  - Deep size calculation (with indirection)
  - Allocation tracking

#### 2.2 Implementation
- [ ] Define `HeapSize` trait
  - `fn heap_size(&self) -> usize` - size in bytes
  - Recursive calculation for containers

- [ ] Implement for stdlib types
  - Primitives (usize, u64, etc.) - stack size
  - Vec, HashMap, BTreeMap - heap allocation
  - Box, Arc, Rc - pointer overhead

- [ ] Implement for Cardano types
  - Crypto keys (fixed size, heap allocation)
  - Containers (recursive calculation)
  - Serialized data (Bytes, Vec<u8>)

- [ ] Utilities
  - `heap_words(size)` - convert bytes to words
  - `align_to(size, alignment)` - alignment overhead
  - `total_heap_size<T>()` - for collections

#### 2.3 Testing
- [ ] Unit tests for size calculations
  - Verify against `std::mem::size_of`
  - Test recursive structures
  - Validate alignment

- [ ] Property tests
  - Size of collection ≥ size of elements
  - Empty collections have minimal overhead

#### 2.4 Documentation
- [ ] API docs
- [ ] Usage examples (profiling memory usage)
- [ ] Comparison with Haskell GHC heap representation
- [ ] Update CHANGELOG

---

### 3. Measures Crate

#### 3.1 Audit & Analysis
- [ ] Compare against Haskell `measures` package
  - Time measurements (UTCTime, DiffTime)
  - Custom metrics
  - Aggregation functions

- [ ] Document Rust equivalents
  - `std::time::Instant`, `Duration`
  - `criterion` for benchmarking
  - Custom metric types

#### 3.2 Implementation
- [ ] Time measurements
  - `Stopwatch` - simple timer
  - `TimeIt` - measure function execution
  - `Histogram` - track distribution

- [ ] Memory measurements
  - Integration with `heapwords`
  - Allocation tracking (via allocator hooks)

- [ ] Custom metrics
  - `Metric<T>` - generic metric type
  - Aggregation (sum, avg, min, max)
  - Percentiles (p50, p95, p99)

- [ ] Integration
  - Feature-gated (no overhead in release builds)
  - Thread-local metrics
  - Export to JSON/Prometheus format

#### 3.3 Testing
- [ ] Unit tests for metric types
- [ ] Property tests (aggregation correctness)
- [ ] Integration tests with crypto/container operations

#### 3.4 Documentation
- [ ] API docs
- [ ] Usage examples (profiling hot paths)
- [ ] Best practices (minimal overhead)
- [ ] Update CHANGELOG

---

### 4. Nothunks Crate

#### 4.1 Audit & Analysis
- [ ] Compare against Haskell `nothunks` package
  - `NoThunks` type class
  - `noThunks`, `wNoThunks` functions
  - GHC thunk representation

- [ ] Document Rust equivalents
  - Rust has no thunks (strict by default)
  - Identify lazy patterns (async, iterators, closures)
  - Define validation strategy

#### 4.2 Implementation
- [ ] Define `NoThunks` trait
  - `fn no_thunks(&self) -> Result<(), ThunkInfo>` - validate
  - Derive macros for common types
  - Integration with strict containers

- [ ] Implement for stdlib types
  - Primitives (always strict)
  - Vec, HashMap, BTreeMap (check elements)
  - Option, Result (check inner values)

- [ ] Implement for Cardano types
  - Crypto keys, signatures
  - StrictSeq, StrictMap (recursive check)
  - Slot/epoch types

- [ ] Runtime checks
  - `assert_no_thunks!(value)` - macro for debug builds
  - Feature-gated (no overhead in release)
  - Thread-safe validation

- [ ] ThunkInfo error type
  - Path to thunk location
  - Type information
  - Debug formatting

#### 4.3 Testing
- [ ] Unit tests for trait implementations
- [ ] Property tests
  - `prop_strict_containers_no_thunks`
  - `prop_no_thunks_after_operations`

- [ ] Integration tests
  - Test with strict containers
  - Test with crypto types
  - Negative tests (detect mock thunks)

#### 4.4 Documentation
- [ ] API docs
- [ ] Usage examples
  - Debug assertions
  - Property tests
  - Integration with containers

- [ ] Haskell comparison
  - Why Rust differs (no thunks by default)
  - When to use (validate lazy iterators, async)

- [ ] Update CHANGELOG

---

## Verification Checklist

### Per-Crate Checks
- [ ] `cargo test -p deepseq` green
- [ ] `cargo test -p heapwords` green
- [ ] `cargo test -p measures` green
- [ ] `cargo test -p nothunks` green

### Integration Checks
- [ ] Nothunks integration with `cardano-strict-containers` works
- [ ] Heapwords reports correct sizes for crypto types
- [ ] Measures can profile CBOR serialization
- [ ] No panics on edge cases

### Quality Checks
- [ ] Property tests pass
- [ ] Documentation complete for all crates
- [ ] Minimal performance overhead in release builds
- [ ] Feature flags work correctly

---

## Dependencies & References

### Haskell Source
- `deepseq` package: `Control.DeepSeq`
- `heapwords` package: `GHC.HeapWords`
- `nothunks` package: `NoThunks.Class`
- Cardano-specific `measures` (if exists)

### Specifications
- **GHC Runtime**: Heap object representation
- **Haskell Lazy Evaluation**: Understanding thunks
- **Rust Memory Model**: Stack vs heap allocation

### Rust Implementation
- `deepseq/src/lib.rs`
- `heapwords/src/lib.rs`
- `measures/src/lib.rs`
- `nothunks/src/lib.rs`

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Deepseq not needed in Rust (strict by default) | Wasted effort | Keep as minimal trait, document differences |
| Heapwords calculations inaccurate | Misleading profiling data | Validate against known sizes, test with jemalloc |
| Nothunks overhead in hot paths | Performance regression | Feature-gate all checks, test with benchmarks |
| Measures integration complex | Time sink | Start with simple time/memory metrics, expand later |

---

## Estimated Effort

### Per-Crate Estimates
- **Deepseq**: 1-2 days (minimal, mostly docs)
- **Heapwords**: 2-3 days (trait + implementations)
- **Measures**: 3-4 days (time/memory metrics + integration)
- **Nothunks**: 2-3 days (trait + integration with containers)

### Total
- **Implementation**: 8-12 days
- **Testing**: 2-3 days
- **Documentation**: 2 days
- **Total**: 12-17 days (approximately 2-3 weeks)

---

## Reporting Cadence

Update the **Status** line and tick checkboxes as work progresses.
- (YYYY-MM-DD) INIT: Phase 10 created.
- (YYYY-MM-DD) PROGRESS: Completed [deepseq/heapwords/measures/nothunks].
- (YYYY-MM-DD) COMPLETE: All checklist items finished.

---

## Related Work

- **Completed**: Phase 03 (VRF), Phase 04 (DSIGN), Phase 05 (KES), Phase 06 (Hash), Phase 07 (CBOR)
- **In Progress**: Phase 08 (Slotting), Phase 09 (Strict Containers)
- **Upcoming**: Phase 11 (Deriving-Via Crates), Phase 12 (Cardano Base)

---

_This document lives at `.github/tasks/phase-10-helper-crates-parity.md`. Update it after every meaningful change._
