# GitHub Issues Template for Gap Closure

This file contains templates for creating GitHub issues to track the gap closure implementation.

## Instructions

Copy each issue template below and create it manually in GitHub, or enable the GitHub integration tool.

---

## Issue #1: Master Tracking Issue

**Title:** `[GAP CLOSURE] Complete Cardano-Base Compatibility (225 Tasks, 28 Days)`

**Labels:** `enhancement`, `documentation`, `tracking`, `meta`

**Body:**

```markdown
# Gap Closure Master Tracking Issue

## Overview

This meta-issue tracks the complete gap closure effort to achieve full compatibility with Haskell [cardano-base](https://github.com/IntersectMBO/cardano-base).

**Total Effort:** 225 tasks, ~28 days
**Status:** 🔴 Planning Complete, Implementation Pending
**Documentation:** See `docs/audit/` folder

## 📊 Progress Summary

- [ ] Phase 1: CBOR Serialization (CRITICAL) - 98 tasks, 8.7 days
- [ ] Phase 2: Comprehensive Testing (HIGH) - 52 tasks, 10.6 days
- [ ] Phase 3: Performance Optimization (MEDIUM) - 27 tasks, 5.2 days
- [ ] Phase 4: Polish & Documentation - 20 tasks, 4.6 days
- [ ] Phase 5: Final Verification - 12 tasks, 1.7 days

**Overall:** 0/225 tasks complete (0%)

## 📚 Key Documents

- **[COMPREHENSIVE_TODO.md](docs/audit/COMPREHENSIVE_TODO.md)** - Complete task breakdown (225 tasks)
- **[GAP_CLOSURE_PLAN.md](docs/audit/GAP_CLOSURE_PLAN.md)** - Implementation guide with Haskell→Rust examples
- **[GAP_CLOSURE_STATUS.md](docs/audit/GAP_CLOSURE_STATUS.md)** - Strategy options and progress tracking
- **[GAPS_ANALYSIS.md](docs/audit/GAPS_ANALYSIS.md)** - Detailed gap analysis
- **[DOCUMENTATION_MAP.md](docs/audit/DOCUMENTATION_MAP.md)** - Navigation guide

## 🎯 8 Gaps Identified

| Priority | Gap | Effort |
|----------|-----|--------|
| 🔴 Critical | CBOR Serialization for KES/VRF/DSIGN | 8.7 days |
| ⚠️ High | Comprehensive Test Suite | 10.6 days |
| 📊 Medium | DirectSerialise Optimization | 5.2 days |
| 🔵 Low | Polish & Documentation | 6.3 days |

## 🚀 Implementation Strategy

**Recommended: Phased Delivery (5 weeks)**

- **Week 1:** Phase 1 - CBOR (enables Cardano node integration)
- **Weeks 2-3:** Phase 2 - Testing (production confidence)
- **Week 4:** Phase 3 - Performance (DirectSerialise)
- **Week 5:** Phases 4-5 - Polish & Release

## 📋 Phase Issues

Track detailed progress in phase-specific issues:

1. #TBD - Phase 1: CBOR Serialization (CRITICAL)
2. #TBD - Phase 2: Comprehensive Testing (HIGH)
3. #TBD - Phase 3: Performance Optimization (MEDIUM)
4. #TBD - Phase 4: Polish & Documentation
5. #TBD - Phase 5: Final Verification

## ✅ Success Criteria

### Phase 1 Complete
- [ ] All crypto types have Serialize/Deserialize implementations
- [ ] Basic roundtrip tests passing
- [ ] CBOR format matches Haskell (spot checks)
- [ ] Can integrate with Cardano node

### Overall Complete
- [ ] All 225 tasks completed
- [ ] All gaps with Haskell closed
- [ ] Production ready
- [ ] Ready for crates.io publication

## 🔗 References

- **Haskell Reference:** https://github.com/IntersectMBO/cardano-base
- **Local Documentation:** `docs/audit/` folder (4,690 lines, 8 documents)

## 🎯 Next Actions

1. Review implementation strategy in GAP_CLOSURE_STATUS.md
2. Start with Phase 1 Issue (CBOR Serialization)
3. Follow GAP_CLOSURE_PLAN.md for implementation patterns
4. Track progress using COMPREHENSIVE_TODO.md

---

**Created:** October 4, 2025
**Target Completion:** ~5 weeks (phased delivery) or 9 days (critical path only)
```

---

## Issue #2: Phase 1 - CBOR Serialization (CRITICAL)

**Title:** `[Phase 1] CBOR Serialization for KES/VRF/DSIGN (98 tasks, 8.7 days)`

