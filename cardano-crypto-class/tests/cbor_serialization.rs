#[cfg(feature = "serde")]
mod cbor_tests {
    use cardano_crypto_class::Ed25519;
    use cardano_crypto_class::dsign::DsignAlgorithm;
    use cardano_crypto_class::seed::mk_seed_from_bytes;

    const SEED_BYTES: usize = 32;

    #[test]
    fn test_ed25519_verification_key_cbor_roundtrip() {
        let seed = mk_seed_from_bytes(vec![42u8; SEED_BYTES]);
        let signing_key = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let verification_key = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing_key);

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor_bytes)
            .expect("Failed to serialize verification key");

        // Deserialize from CBOR
        let deserialized: <Ed25519 as DsignAlgorithm>::VerificationKey =
            ciborium::from_reader(cbor_bytes.as_slice())
                .expect("Failed to deserialize verification key");

        // Verify they're equal
        assert_eq!(verification_key, deserialized);
    }

    #[test]
    fn test_ed25519_signature_cbor_roundtrip() {
        let seed = mk_seed_from_bytes(vec![1u8; SEED_BYTES]);
        let signing_key = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let message = b"Hello, Cardano!";
        let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), message, &signing_key);

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&signature, &mut cbor_bytes).expect("Failed to serialize signature");

        // Deserialize from CBOR
        let deserialized: <Ed25519 as DsignAlgorithm>::Signature =
            ciborium::from_reader(cbor_bytes.as_slice()).expect("Failed to deserialize signature");

        // Verify they're equal
        assert_eq!(signature, deserialized);
    }

    #[test]
    fn test_ed25519_cbor_signature_verification() {
        let seed = mk_seed_from_bytes(vec![7u8; SEED_BYTES]);
        let signing_key = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let verification_key = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing_key);
        let message = b"Test message for Cardano";

        // Sign the message
        let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), message, &signing_key);

        // Serialize both VK and signature to CBOR
        let mut vk_cbor = Vec::new();
        ciborium::into_writer(&verification_key, &mut vk_cbor).unwrap();
        let mut sig_cbor = Vec::new();
        ciborium::into_writer(&signature, &mut sig_cbor).unwrap();

        // Deserialize
        let vk_decoded: <Ed25519 as DsignAlgorithm>::VerificationKey =
            ciborium::from_reader(vk_cbor.as_slice()).unwrap();
        let sig_decoded: <Ed25519 as DsignAlgorithm>::Signature =
            ciborium::from_reader(sig_cbor.as_slice()).unwrap();

        // Verify the deserialized signature with deserialized VK
        let result =
            <Ed25519 as DsignAlgorithm>::verify_bytes(&(), &vk_decoded, message, &sig_decoded);
        assert!(result.is_ok(), "Signature verification should succeed");
    }

    #[test]
    fn test_ed25519_cbor_deterministic() {
        // Same key should always produce same CBOR encoding
        let seed = mk_seed_from_bytes(vec![99u8; SEED_BYTES]);
        let signing_key = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let verification_key = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing_key);

        let mut cbor1 = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor1).unwrap();
        let mut cbor2 = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor2).unwrap();

        assert_eq!(cbor1, cbor2, "CBOR encoding should be deterministic");
    }
}

#[cfg(feature = "serde")]
mod vrf_cbor_tests {
    use cardano_crypto_class::vrf::praos::{gen_seed, keypair_from_seed};

    #[test]
    fn test_praos_verification_key_cbor_roundtrip() {
        let seed = gen_seed().expect("Failed to generate seed");
        let (verification_key, _signing_key) =
            keypair_from_seed(&seed).expect("Failed to generate keypair");

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor_bytes)
            .expect("Failed to serialize Praos verification key");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize Praos verification key");

