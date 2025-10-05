# Phase 10: Request Haskell CBOR Test Vectors

## Purpose
To finalize production readiness and ensure full compatibility, we need CBOR test vectors from the Haskell cardano-base implementation. These vectors will allow us to validate our Rust serialization and cryptographic outputs against the canonical Haskell reference.

## Request (Draft for GitHub Issue)

---

**Title:** Request: CBOR Test Vectors for Ed25519DSIGN, PraosVRF, KES (Single, Sum, CompactSum), and PraosBatchCompatVRF

**Body:**
Hello IntersectMBO/cardano-base maintainers,

We have completed the Rust port of cardano-base, including all core cryptography, DirectSerialise, and batch-compatible VRF (Draft-13). All tests are passing, and the implementation is production-ready. To ensure full compatibility, we kindly request CBOR test vectors for the following types:

- Ed25519DSIGN (including MLocked variants)
- PraosVRF
- KES: SingleKES, SumKES, CompactSumKES
- PraosBatchCompatVRF (Draft-13 format)

These vectors will be used to validate our serialization and cryptographic outputs against the canonical Haskell implementation. If possible, please provide:
- CBOR-encoded values for representative keys, signatures, and proofs
- Any edge cases or known tricky values
- Reference scripts or documentation for generating these vectors

Thank you for your support!

---

## Next Steps
- Submit this issue to IntersectMBO/cardano-base
- Integrate provided test vectors into Rust test suite
- Finalize production release

---

**Status:** Draft prepared, ready for submission.