**Labels:** `enhancement`, `critical`, `phase-1`, `cbor`, `serialization`

**Milestone:** Phase 1 (if you create milestones)

**Body:**

```markdown
# Phase 1: CBOR Serialization (CRITICAL)

## Overview

Implement CBOR serialization/deserialization for all cryptographic types to enable Cardano node integration.

**Effort:** 98 tasks, 8.7 days
**Priority:** 🔴 CRITICAL - Blocks Cardano node integration
**Status:** 🔴 Not Started

## 📋 Task Breakdown

### 1.1 KES - SingleKes (10 tasks, 0.7 days)
- [ ] 1.1.1: Add Serialize for SingleKesVerificationKey
- [ ] 1.1.2: Add Deserialize for SingleKesVerificationKey
- [ ] 1.1.3: Add Serialize for SingleKesSignature
- [ ] 1.1.4: Add Deserialize for SingleKesSignature
- [ ] 1.1.5: Define UnsoundPureSingleKesSigningKey
- [ ] 1.1.6: Add Serialize for UnsoundPure signing key
- [ ] 1.1.7: Add Deserialize for UnsoundPure signing key
- [ ] 1.1.8: Test: VK CBOR roundtrip
- [ ] 1.1.9: Test: Signature CBOR roundtrip
- [ ] 1.1.10: Test: UnsoundPure SK CBOR roundtrip

### 1.2 KES - SumKes (14 tasks, 1.4 days)
- [ ] 1.2.1-1.2.7: Serialize/Deserialize implementations
- [ ] 1.2.8-1.2.14: Tests for Sum1Kes through Sum7Kes

### 1.3 KES - CompactSingleKes (8 tasks, 0.6 days)
- [ ] 1.3.1-1.3.7: Serialize/Deserialize implementations
- [ ] 1.3.8: CBOR roundtrip tests

### 1.4 KES - CompactSumKes (8 tasks, 1.2 days)
- [ ] 1.4.1-1.4.7: Serialize/Deserialize implementations
- [ ] 1.4.8: CBOR tests for CompactSum1-7

### 1.5 VRF Types (17 tasks, 1.5 days)
- [ ] 1.5.1-1.5.6: Praos VRF Serialize/Deserialize
- [ ] 1.5.7-1.5.12: PraosBatchCompat Serialize/Deserialize
- [ ] 1.5.13-1.5.14: Tests for all Praos types
- [ ] 1.5.15-1.5.17: SimpleVRF, MockVRF + tests

### 1.6 DSIGN Types (12 tasks, 0.9 days)
- [ ] 1.6.1-1.6.10: Ed25519 and Ed25519MLocked Serialize/Deserialize
- [ ] 1.6.11-1.6.12: CBOR roundtrip tests

## 📚 Implementation Guide

See [GAP_CLOSURE_PLAN.md](docs/audit/GAP_CLOSURE_PLAN.md) Phase 1 for:
- Haskell code examples
- Rust implementation patterns
- CBOR encoding format
- Test requirements

**Key Pattern:**
```rust
impl Serialize for SingleKesVerificationKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let bytes = self.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}
```

## ✅ Acceptance Criteria

- [ ] All KES types have Serialize/Deserialize (SingleKes, SumKes, CompactSingleKes, CompactSumKes)
- [ ] All VRF types have Serialize/Deserialize (Praos, PraosBatchCompat, SimpleVRF, MockVRF)
- [ ] All DSIGN types have Serialize/Deserialize (Ed25519, Ed25519MLocked)
- [ ] Basic roundtrip tests pass for all types
- [ ] CBOR format follows Haskell pattern: `encodeBytes . rawSerialise`
- [ ] CI passes with all new tests

## 🔗 References

- **Haskell:** <https://github.com/IntersectMBO/cardano-base/blob/master/cardano-crypto-class/src/Cardano/Crypto/KES/Class.hs>
- **Task Details:** [COMPREHENSIVE_TODO.md](docs/audit/COMPREHENSIVE_TODO.md) Phase 1

## 🎯 Next Steps

1. Create feature branch: `feature/cbor-serialization`
2. Start with task 1.1.1 (SingleKes VK Serialize)
3. Implement → Test → PR → Review
4. Proceed through all 98 tasks systematically

---

**Dependencies:** None (can start immediately)
**Blocks:** Phase 2 testing, Cardano node integration
**Related:** Master tracking issue #TBD

```

---

## Issue #3: Phase 2 - Comprehensive Testing (HIGH)

**Title:** `[Phase 2] Comprehensive Test Suite (52 tasks, 10.6 days)`

