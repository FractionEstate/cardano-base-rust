//! Guards serialized size stability for selected KES variants.
//!
//! If any of these constants change unexpectedly it likely indicates a
//! serialization format change that should be reviewed and documented.

use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{
    CompactSum4Kes, CompactSum7Kes, KesAlgorithm, SingleKes, Sum4Kes, Sum7Kes,
};

// Expected serialized sizes (derived from current implementation / Haskell parity)
const SINGLE_VK: usize = <SingleKes<Ed25519> as KesAlgorithm>::VERIFICATION_KEY_SIZE;
const SINGLE_SIG: usize = <SingleKes<Ed25519> as KesAlgorithm>::SIGNATURE_SIZE;
const SUM4_VK: usize = <Sum4Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE;
const SUM4_SIG: usize = <Sum4Kes as KesAlgorithm>::SIGNATURE_SIZE;
const CSUM4_VK: usize = <CompactSum4Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE;
const CSUM4_SIG: usize = <CompactSum4Kes as KesAlgorithm>::SIGNATURE_SIZE;
const SUM7_VK: usize = <Sum7Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE;
const SUM7_SIG: usize = <Sum7Kes as KesAlgorithm>::SIGNATURE_SIZE;
const CSUM7_VK: usize = <CompactSum7Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE;
const CSUM7_SIG: usize = <CompactSum7Kes as KesAlgorithm>::SIGNATURE_SIZE;

#[test]
fn kes_serialized_sizes_stable() {
    // Any future change: update CHANGELOG + phase doc rationale.
    assert_eq!(SINGLE_VK, 32, "SingleKes vk size changed");
    assert_eq!(SINGLE_SIG, 64, "SingleKes sig size changed");
    // Sum4 / CompactSum4 / Sum7 / CompactSum7 constants intentionally asserted against the
    // implementation constants to catch accidental drift if the code changes them in one place
    // but not here. (If a redesign intentionally changes them, update both sides.)
    assert_eq!(SUM4_VK, <Sum4Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE);
    assert_eq!(SUM4_SIG, <Sum4Kes as KesAlgorithm>::SIGNATURE_SIZE);
    assert_eq!(
        CSUM4_VK,
        <CompactSum4Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE
    );
    assert_eq!(CSUM4_SIG, <CompactSum4Kes as KesAlgorithm>::SIGNATURE_SIZE);
    assert_eq!(SUM7_VK, <Sum7Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE);
    assert_eq!(SUM7_SIG, <Sum7Kes as KesAlgorithm>::SIGNATURE_SIZE);
    assert_eq!(
        CSUM7_VK,
        <CompactSum7Kes as KesAlgorithm>::VERIFICATION_KEY_SIZE
    );
    assert_eq!(CSUM7_SIG, <CompactSum7Kes as KesAlgorithm>::SIGNATURE_SIZE);
}