        // Verify they're equal
        assert_eq!(verification_key, deserialized);
    }

    #[test]
    fn test_praos_proof_cbor_roundtrip() {
        let seed = gen_seed().expect("Failed to generate seed");
        let (_verification_key, signing_key) =
            keypair_from_seed(&seed).expect("Failed to generate keypair");

        let message = b"Cardano VRF test message";
        let proof = signing_key.prove(message).expect("Failed to create proof");

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&proof, &mut cbor_bytes).expect("Failed to serialize Praos proof");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize Praos proof");

        // Verify they're equal
        assert_eq!(proof, deserialized);
    }

    #[test]
    fn test_praos_cbor_proof_verification() {
        use cardano_crypto_class::vrf::praos::{PraosProof, PraosVerificationKey};

        let seed = gen_seed().expect("Failed to generate seed");
        let (verification_key, signing_key) =
            keypair_from_seed(&seed).expect("Failed to generate keypair");

        let message = b"Test VRF proof for Cardano";
        let proof = signing_key.prove(message).expect("Failed to create proof");

        // Serialize both VK and proof to CBOR
        let mut vk_cbor = Vec::new();
        ciborium::into_writer(&verification_key, &mut vk_cbor).unwrap();
        let mut proof_cbor = Vec::new();
        ciborium::into_writer(&proof, &mut proof_cbor).unwrap();

        // Deserialize
        let vk_decoded: PraosVerificationKey = ciborium::from_reader(vk_cbor.as_slice()).unwrap();
        let proof_decoded: PraosProof = ciborium::from_reader(proof_cbor.as_slice()).unwrap();

        // Verify the deserialized proof with deserialized VK
        let result = vk_decoded.verify(message, &proof_decoded);
        assert!(result.is_ok(), "VRF proof verification should succeed");
        assert!(result.unwrap().is_some(), "VRF should produce output");
    }

    #[test]
    fn test_praos_cbor_deterministic() {
        let seed = gen_seed().expect("Failed to generate seed");
        let (verification_key, _signing_key) =
            keypair_from_seed(&seed).expect("Failed to generate keypair");

        let mut cbor1 = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor1).unwrap();
        let mut cbor2 = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor2).unwrap();

        assert_eq!(
            cbor1, cbor2,
            "Praos VRF CBOR encoding should be deterministic"
        );
    }

    #[test]
    fn test_simple_vrf_verification_key_cbor_roundtrip() {
        use cardano_crypto_class::seed::{Seed, mk_seed_from_bytes};
        use cardano_crypto_class::vrf::VRFAlgorithm;
        use cardano_crypto_class::vrf::simple::{SimpleVRF, gen_keypair};

        let seed_bytes = vec![42u8; SimpleVRF::SEED_SIZE];
        let seed = Seed::from_bytes(seed_bytes);
        let (_, verification_key) = gen_keypair(&seed);

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor_bytes)
            .expect("Failed to serialize Simple VRF verification key");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize Simple VRF verification key");

        // Verify they're equal
        assert_eq!(verification_key, deserialized);
    }

    #[test]
    fn test_simple_vrf_proof_cbor_roundtrip() {
        use cardano_crypto_class::seed::{Seed, mk_seed_from_bytes};
        use cardano_crypto_class::vrf::VRFAlgorithm;
        use cardano_crypto_class::vrf::simple::{SimpleVRF, gen_keypair};

        let seed_bytes = vec![7u8; SimpleVRF::SEED_SIZE];
        let seed = Seed::from_bytes(seed_bytes);
        let (signing_key, _) = gen_keypair(&seed);

        let message = b"Simple VRF test message";
        let (_, proof) = SimpleVRF::evaluate_bytes(&(), message, &signing_key);

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&proof, &mut cbor_bytes)
            .expect("Failed to serialize Simple VRF proof");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize Simple VRF proof");

        // Verify they're equal
        assert_eq!(proof, deserialized);
    }

    #[test]
    fn test_mock_vrf_verification_key_cbor_roundtrip() {
        use cardano_crypto_class::seed::Seed;
        use cardano_crypto_class::vrf::VRFAlgorithm;
        use cardano_crypto_class::vrf::mock::{MockVRF, gen_keypair};

        let seed_bytes = vec![99u8; MockVRF::SEED_SIZE];
        let seed = Seed::from_bytes(seed_bytes);
        let (_, verification_key) = gen_keypair(&seed);

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor_bytes)
            .expect("Failed to serialize Mock VRF verification key");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize Mock VRF verification key");

        // Verify they're equal
        assert_eq!(verification_key, deserialized);
    }

    #[test]
    fn test_mock_vrf_proof_cbor_roundtrip() {
        use cardano_crypto_class::seed::Seed;
        use cardano_crypto_class::vrf::VRFAlgorithm;
        use cardano_crypto_class::vrf::mock::{MockVRF, gen_keypair};

        let seed_bytes = vec![123u8; MockVRF::SEED_SIZE];
        let seed = Seed::from_bytes(seed_bytes);
        let (signing_key, _) = gen_keypair(&seed);

        let message = b"Mock VRF test";
        let (_, proof) = MockVRF::evaluate_bytes(&(), message, &signing_key);

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&proof, &mut cbor_bytes).expect("Failed to serialize Mock VRF proof");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize Mock VRF proof");

        // Verify they're equal
        assert_eq!(proof, deserialized);
    }
}