**Labels:** `enhancement`, `high-priority`, `phase-2`, `testing`

**Body:**

```markdown
# Phase 2: Comprehensive Testing (HIGH)

## Overview

Implement comprehensive test suite including basic tests, cross-compatibility validation with Haskell, property-based tests, and UnsoundPure trait.

**Effort:** 52 tasks, 10.6 days
**Priority:** ⚠️ HIGH - Required for production confidence
**Status:** 🔴 Not Started

## 📋 Task Breakdown

### 2.1 Basic KES Tests - Positive (10 tasks, 1.1 days)
- [ ] 2.1.1: Setup test file structure
- [ ] 2.1.2-2.1.4: Sign/verify tests at period 0 (SingleKes, Sum1Kes, Sum7Kes)
- [ ] 2.1.5: Key evolution through multiple periods
- [ ] 2.1.6: VK constancy after updates
- [ ] 2.1.7: Sign at different periods
- [ ] 2.1.8-2.1.9: CompactKes tests
- [ ] 2.1.10: All KES variants at max period

### 2.2 Basic KES Tests - Negative (7 tasks, 0.9 days)
- [ ] 2.2.1: Verify with wrong verification key
- [ ] 2.2.2: Verify with wrong message
- [ ] 2.2.3: Verify with wrong period
- [ ] 2.2.4: Sign at invalid period
- [ ] 2.2.5: Update beyond max period
- [ ] 2.2.6: Corrupted signature verification
- [ ] 2.2.7: Corrupted verification key

### 2.3 Serialization Tests (5 tasks, 1 day)
- [ ] 2.3.1: Raw serialization roundtrip
- [ ] 2.3.2: Size matches constants
- [ ] 2.3.3: CBOR matches raw + wrapping
- [ ] 2.3.4: Signature serialization roundtrip
- [ ] 2.3.5: UnsoundPure SK serialization

### 2.4 Cross-Compatibility with Haskell (9 tasks, 2.8 days)
- [ ] 2.4.1: Create Haskell test vector generator
- [ ] 2.4.2-2.4.5: Generate test vectors for all KES types
- [ ] 2.4.6: Create Rust test loader
- [ ] 2.4.7: Verify all Haskell-generated signatures
- [ ] 2.4.8: Generate Rust signatures, verify in Haskell
- [ ] 2.4.9: CBOR encoding matches byte-for-byte

### 2.5 UnsoundPureKESAlgorithm Trait (10 tasks, 3.1 days)
- [ ] 2.5.1: Define trait with associated types
- [ ] 2.5.2: Add comprehensive documentation
- [ ] 2.5.3: Implement for SingleKes
- [ ] 2.5.4: Implement for SumKes
- [ ] 2.5.5: Implement for CompactSingleKes
- [ ] 2.5.6: Implement for CompactSumKes
- [ ] 2.5.7-2.5.8: Add to/from conversions
- [ ] 2.5.9-2.5.10: Unit tests and conversion tests

### 2.6 Property-Based Tests (11 tasks, 1.7 days)
- [ ] 2.6.1: Add proptest dependency
- [ ] 2.6.2: Setup property test framework
- [ ] 2.6.3: Property: sign/verify roundtrip
- [ ] 2.6.4: Property: VK constant after updates
- [ ] 2.6.5: Property: wrong key fails
- [ ] 2.6.6: Property: wrong message fails
- [ ] 2.6.7: Property: wrong period fails
- [ ] 2.6.8: Property: serialization deterministic
- [ ] 2.6.9: Property: serialization roundtrip identity
- [ ] 2.6.10: Property: period range validation
- [ ] 2.6.11: Property: update monotonic

## 📚 Implementation Guide

See [GAP_CLOSURE_PLAN.md](docs/audit/GAP_CLOSURE_PLAN.md) Phase 2 for detailed implementation patterns.

## ✅ Acceptance Criteria

- [ ] Test coverage > 80% for KES/VRF/DSIGN
- [ ] All Haskell test vectors pass
- [ ] Property tests run without finding issues (1000+ iterations)
- [ ] UnsoundPure trait fully implemented and documented
- [ ] Cross-compatibility validated with Haskell
- [ ] CI passes all tests

## 🔗 References

- **Haskell Tests:** https://github.com/IntersectMBO/cardano-base/tree/master/cardano-crypto-class/test
- **Task Details:** [COMPREHENSIVE_TODO.md](docs/audit/COMPREHENSIVE_TODO.md) Phase 2

---

**Dependencies:** Phase 1 (CBOR) must be complete
**Blocks:** Production deployment
**Related:** Master tracking issue #TBD
```

