# Phase 08 – Cardano Slotting Parity

**Status:** ☐ Not started  \
**Primary owners:** _Unassigned_  \
**Supporting crates:** `cardano-slotting`

---

## Objective

Port and validate the Cardano slotting system from Haskell to Rust, ensuring correct time-to-slot conversions, epoch calculations, and slot arithmetic that are critical for blockchain synchronization and consensus validation.

## Success Criteria

- Slot/epoch arithmetic matches Haskell byte-for-byte
- Time conversions (slots ↔ POSIX timestamps) produce identical results
- Epoch boundary calculations are correct
- Clock skew handling works as expected
- All edge cases (overflow, boundary conditions) handled identically
- Property-based tests validate invariants
- Documentation includes usage examples and migration guide

## Scope

### Core Functionality

1. **Slot Types**
   - `SlotNo` - Absolute slot number since genesis
   - `EpochNo` - Epoch number
   - `EpochSize` - Number of slots per epoch
   - `WithOrigin` - Optional slot (genesis or actual slot)

2. **Time Conversions**
   - Slot → POSIX time
   - POSIX time → Slot
   - `SystemStart` - Genesis time anchor
   - `SlotLength` - Duration of one slot

3. **Epoch Information**
   - `EpochInfo` - Provides epoch boundaries and slot counts
   - `FixedEpochInfo` - Constant epoch size
   - `ExtendedEpochInfo` - Variable epoch sizes (for protocol transitions)

4. **Arithmetic Operations**
   - Slot addition/subtraction
   - Epoch transitions
   - Relative time calculations
   - Overflow protection

### Out of Scope (Future Phases)

- Network protocol integration
- Wall-clock synchronization daemon
- Ledger state transitions

---

## Milestone Checklist

### 1. Audit & Analysis

- [ ] Compare Rust implementation against Haskell `cardano-slotting`
  - `Cardano.Slotting.Slot` - Core slot types
  - `Cardano.Slotting.Time` - Time conversions
  - `Cardano.Slotting.EpochInfo` - Epoch information providers
  - `Cardano.Slotting.Block` - Block slot metadata

- [ ] Document differences and missing features
- [ ] Identify property invariants from Haskell test suite
- [ ] Map out type conversions and arithmetic operations

### 2. Core Types Implementation

- [ ] `SlotNo` type with arithmetic operations
  - [ ] Addition, subtraction with overflow checks
  - [ ] Comparison operators
  - [ ] Serialization (CBOR via `cardano-binary`)

- [ ] `EpochNo` and `EpochSize` types
  - [ ] Epoch boundary calculations
  - [ ] Slot-to-epoch conversions
  - [ ] Range validation

- [ ] `WithOrigin<T>` type
  - [ ] Functor/monad-like operations
  - [ ] Ordering semantics (origin < any slot)

### 3. Time Conversion Logic

- [ ] `SystemStart` - genesis time anchor
  - [ ] Parse from ISO 8601 / POSIX timestamp
  - [ ] Serialization roundtrip

- [ ] `SlotLength` - slot duration
  - [ ] Millisecond precision
  - [ ] Arithmetic operations
  - [ ] Conversion to/from Duration

- [ ] Slot ↔ Time conversions
  - [ ] `slotToTime` - deterministic slot → timestamp
  - [ ] `timeToSlot` - timestamp → slot (with rounding rules)
  - [ ] Overflow/underflow handling
  - [ ] Clock skew tolerance

### 4. Epoch Information Providers

- [ ] `FixedEpochInfo` - constant epoch size
  - [ ] Epoch → first/last slot
  - [ ] Slot → containing epoch
  - [ ] Tests with common mainnet parameters

- [ ] `ExtendedEpochInfo` - variable epoch sizes
  - [ ] Support for protocol transition boundaries
  - [ ] Epoch size lookups
  - [ ] Slot-to-epoch with variable boundaries

### 5. Arithmetic & Boundary Conditions

- [ ] Slot arithmetic edge cases
  - [ ] Overflow detection (u64::MAX scenarios)
  - [ ] Underflow (cannot go before genesis)
  - [ ] Epoch boundary crossings

- [ ] Time arithmetic
  - [ ] Duration overflow (very large time spans)
  - [ ] Negative time (before genesis)
  - [ ] Rounding behavior for fractional slots

- [ ] Relative time calculations
  - [ ] Time between two slots
  - [ ] Slots between two epochs
  - [ ] Future slot predictions

### 6. Property-Based Testing

- [ ] Slot arithmetic properties
  - [ ] Associativity: (a + b) + c == a + (b + c)
  - [ ] Identity: slot + 0 == slot
  - [ ] Commutativity: a + b == b + a (where valid)

