//! Cardano cryptography utilities.
//!
//! This crate provides pure-Rust replacements for modules from
//! `cardano-crypto-class`. The initial focus is the `Seed` abstraction used to
//! seed deterministic key generation and pseudo-random generators.

#![allow(clippy::unreadable_literal)]
#![cfg_attr(test, allow(clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::panic))]

pub mod direct_serialise;
pub mod dsign;
pub mod ffi;
pub mod hash;
pub mod kes;
pub mod mlocked_bytes;
#[cfg(feature = "mlocked-metrics")]
pub mod mlocked_metrics;
pub mod mlocked_seed;
pub mod packed_bytes;
pub mod pinned_sized_bytes;
pub mod seed;
pub mod util;
pub mod vrf;

pub use seed::{
    Seed, SeedBytesExhausted, SeedRng, expand_seed, get_bytes_from_seed,
    get_bytes_from_seed_either, get_bytes_from_seed_t, get_seed_bytes, get_seed_size,
    mk_seed_from_bytes, read_seed_from_system_entropy, run_with_seed, split_seed,
};

pub use packed_bytes::{
    PackedBytes, PackedBytesError, pack_bytes, pack_bytes_maybe, pack_pinned_bytes, unpack_bytes,
    unpack_pinned_bytes, xor_packed_bytes,
};

pub use util::{
    DecodeHexError, Empty, SignableRepresentation, bytes_to_natural, decode_hex_byte_string,
    decode_hex_string, get_random_word64, natural_to_bytes, read_binary_natural,
    read_binary_word64, slice, splits_at, write_binary_natural, write_binary_word64,
};

pub use direct_serialise::{
    DirectDeserialise, DirectResult, DirectSerialise, SizeCheckError, direct_deserialise_buf,
    direct_deserialise_buf_checked, direct_deserialise_from, direct_deserialise_from_checked,
    direct_serialise_buf, direct_serialise_buf_checked, direct_serialise_to,
    direct_serialise_to_checked,
};

pub use ffi::{SizedMutPtr, SizedPtr};

pub use pinned_sized_bytes::{PinnedSizedBytes, PinnedSizedBytesError};

pub use mlocked_bytes::{
    MLockedAllocator, MLockedBytes, MLockedError, MLockedSizedBytes, copy_mem, mlocked_alloc_bytes,
    mlocked_alloc_bytes_aligned, mlocked_alloc_bytes_zeroed, mlocked_allocator, zero_mem,
};

pub use mlocked_seed::MLockedSeed;

pub use dsign::{
    DsignAlgorithm, DsignError, DsignMAlgorithm, DsignMError, SignedDsign, UnsoundDsignMAlgorithm,
    fail_size_check, seed_size, signed_dsign, signed_dsign_m, size_signature, size_signing_key,
    size_verification_key, verify_signed_dsign,
};

pub use dsign::ed25519::{Ed25519, Ed25519Signature, Ed25519SigningKey, Ed25519VerificationKey};
pub use dsign::ed25519_mlocked::Ed25519MLockedSigningKey;

pub use kes::{
    // Hash algorithms
    Blake2b224,
    Blake2b256,
    Blake2b512,
    // CompactSingle KES
    CompactSingleKes,
    CompactSingleSig,
    // CompactSum KES type aliases (using Blake2b256)
    CompactSum0Kes,
    CompactSum1Kes,
    CompactSum2Kes,
    CompactSum3Kes,
    CompactSum4Kes,
    CompactSum5Kes,
    CompactSum6Kes,
    CompactSum7Kes,
    // Core KES traits and types
    KesAlgorithm,
    KesError,
    KesHashAlgorithm,
    KesMError,
    OptimizedKesSignature,
    Period,
    // Single KES
    SingleKes,
    // Sum KES type aliases (using Blake2b256)
    Sum0Kes,
    Sum1Kes,
    Sum2Kes,
    Sum3Kes,
    Sum4Kes,
    Sum5Kes,
    Sum6Kes,
    Sum7Kes,
};

pub use vrf::{CertifiedVRF, OutputVRF, VRFAlgorithm, VRFError, eval_certified, verify_certified};