---

## Issue #4: Phase 3 - Performance Optimization (MEDIUM)

**Title:** `[Phase 3] DirectSerialise & Performance Optimization (27 tasks, 5.2 days)`

**Labels:** `enhancement`, `performance`, `phase-3`, `optimization`

**Body:**

```markdown
# Phase 3: Performance Optimization (MEDIUM)

## Overview

Implement zero-copy DirectSerialise trait for all crypto types and create comprehensive benchmark suite.

**Effort:** 27 tasks, 5.2 days
**Priority:** 📊 MEDIUM - Performance optimization
**Status:** 🔴 Not Started

## 📋 Task Breakdown

### 3.1 DirectSerialise for KES (11 tasks, 2.5 days)
- [ ] 3.1.1-3.1.2: SingleKes VK DirectSerialise/Deserialise
- [ ] 3.1.3-3.1.4: Sum1-7 VK DirectSerialise/Deserialise
- [ ] 3.1.5-3.1.6: CompactSingleKes VK DirectSerialise/Deserialise
- [ ] 3.1.7-3.1.8: CompactSum1-7 VK DirectSerialise/Deserialise
- [ ] 3.1.9-3.1.10: All KES signatures DirectSerialise/Deserialise
- [ ] 3.1.11: DirectSerialise roundtrip tests

### 3.2 DirectSerialise for VRF (10 tasks, 1.6 days)
- [ ] 3.2.1-3.2.6: Praos types DirectSerialise/Deserialise
- [ ] 3.2.7: PraosBatchCompat DirectSerialise
- [ ] 3.2.8: SimpleVRF DirectSerialise
- [ ] 3.2.9: MockVRF DirectSerialise
- [ ] 3.2.10: DirectSerialise tests for all VRF

### 3.3 Performance Benchmarking (6 tasks, 1.1 days)
- [ ] 3.3.1: Create benchmark suite
- [ ] 3.3.2: Benchmark: Raw vs DirectSerialise
- [ ] 3.3.3: Benchmark: CBOR performance
- [ ] 3.3.4: Benchmark: KES sign/verify
- [ ] 3.3.5: Benchmark: VRF operations
- [ ] 3.3.6: Create performance comparison report

## 📚 Implementation Guide

See [GAP_CLOSURE_PLAN.md](docs/audit/GAP_CLOSURE_PLAN.md) Phase 3 for zero-copy patterns.

**Key Pattern:**
```rust
impl DirectSerialise for SingleKesVerificationKey {
    fn direct_to_bytes(&self) -> &[u8] {
        &self.inner_bytes  // Zero-copy reference
    }
}
```

## ✅ Acceptance Criteria

- [ ] DirectSerialise implemented for all KES types
- [ ] DirectSerialise implemented for all VRF types
- [ ] Benchmarks show performance improvements over regular serialization
- [ ] Zero-copy working correctly (no allocations)
- [ ] Performance report documents Rust vs Haskell comparison

## 🔗 References

- **Haskell:** <https://github.com/IntersectMBO/cardano-base/blob/master/cardano-binary/src/Cardano/Binary/Serialize.hs>
- **Task Details:** [COMPREHENSIVE_TODO.md](docs/audit/COMPREHENSIVE_TODO.md) Phase 3

---

**Dependencies:** Phase 1 (CBOR)
**Blocks:** None (optimization only)
**Related:** Master tracking issue #TBD

```

---

## Issue #5: Phase 4 - Polish & Documentation

**Title:** `[Phase 4] Polish, Documentation & Code Quality (20 tasks, 4.6 days)`

**Labels:** `enhancement`, `documentation`, `code-quality`, `phase-4`

**Body:**

