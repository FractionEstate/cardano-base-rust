# VRF Compatibility Analysis: Pure Rust vs Libsodium

## Problem Statement

The current pure Rust VRF implementation produces different cryptographic outputs than the Haskell libsodium-based implementation used in official Cardano. ALL 14 test vectors from IntersectMBO/cardano-base fail.

## Root Cause Analysis

### Current Rust Implementation
- Uses `curve25519-dalek` pure Rust library
- Implements standard IETF draft-03 VRF specification
- Uses standard Elligator2 hash-to-curve

### Haskell/Libsodium Implementation
- Uses custom Cardano fork of libsodium C library
- Located in: `cardano-crypto-praos/cbits/vrf03/`
- Contains Cardano-specific customizations

### Key Differences

#### 1. Custom Elligator2 Implementation
```c
// Cardano-specific function in ed25519_ref10.c
void cardano_ge25519_from_uniform(unsigned char s[32], const unsigned char r[32])
{
    // Custom implementation with:
    // - Specific field element operations
    // - Custom cofactor clearing
    // - Cardano-specific Montgomery curve parameters
}
```

#### 2. Custom Field Element Operations
All field operations use `cardano_fe25519_*` prefix with Cardano-specific implementations:
- `cardano_fe25519_sq2()` - Square and double
- `cardano_fe25519_invert()` - Inversion
- `cardano_fe25519_neg()` - Negation
- `cardano_fe25519_cmov()` - Conditional move
- And ~20 more operations

#### 3. Cofactor Clearing
```c
cardano_ge25519_clear_cofactor(&p3);  // Multiplies by cofactor h=8
```

This operation is specific to Cardano's implementation and differs from standard curve25519 operations.

## Solution Options

### Option A: FFI Bindings to Libsodium (RECOMMENDED)

**Approach:**
- Create Rust FFI bindings to the existing C code
- Use the exact same C implementation as Haskell
- Guaranteed 100% byte-exact compatibility

**Advantages:**
- ✅ 100% guaranteed compatibility (same code = same results)
- ✅ Faster to implement (3-5 days)
- ✅ Lower risk (proven code)
- ✅ Easier to maintain (follows Haskell exactly)
- ✅ No cryptographic expertise required

**Disadvantages:**
- ❌ Requires C compiler at build time
- ❌ Not "pure Rust" (but neither is Haskell - it uses C too)
- ❌ Platform-specific builds

**Implementation Steps:**
1. Copy C source files from cardano-crypto-praos
2. Create Rust build.rs to compile C code
3. Write Rust FFI wrappers
4. Test against all 14 official vectors
5. Estimated time: 3-5 days

**Files Needed:**
```
cardano-crypto-praos/cbits/vrf03/
├── prove.c
├── verify.c
├── vrf.c
├── crypto_vrf_ietfdraft03.h
└── private/
    ├── ed25519_ref10.c
    ├── ed25519_ref10.h
    └── fe_*.h (field element headers)
```

### Option B: Pure Rust Implementation

**Approach:**
- Reimplement all Cardano-specific operations in pure Rust
- Match libsodium behavior exactly
- Extensive testing required

**Advantages:**
- ✅ Pure Rust (no C dependencies)
- ✅ Cross-platform without C compiler
- ✅ Potentially better integration with Rust ecosystem

**Disadvantages:**
- ❌ Very time-consuming (2-4 weeks minimum)
- ❌ High risk of subtle bugs
- ❌ Requires deep cryptographic expertise
- ❌ Difficult to verify correctness
- ❌ Must re-implement ~2000 lines of C code
- ❌ Needs extensive validation

**What Must Be Implemented:**

1. **Custom Elligator2 (200+ lines)**
   - `ge25519_elligator2()` - Montgomery curve operations
   - `ge25519_mont_to_ed()` - Montgomery to Edwards conversion
   - `ge25519_xmont_to_ymont()` - X to Y coordinate recovery
   
2. **Field Element Operations (500+ lines)**
   - ~25 different field operations
   - Must match libsodium's exact behavior
   - Includes: sq2, invert, neg, cmov, reduce, etc.

3. **Point Operations (300+ lines)**
   - `cardano_ge25519_scalarmult()` - Scalar multiplication
   - `cardano_ge25519_scalarmult_base()` - Base point multiplication
   - `cardano_ge25519_clear_cofactor()` - Cofactor clearing
   - `cardano_ge25519_p3_tobytes()` - Point serialization

4. **Scalar Operations (200+ lines)**
   - `cardano_sc25519_reduce()` - Scalar reduction
   - `cardano_sc25519_muladd()` - Scalar multiply-add

**Estimated Effort:**
- Research and understanding: 2-3 days
- Implementation: 7-10 days
- Testing and debugging: 5-7 days
- **Total: 2-4 weeks**

## Test Evidence

### Current Failure
```
Input: sk=0x00..00, pk=0x3b6a..., alpha=0x00

Expected (Haskell libsodium):
  proof:  000f006e64c91f84...
  output: 9930b5dddc0938f0...

Actual (Rust pure):
  proof:  66ab39fcb475eae4...
  output: fdf1b5111afdcf52...

Status: COMPLETELY DIFFERENT (fails from first byte)
```

All 14 test vectors show similar complete mismatches, confirming fundamental algorithmic differences.

## Recommendation

**Use Option A (FFI Bindings)** because:

1. **Haskell itself uses C** - It's not about pure vs FFI, it's about correctness
2. **Faster delivery** - 3-5 days vs 2-4 weeks
3. **Lower risk** - Uses proven code
4. **Guaranteed compatibility** - Same code = same results
5. **Easier maintenance** - Can track upstream Cardano changes

The goal is 100% Cardano compatibility, not 100% pure Rust at the cost of correctness and time.

## Implementation Priority

**Immediate (if Option A chosen):**
1. Copy C source files
2. Set up build system
3. Create FFI bindings
4. Validate with test vectors

**Immediate (if Option B chosen):**
1. Study libsodium implementations in detail
2. Implement field element operations
3. Implement point operations
4. Extensive testing at each step

## References

- Haskell implementation: `cardano-crypto-praos/cbits/vrf03/`
- Test vectors: `cardano-crypto-tests/test_vectors/vrf_ver03_*`
- IETF Draft-03: draft-irtf-cfrg-vrf-03
- Libsodium: https://github.com/input-output-hk/libsodium (Cardano fork)