#[cfg(feature = "serde")]
mod kes_cbor_tests {
    use cardano_crypto_class::dsign::DsignMAlgorithm;
    use cardano_crypto_class::dsign::ed25519::Ed25519;
    use cardano_crypto_class::kes::{KesAlgorithm, SingleKes};
    use cardano_crypto_class::mlocked_seed::MLockedSeed;

    // Type alias for SingleKes using Ed25519
    type SingleKesEd25519 = SingleKes<Ed25519>;

    #[test]
    fn test_single_kes_verification_key_cbor_roundtrip() {
        // Generate a signing key using Ed25519 directly
        let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
        seed.as_mut_bytes().copy_from_slice(&[42u8; 32]);
        let signing_key = Ed25519::gen_key_m(&seed).unwrap();

        // Derive verification key
        let verification_key = SingleKesEd25519::derive_verification_key(&signing_key)
            .expect("Failed to derive verification key");

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor_bytes)
            .expect("Failed to serialize SingleKes verification key");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize SingleKes verification key");

        // Verify they're equal
        assert_eq!(verification_key, deserialized);

        // Cleanup
        Ed25519::forget_signing_key_m(signing_key);
        seed.finalize();
    }

    #[test]
    fn test_single_kes_signature_cbor_roundtrip() {
        // Generate a signing key
        let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
        seed.as_mut_bytes().copy_from_slice(&[7u8; 32]);
        let signing_key = Ed25519::gen_key_m(&seed).unwrap();

        let message = b"SingleKes CBOR test message";
        let signature = SingleKesEd25519::sign_kes(&(), 0, message, &signing_key)
            .expect("Failed to sign message");

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&signature, &mut cbor_bytes)
            .expect("Failed to serialize SingleKes signature");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize SingleKes signature");

        // Verify they're equal
        assert_eq!(signature, deserialized);

        // Cleanup
        Ed25519::forget_signing_key_m(signing_key);
        seed.finalize();
    }

    #[test]
    fn test_single_kes_cbor_signature_verification() {
        // Generate keypair
        let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
        seed.as_mut_bytes().copy_from_slice(&[99u8; 32]);
        let signing_key = Ed25519::gen_key_m(&seed).unwrap();
        let verification_key = SingleKesEd25519::derive_verification_key(&signing_key)
            .expect("Failed to derive verification key");

        let message = b"Verify SingleKes CBOR";
        let signature = SingleKesEd25519::sign_kes(&(), 0, message, &signing_key)
            .expect("Failed to sign message");

        // Serialize both VK and signature to CBOR
        let mut vk_cbor = Vec::new();
        ciborium::into_writer(&verification_key, &mut vk_cbor).unwrap();
        let mut sig_cbor = Vec::new();
        ciborium::into_writer(&signature, &mut sig_cbor).unwrap();

        // Deserialize
        let vk_decoded = ciborium::from_reader(vk_cbor.as_slice()).unwrap();
        let sig_decoded = ciborium::from_reader(sig_cbor.as_slice()).unwrap();

        // Verify the deserialized signature with deserialized VK
        let result = SingleKesEd25519::verify_kes(&(), &vk_decoded, 0, message, &sig_decoded);
        assert!(
            result.is_ok(),
            "SingleKes signature verification should succeed"
        );

        // Cleanup
        Ed25519::forget_signing_key_m(signing_key);
        seed.finalize();
    }

    // Note: Sum KES tests are not included yet because gen_key_kes_from_seed_bytes
    // is not implemented for SingleKes (which Sum KES builds upon).
    // This requires a more complex implementation that handles the generic
    // DsignMAlgorithm::SeedMaterial type construction from raw bytes.

    #[test]
    fn test_compact_single_kes_verification_key_cbor_roundtrip() {
        use cardano_crypto_class::dsign::DsignMAlgorithm;
        use cardano_crypto_class::dsign::ed25519::Ed25519;
        use cardano_crypto_class::kes::compact_single::CompactSingleKes;
        use cardano_crypto_class::mlocked_seed::MLockedSeed;

        type CompactSingleKesEd25519 = CompactSingleKes<Ed25519>;

        // Generate a signing key
        let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
        seed.as_mut_bytes().copy_from_slice(&[55u8; 32]);
        let signing_key = Ed25519::gen_key_m(&seed).unwrap();

        // Derive verification key
        let verification_key = CompactSingleKesEd25519::derive_verification_key(&signing_key)
            .expect("Failed to derive verification key");

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&verification_key, &mut cbor_bytes)
            .expect("Failed to serialize CompactSingleKes verification key");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize CompactSingleKes verification key");

        // Verify they're equal
        assert_eq!(verification_key, deserialized);

        // Cleanup
        Ed25519::forget_signing_key_m(signing_key);
        seed.finalize();
    }

    #[test]
    fn test_compact_single_kes_signature_cbor_roundtrip() {
        use cardano_crypto_class::dsign::DsignMAlgorithm;
        use cardano_crypto_class::dsign::ed25519::Ed25519;
        use cardano_crypto_class::kes::compact_single::CompactSingleKes;
        use cardano_crypto_class::mlocked_seed::MLockedSeed;

        type CompactSingleKesEd25519 = CompactSingleKes<Ed25519>;

        // Generate a signing key
        let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
        seed.as_mut_bytes().copy_from_slice(&[77u8; 32]);
        let signing_key = Ed25519::gen_key_m(&seed).unwrap();

        let message = b"CompactSingleKes CBOR test message";
        let signature = CompactSingleKesEd25519::sign_kes(&(), 0, message, &signing_key)
            .expect("Failed to sign message");

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ciborium::into_writer(&signature, &mut cbor_bytes)
            .expect("Failed to serialize CompactSingleKes signature");

        // Deserialize from CBOR
        let deserialized = ciborium::from_reader(cbor_bytes.as_slice())
            .expect("Failed to deserialize CompactSingleKes signature");

        // Verify they're equal
        assert_eq!(signature, deserialized);

        // Cleanup
        Ed25519::forget_signing_key_m(signing_key);
        seed.finalize();
    }

    #[test]
    fn test_compact_single_kes_cbor_signature_verification() {
        use cardano_crypto_class::dsign::DsignMAlgorithm;
        use cardano_crypto_class::dsign::ed25519::Ed25519;
        use cardano_crypto_class::kes::compact_single::CompactSingleKes;
        use cardano_crypto_class::mlocked_seed::MLockedSeed;

        type CompactSingleKesEd25519 = CompactSingleKes<Ed25519>;

        // Generate keypair
        let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
        seed.as_mut_bytes().copy_from_slice(&[88u8; 32]);
        let signing_key = Ed25519::gen_key_m(&seed).unwrap();
        let verification_key = CompactSingleKesEd25519::derive_verification_key(&signing_key)
            .expect("Failed to derive verification key");

        let message = b"Verify CompactSingleKes CBOR";
        let signature = CompactSingleKesEd25519::sign_kes(&(), 0, message, &signing_key)
            .expect("Failed to sign message");

        // Serialize both VK and signature to CBOR
        let mut vk_cbor = Vec::new();
        ciborium::into_writer(&verification_key, &mut vk_cbor).unwrap();
        let mut sig_cbor = Vec::new();
        ciborium::into_writer(&signature, &mut sig_cbor).unwrap();

        // Deserialize
        let vk_decoded = ciborium::from_reader(vk_cbor.as_slice()).unwrap();
        let sig_decoded = ciborium::from_reader(sig_cbor.as_slice()).unwrap();

        // Verify the deserialized signature with deserialized VK
        let result =
            CompactSingleKesEd25519::verify_kes(&(), &vk_decoded, 0, message, &sig_decoded);
        assert!(
            result.is_ok(),
            "CompactSingleKes signature verification should succeed"
        );

        // Cleanup
        Ed25519::forget_signing_key_m(signing_key);
        seed.finalize();
    }
}