```markdown
# Phase 4: Polish & Documentation

## Overview

Add comprehensive documentation, improve code quality, and expand test coverage for VRF and DSIGN.

**Effort:** 20 tasks, 4.6 days
**Priority:** 🔵 LOW - Quality improvements
**Status:** 🔴 Not Started

## 📋 Task Breakdown

### 4.1 Additional Test Coverage (5 tasks, 1.3 days)
- [ ] 4.1.1-4.1.2: VRF basic positive/negative tests
- [ ] 4.1.3-4.1.4: DSIGN basic positive/negative tests
- [ ] 4.1.5: Edge case tests (empty messages, max sizes, boundaries)

### 4.2 Documentation Improvements (6 tasks, 2 days)
- [ ] 4.2.1: Add module-level docs for kes/mod.rs
- [ ] 4.2.2: Add module-level docs for vrf/mod.rs
- [ ] 4.2.3: Add module-level docs for dsign/mod.rs
- [ ] 4.2.4: Document all public APIs with examples
- [ ] 4.2.5: Create usage guide (CRYPTO_GUIDE.md)
- [ ] 4.2.6: Create Haskell→Rust migration guide

### 4.3 Code Quality and Cleanup (6 tasks, 1.3 days)
- [ ] 4.3.1: Run clippy, fix all warnings
- [ ] 4.3.2: Run rustfmt on all crypto code
- [ ] 4.3.3: Check for code duplication, refactor
- [ ] 4.3.4: Review and improve error messages
- [ ] 4.3.5: Add #[must_use] attributes where appropriate
- [ ] 4.3.6: Audit unsafe code usage

## ✅ Acceptance Criteria

- [ ] All modules have comprehensive documentation
- [ ] All public APIs documented with examples
- [ ] Usage guide created for developers
- [ ] No clippy warnings
- [ ] Code formatted consistently
- [ ] All unsafe code audited and documented

## 🔗 References

- **Task Details:** [COMPREHENSIVE_TODO.md](docs/audit/COMPREHENSIVE_TODO.md) Phase 4

---

**Dependencies:** Phases 1-3 complete
**Blocks:** None
**Related:** Master tracking issue #TBD
```

---

## Issue #6: Phase 5 - Final Verification

**Title:** `[Phase 5] Final Verification & Release (12 tasks, 1.7 days)`

**Labels:** `release`, `verification`, `phase-5`

**Body:**

```markdown
# Phase 5: Final Verification & Release

## Overview

Final testing, documentation updates, and release preparation.

**Effort:** 12 tasks, 1.7 days
**Priority:** 🎯 FINAL - Release preparation
**Status:** 🔴 Not Started

## 📋 Task Breakdown

### 5.1 Final Testing (4 tasks, 0.7 days)
- [ ] 5.1.1: Run full test suite (`cargo test --workspace`)
- [ ] 5.1.2: Run property tests with high iteration count
- [ ] 5.1.3: Run benchmarks (`cargo bench`)
- [ ] 5.1.4: Integration test with Haskell cardano-node (if available)

### 5.2 Documentation Updates (4 tasks, 0.6 days)
- [ ] 5.2.1: Update GAPS_ANALYSIS.md (mark gaps closed)
- [ ] 5.2.2: Update README.md with new features
- [ ] 5.2.3: Update CHANGELOG.md
- [ ] 5.2.4: Create COMPLETION_REPORT.md

### 5.3 Final Cleanup (4 tasks, 0.4 days)
- [ ] 5.3.1: Review all commit messages
- [ ] 5.3.2: Squash/organize commits if needed
- [ ] 5.3.3: Update version numbers (bump to 0.2.0)
- [ ] 5.3.4: Create GitHub release notes

## ✅ Acceptance Criteria

- [ ] All 225 tasks completed
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Version bumped
- [ ] Release notes created
- [ ] Ready for crates.io publication

## 🎉 Success Metrics

After completion:
- ✅ All gaps with Haskell cardano-base closed
- ✅ CBOR serialization working for all types
- ✅ Test coverage > 80%
- ✅ Cross-compatibility validated
- ✅ Performance optimized
- ✅ Production ready

## 🔗 References

- **Task Details:** [COMPREHENSIVE_TODO.md](docs/audit/COMPREHENSIVE_TODO.md) Phase 5

---

**Dependencies:** Phases 1-4 complete
**Blocks:** crates.io publication
**Related:** Master tracking issue #TBD
```

---

## Quick Create Script

You can use this bash script to create all issues at once using GitHub CLI:

```bash
#!/bin/bash
# create_gap_issues.sh

# Requires: gh CLI (https://cli.github.com/)

# Create master tracking issue
gh issue create \
  --title "[GAP CLOSURE] Complete Cardano-Base Compatibility (225 Tasks, 28 Days)" \
  --label "enhancement,documentation,tracking,meta" \
  --body-file .github/issue_templates/master_tracking.md

# Create phase issues
for phase in 1 2 3 4 5; do
  gh issue create \
    --label "enhancement,phase-$phase" \
    --body-file ".github/issue_templates/phase_$phase.md"
done

echo "✅ All issues created!"
```

---

## Summary

**Total Issues to Create:** 6

- 1 Master tracking issue
- 5 Phase-specific issues

**Next Steps:**

1. Copy templates above and create issues manually, OR
2. Enable GitHub integration tool and I can create them automatically, OR
3. Use the `gh` CLI script above

Each issue links back to the comprehensive documentation in `docs/audit/` for detailed implementation guidance.