- [ ] Time conversion properties
  - [ ] Roundtrip: timeToSlot(slotToTime(s)) == s (or s+1 with rounding)
  - [ ] Monotonicity: t1 < t2 ⟹ timeToSlot(t1) ≤ timeToSlot(t2)

- [ ] Epoch boundary properties
  - [ ] All slots belong to exactly one epoch
  - [ ] Epoch transitions are continuous
  - [ ] First slot of epoch N+1 follows last slot of epoch N

### 7. Haskell Cross-Validation

- [ ] Extract test vectors from Haskell test suite
  - [ ] Mainnet genesis time and slot length
  - [ ] Known slot/time pairs (block timestamps)
  - [ ] Epoch boundaries for Byron, Shelley, etc.

- [ ] Create JSON test vector files
  - [ ] `slot_time_conversions.json`
  - [ ] `epoch_boundaries.json`
  - [ ] `arithmetic_operations.json`

- [ ] Implement validation harness
  - [ ] Parse Haskell-generated vectors
  - [ ] Compare Rust outputs
  - [ ] Report any discrepancies

### 8. Integration Tests

- [ ] Real-world scenarios
  - [ ] Mainnet genesis → current slot
  - [ ] Byron → Shelley transition
  - [ ] Future slot predictions for staking

- [ ] Error handling
  - [ ] Overflow scenarios
  - [ ] Invalid time ranges
  - [ ] Malformed epoch info

### 9. Performance Benchmarks

- [ ] Time conversion speed (1M conversions)
- [ ] Epoch boundary lookups
- [ ] Arithmetic operation throughput
- [ ] Memory usage for epoch info structures

### 10. Documentation

- [ ] API documentation for all types
  - [ ] Slot, Epoch, Time types
  - [ ] Conversion functions
  - [ ] EpochInfo providers

- [ ] Usage examples
  - [ ] Converting timestamps to slots
  - [ ] Calculating epoch boundaries
  - [ ] Handling clock skew

- [ ] Haskell → Rust migration guide
  - [ ] Type mapping table
  - [ ] Common patterns
  - [ ] Pitfall warnings (overflow, rounding)

- [ ] Update CHANGELOG

---

## Verification Checklist

- [ ] `cargo test -p cardano-slotting` green
- [ ] Property tests pass (quickcheck/proptest)
- [ ] Haskell cross-validation tests green
- [ ] No panics on edge cases (overflow, underflow)
- [ ] Integration tests with real-world parameters pass
- [ ] Benchmarks establish baseline
- [ ] Documentation complete

---

## Dependencies & References

### Haskell Source
- `cardano-base/cardano-slotting/src/Cardano/Slotting/*.hs`
- Test suite: `cardano-base/cardano-slotting/test/*.hs`

### Specifications
- **Ouroboros**: Slot numbering and time synchronization assumptions
- **Byron spec**: Genesis time and initial epoch parameters
- **Shelley spec**: Epoch transitions and slot length

### Rust Implementation
- `cardano-slotting/src/slot.rs` - Slot and epoch types
- `cardano-slotting/src/time.rs` - Time conversions
- `cardano-slotting/src/epoch_info/` - Epoch information providers

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Off-by-one errors in epoch boundaries | Incorrect block validation | Exhaustive boundary tests, Haskell cross-validation |
| Time conversion rounding inconsistencies | Clock skew issues | Property tests, explicit rounding rules |
| Integer overflow in slot arithmetic | Panics or incorrect results | Checked arithmetic, overflow tests |
| Incorrect epoch transition logic | Chain synchronization failures | Real-world test vectors from mainnet |

---

## Estimated Effort

- **Audit & Planning**: 0.5 days
- **Core Types**: 1 day
- **Time Conversions**: 1-2 days
- **Epoch Info**: 1-2 days
- **Property Tests**: 1 day
- **Cross-Validation**: 1 day
- **Documentation**: 1 day
- **Total**: 6-9 days (approximately 1-2 weeks)

---

## Reporting Cadence

Update the **Status** line and tick checkboxes as work progresses.
- (YYYY-MM-DD) INIT: Phase 08 created.
- (YYYY-MM-DD) PROGRESS: [Brief status update]
- (YYYY-MM-DD) COMPLETE: All checklist items finished.

---

## Related Work

- **Completed**: Phase 03 (VRF), Phase 04 (DSIGN), Phase 05 (KES), Phase 06 (Hash), Phase 07 (CBOR)
- **Upcoming**: Phase 09 (Strict Containers), Phase 10 (Helper Crates)

---

_This document lives at `.github/tasks/phase-08-slotting-parity.md`. Update it after every meaningful change._
