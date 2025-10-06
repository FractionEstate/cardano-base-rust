use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::compact_single::OptimizedKesSignature;
use cardano_crypto_class::kes::{
    CompactSingleKes, CompactSum2Kes, KesAlgorithm, KesError, KesMError, SingleKes,
};

#[test]
fn single_kes_update_expires_after_period() {
    let seed = vec![0u8; SingleKes::<Ed25519>::SEED_SIZE];
    let signing_key =
        SingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed).expect("single KES signing key");

    let result =
        SingleKes::<Ed25519>::update_kes(&(), signing_key, 0).expect("single KES update succeeds");
    assert!(result.is_none(), "SingleKES must expire after period 0");
}

#[test]
fn compact_single_kes_update_expires_after_period() {
    let seed = vec![1u8; CompactSingleKes::<Ed25519>::SEED_SIZE];
    let signing_key = CompactSingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed)
        .expect("compact single KES signing key");

    let result = CompactSingleKes::<Ed25519>::update_kes(&(), signing_key, 0)
        .expect("compact single KES update succeeds");
    assert!(
        result.is_none(),
        "CompactSingleKES must expire after period 0"
    );
}

#[test]
fn compact_sum2_kes_update_expires_at_total_periods() {
    let seed = vec![2u8; CompactSum2Kes::SEED_SIZE];
    let mut signing_key =
        CompactSum2Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");
    let verification_key = CompactSum2Kes::derive_verification_key(&signing_key)
        .expect("compact sum verification key");
    let total_periods = CompactSum2Kes::total_periods();

    for period in 0..total_periods {
        let message = period.to_be_bytes();
        let signature = CompactSum2Kes::sign_kes(&(), period, &message, &signing_key)
            .expect("compact sum signing");
        CompactSum2Kes::verify_kes(&(), &verification_key, period, &message, &signature)
            .expect("compact sum verification");

        let next =
            CompactSum2Kes::update_kes(&(), signing_key, period).expect("compact sum key update");
        if period + 1 == total_periods {
            assert!(next.is_none(), "key must expire after the final period");
            break;
        } else {
            signing_key = next.expect("key should remain valid before final period");
        }
    }
}

#[test]
fn compact_sum2_kes_rejects_verification_key_mismatch() {
    let seed = vec![3u8; CompactSum2Kes::SEED_SIZE];
    let signing_key =
        CompactSum2Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");
    let verification_key = CompactSum2Kes::derive_verification_key(&signing_key)
        .expect("compact sum verification key");
    let message = b"compact-sum-tamper";
    let signature =
        CompactSum2Kes::sign_kes(&(), 1, message, &signing_key).expect("compact sum signing");

    let mut mismatched_vk = CompactSum2Kes::raw_serialize_verification_key_kes(&verification_key);
    if let Some(first) = mismatched_vk.first_mut() {
        *first ^= 0x01;
    } else {
        panic!("verification key unexpectedly empty");
    }

    assert_eq!(
        CompactSum2Kes::verify_kes(&(), &mismatched_vk, 1, message, &signature),
        Err(KesError::VerificationFailed),
        "verification should fail when H(vk0 || vk1) does not match"
    );

    CompactSum2Kes::forget_signing_key_kes(signing_key);
}

#[test]
fn compact_sum2_kes_rejects_tampered_signature() {
    let seed = vec![4u8; CompactSum2Kes::SEED_SIZE];
    let signing_key =
        CompactSum2Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");
    let verification_key = CompactSum2Kes::derive_verification_key(&signing_key)
        .expect("compact sum verification key");
    let period = 2;
    let message = b"compact-sum-signature";
    let signature =
        CompactSum2Kes::sign_kes(&(), period, message, &signing_key).expect("compact sum signing");

    let mut raw = CompactSum2Kes::raw_serialize_signature_kes(&signature);
    let last = raw
        .last_mut()
        .expect("compact sum signature must have at least one byte");
    *last ^= 0x01;

    let tampered = CompactSum2Kes::raw_deserialize_signature_kes(&raw)
        .expect("tampered signature length remains valid");

    assert_eq!(
        CompactSum2Kes::verify_kes(&(), &verification_key, period, message, &tampered),
        Err(KesError::VerificationFailed),
        "verification should fail for tampered signatures"
    );

    CompactSum2Kes::forget_signing_key_kes(signing_key);
}

#[test]
fn compact_sum2_kes_sign_out_of_range_period_fails() {
    let seed = vec![5u8; CompactSum2Kes::SEED_SIZE];
    let signing_key =
        CompactSum2Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");
    let period = CompactSum2Kes::total_periods();
    let message = b"out-of-range";

    let err = CompactSum2Kes::sign_kes(&(), period, message, &signing_key)
        .err()
        .expect("signing at out-of-range period should fail");
    match err {
        KesMError::Kes(KesError::PeriodOutOfRange { .. }) => {},
        other => panic!("unexpected error for out-of-range signing: {other:?}"),
    }

    CompactSum2Kes::forget_signing_key_kes(signing_key);
}

#[test]
fn compact_single_kes_embedded_vk_matches_derived() {
    let seed = vec![6u8; CompactSingleKes::<Ed25519>::SEED_SIZE];
    let signing_key = CompactSingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed)
        .expect("compact single signing key");
    let derived = CompactSingleKes::<Ed25519>::derive_verification_key(&signing_key)
        .expect("compact single verification key");
    let message = b"compact-single";

    let signature =
        CompactSingleKes::<Ed25519>::sign_kes(&(), 0, message, &signing_key).expect("signing");
    let embedded = CompactSingleKes::<Ed25519>::raw_serialize_verification_key_kes(
        signature.extract_verification_key(),
    );
    let derived_bytes = CompactSingleKes::<Ed25519>::raw_serialize_verification_key_kes(&derived);

    assert_eq!(embedded, derived_bytes, "embedded vk must match derived vk");

    CompactSingleKes::<Ed25519>::forget_signing_key_kes(signing_key);
}
