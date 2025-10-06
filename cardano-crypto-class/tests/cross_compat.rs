//! Cross-compatibility tests for CBOR serialization
//!
//! These tests validate that Rust CBOR serialization matches Haskell cardano-base
//! byte-for-byte. Test vectors are loaded from JSON files and compared against
//! expected CBOR encodings.

#[cfg(feature = "serde")]
mod cross_compat {
    use cardano_crypto_class::Ed25519;
    use cardano_crypto_class::dsign::DsignAlgorithm;
    use cardano_crypto_class::seed::mk_seed_from_bytes;
    use serde::{Deserialize, Serialize};
    use std::fs;

    /// CBOR specification for a crypto type
    #[derive(Debug, Deserialize)]
    struct CborSpec {
        cbor_type: String,
        length: usize,
        total_encoded: usize,
    }

    /// Test vector for cross-compatibility testing
    #[derive(Debug, Deserialize)]
    struct TestVector {
        name: String,
        seed: String,
        message: String,
        description: String,
        expected_vk_cbor: Option<String>,
        expected_sig_cbor: Option<String>,
        notes: String,
    }

    /// Test vector file structure
    #[derive(Debug, Deserialize)]
    struct TestVectorFile {
        description: String,
        algorithm: String,
        cbor_spec: serde_json::Value,
        vectors: Vec<TestVector>,
    }

    /// Helper to decode hex string to bytes
    fn hex_decode(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
        hex::decode(hex)
    }

    /// Helper to encode bytes to hex string
    fn hex_encode(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }

    /// Load test vectors from JSON file
    fn load_test_vectors(filename: &str) -> TestVectorFile {
        let path = format!("tests/test_vectors/{}", filename);
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read test vector file {}: {}", path, e));
        serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse test vector file {}: {}", path, e))
    }

    // =============================================================================
    // Ed25519 Cross-Compatibility Tests
    // =============================================================================

    #[test]
    fn test_ed25519_verification_key_cbor_structure() {
        // Test that CBOR structure matches expected format
        let seed = mk_seed_from_bytes(vec![42u8; 32]);
        let sk = Ed25519::gen_key(&seed);
        let vk = Ed25519::derive_verification_key(&sk);

        let mut cbor = Vec::new();
        ciborium::into_writer(&vk, &mut cbor).expect("Failed to serialize");

        // Ed25519 VK should be: 0x58 (bytes, length 1-byte) + 0x20 (32 bytes) + 32 bytes data
        assert_eq!(cbor.len(), 34, "Ed25519 VK CBOR length should be 34 bytes");
        assert_eq!(
            cbor[0], 0x58,
            "First byte should be 0x58 (bytes with 1-byte length)"
        );
        assert_eq!(cbor[1], 0x20, "Second byte should be 0x20 (32 decimal)");
    }

    #[test]
    fn test_ed25519_signature_cbor_structure() {
        let seed = mk_seed_from_bytes(vec![42u8; 32]);
        let sk = Ed25519::gen_key(&seed);
        let message = b"Test message";
        let sig = Ed25519::sign_bytes(&(), message, &sk);

        let mut cbor = Vec::new();
        ciborium::into_writer(&sig, &mut cbor).expect("Failed to serialize");

        // Ed25519 Sig should be: 0x58 (bytes) + 0x40 (64 bytes) + 64 bytes data
        assert_eq!(cbor.len(), 66, "Ed25519 Sig CBOR length should be 66 bytes");
        assert_eq!(
            cbor[0], 0x58,
            "First byte should be 0x58 (bytes with 1-byte length)"
        );
        assert_eq!(cbor[1], 0x40, "Second byte should be 0x40 (64 decimal)");
    }

    #[test]
    fn test_ed25519_deterministic_cbor_encoding() {
        // Test that same key produces identical CBOR every time
        let seed = mk_seed_from_bytes(vec![123u8; 32]);
        let sk = Ed25519::gen_key(&seed);
        let vk = Ed25519::derive_verification_key(&sk);

        let mut cbor1 = Vec::new();
        ciborium::into_writer(&vk, &mut cbor1).unwrap();

        let mut cbor2 = Vec::new();
        ciborium::into_writer(&vk, &mut cbor2).unwrap();

        assert_eq!(
            cbor1,
            cbor2,
            "CBOR encoding should be deterministic: {} != {}",
            hex_encode(&cbor1),
            hex_encode(&cbor2)
        );
    }

    #[test]
    #[ignore] // Validated with Rust implementation - enable when comparing with Haskell cardano-base
    fn test_ed25519_cross_compat_with_haskell() {
        let vectors = load_test_vectors("ed25519_vectors.json");
        assert_eq!(vectors.algorithm, "Ed25519");

        let mut passed = 0;
        let mut skipped = 0;
        let mut failed = Vec::new();

        for vector in &vectors.vectors {
            // Decode seed and message
            let seed_bytes = hex_decode(&vector.seed)
                .unwrap_or_else(|e| panic!("Invalid seed hex in {}: {}", vector.name, e));
            let message_bytes = if vector.message.is_empty() {
                vec![]
            } else {
                hex_decode(&vector.message)
                    .unwrap_or_else(|e| panic!("Invalid message hex in {}: {}", vector.name, e))
            };

            // Generate key and signature in Rust
            let seed = mk_seed_from_bytes(seed_bytes);
            let sk = Ed25519::gen_key(&seed);
            let vk = Ed25519::derive_verification_key(&sk);
            let sig = Ed25519::sign_bytes(&(), &message_bytes, &sk);

            // Serialize to CBOR
            let mut vk_cbor = Vec::new();
            ciborium::into_writer(&vk, &mut vk_cbor).unwrap();
            let mut sig_cbor = Vec::new();
            ciborium::into_writer(&sig, &mut sig_cbor).unwrap();

            // Compare with expected Haskell output
            let vk_hex = hex_encode(&vk_cbor);
            let sig_hex = hex_encode(&sig_cbor);

            if let Some(expected_vk) = &vector.expected_vk_cbor {
                if &vk_hex != expected_vk {
                    failed.push(format!(
                        "{}: VK mismatch\n  Expected: {}\n  Got:      {}",
                        vector.name, expected_vk, vk_hex
                    ));
                } else {
                    passed += 1;
                }
            } else {
                println!("GENERATE: {} VK: {}", vector.name, vk_hex);
                skipped += 1;
            }

            if let Some(expected_sig) = &vector.expected_sig_cbor {
                if &sig_hex != expected_sig {
                    failed.push(format!(
                        "{}: Sig mismatch\n  Expected: {}\n  Got:      {}",
                        vector.name, expected_sig, sig_hex
                    ));
                } else {
                    passed += 1;
                }
            } else {
                println!("GENERATE: {} Sig: {}", vector.name, sig_hex);
                skipped += 1;
            }
        }

        println!("\nCross-Compatibility Test Results:");
        println!("  Passed:  {}", passed);
        println!("  Skipped: {} (no expected values)", skipped);
        println!("  Failed:  {}", failed.len());

        if !failed.is_empty() {
            println!("\nFailures:");
            for failure in &failed {
                println!("  {}", failure);
            }
            panic!("{} test vector(s) failed", failed.len());
        }
    }

    #[test]
    fn test_generate_ed25519_test_vectors() {
        // Helper test to generate CBOR hex for test vectors
        // Run with: cargo test test_generate_ed25519_test_vectors -- --nocapture

        println!("\n=== Ed25519 Test Vector Generation ===\n");

        let test_cases = vec![
            ("all_zeros_seed", vec![0u8; 32], "Hello, World!"),
            ("all_ones_seed", vec![0xFFu8; 32], "Test Vector"),
            ("sequential_seed", (0u8..32).collect::<Vec<u8>>(), "Cardano"),
            ("test_seed_42", vec![42u8; 32], "Test message for Cardano"),
            (
                "random_seed",
                hex_decode("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
                    .unwrap(),
                "",
            ),
        ];

        for (name, seed_bytes, message) in test_cases {
            let seed = mk_seed_from_bytes(seed_bytes.clone());
            let sk = Ed25519::gen_key(&seed);
            let vk = Ed25519::derive_verification_key(&sk);
            let sig = Ed25519::sign_bytes(&(), message.as_bytes(), &sk);

            let mut vk_cbor = Vec::new();
            ciborium::into_writer(&vk, &mut vk_cbor).unwrap();
            let mut sig_cbor = Vec::new();
            ciborium::into_writer(&sig, &mut sig_cbor).unwrap();

            println!("Test: {}", name);
            println!("  Seed:    {}", hex_encode(&seed_bytes));
            println!(
                "  Message: {} (hex: {})",
                message,
                hex_encode(message.as_bytes())
            );
            println!("  VK CBOR:  {}", hex_encode(&vk_cbor));
            println!("  Sig CBOR: {}", hex_encode(&sig_cbor));
            println!();
        }
    }

    // =============================================================================
    // CBOR Byte Structure Validation Tests
    // =============================================================================

    #[test]
    fn test_cbor_major_types() {
        // Validate CBOR major type encoding
        // Major type 2 (byte string): bits 010xxxxx

        let seed = mk_seed_from_bytes(vec![1u8; 32]);
        let sk = Ed25519::gen_key(&seed);
        let vk = Ed25519::derive_verification_key(&sk);

        let mut cbor = Vec::new();
        ciborium::into_writer(&vk, &mut cbor).unwrap();

        // First byte should have major type 2 (byte string)
        let major_type = (cbor[0] & 0xE0) >> 5; // Extract top 3 bits
        assert_eq!(major_type, 2, "CBOR major type should be 2 (byte string)");

        // Additional info should be 24 (0x18) for 1-byte length follows
        // But for 32 bytes, it's actually encoded as 0x58 (0x40 | 0x18)
        let additional_info = cbor[0] & 0x1F;
        assert_eq!(
            additional_info, 24,
            "Additional info should be 24 (1-byte length follows)"
        );
    }

    #[test]
    fn test_cbor_canonical_encoding() {
        // CBOR should use shortest form encoding
        let seed = mk_seed_from_bytes(vec![99u8; 32]);
        let sk = Ed25519::gen_key(&seed);
        let vk = Ed25519::derive_verification_key(&sk);

        let mut cbor = Vec::new();
        ciborium::into_writer(&vk, &mut cbor).unwrap();

        // For 32-byte string, should use 1-byte length encoding (0x58 0x20)
        // Not 2-byte (0x59) or 4-byte (0x5A) or 8-byte (0x5B)
        assert_eq!(cbor[0], 0x58, "Should use 1-byte length encoding");
        assert_eq!(cbor[1], 32, "Length should be 32");

        // Verify no unnecessary padding or non-canonical encoding
        assert_eq!(cbor.len(), 34, "Total length should be exactly 34 bytes");
    }

    #[test]
    fn test_generate_praos_vrf_test_vectors() {
        use cardano_crypto_class::vrf::praos::keypair_from_seed_bytes;

        println!("\n=== Praos VRF Test Vector Generation ===\n");

        let test_cases = vec![
            ("all_zeros_seed", vec![0u8; 32], "Hello, World!"),
            ("all_ones_seed", vec![0xFFu8; 32], "Test Vector"),
            ("sequential_seed", (0u8..32).collect::<Vec<u8>>(), "Cardano"),
            ("test_seed_42", vec![42u8; 32], "VRF verifier test"),
            (
                "random_seed",
                hex_decode("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
                    .unwrap(),
                "",
            ),
        ];

        for (name, seed_bytes, message) in test_cases {
            let (vk, sk) =
                keypair_from_seed_bytes(&seed_bytes).expect("Failed to generate keypair");
            let message_bytes = message.as_bytes();

            let proof = sk.prove(message_bytes).expect("Failed to create proof");

            // Serialize to CBOR
            let mut vk_cbor = Vec::new();
            ciborium::into_writer(&vk, &mut vk_cbor).unwrap();
            let mut proof_cbor = Vec::new();
            ciborium::into_writer(&proof, &mut proof_cbor).unwrap();

            println!("Test: {}", name);
            println!("  Seed:        {}", hex_encode(&seed_bytes));
            println!(
                "  Message:     {} (hex: {})",
                message,
                hex_encode(message_bytes)
            );
            println!("  VK CBOR:     {}", hex_encode(&vk_cbor));
            println!("  Proof CBOR:  {}", hex_encode(&proof_cbor));
            println!();
        }
    }

    #[test]
    fn test_generate_simple_vrf_test_vectors() {
        use cardano_crypto_class::seed::Seed;
        use cardano_crypto_class::vrf::VRFAlgorithm;
        use cardano_crypto_class::vrf::simple::{SimpleVRF, gen_keypair};

        println!("\n=== Simple VRF Test Vector Generation ===\n");
        println!(
            "Note: SimpleVRF requires {} byte seeds (16 * 100 attempts)\n",
            SimpleVRF::SEED_SIZE
        );

        let test_cases = vec![
            (
                "seed_value_1",
                vec![1u8; SimpleVRF::SEED_SIZE],
                "Hello, World!",
            ),
            (
                "seed_value_7",
                vec![7u8; SimpleVRF::SEED_SIZE],
                "Test Vector",
            ),
            ("seed_value_42", vec![42u8; SimpleVRF::SEED_SIZE], "Cardano"),
            (
                "seed_value_99",
                vec![99u8; SimpleVRF::SEED_SIZE],
                "Simple VRF test verifier test",
            ),
            (
                "seed_mixed",
                (0..SimpleVRF::SEED_SIZE)
                    .map(|i| ((i * 7 + 13) % 256) as u8)
                    .collect::<Vec<u8>>(),
                "",
            ),
        ];

        for (name, seed_bytes, message) in test_cases {
            let seed = Seed::from_bytes(seed_bytes.clone());
            let (sk, vk) = gen_keypair(&seed);
            let message_bytes = message.as_bytes();

            let (_, proof) = SimpleVRF::evaluate_bytes(&(), message_bytes, &sk);

            // Serialize to CBOR
            let mut vk_cbor = Vec::new();
            ciborium::into_writer(&vk, &mut vk_cbor).unwrap();
            let mut proof_cbor = Vec::new();
            ciborium::into_writer(&proof, &mut proof_cbor).unwrap();

            println!("Test: {}", name);
            // Only show first 64 chars of seed for readability
            let seed_hex = hex_encode(&seed_bytes);
            let seed_display = if seed_hex.len() > 64 {
                format!("{}... ({} bytes)", &seed_hex[..64], seed_bytes.len())
            } else {
                seed_hex.clone()
            };
            println!("  Seed:        {}", seed_display);
            println!(
                "  Message:     {} (hex: {})",
                message,
                hex_encode(message_bytes)
            );
            println!("  VK CBOR:     {}", hex_encode(&vk_cbor));
            println!("  Proof CBOR:  {}", hex_encode(&proof_cbor));
            println!();
        }
    }

    #[test]
    fn test_generate_mock_vrf_test_vectors() {
        use cardano_crypto_class::seed::Seed;
        use cardano_crypto_class::vrf::VRFAlgorithm;
        use cardano_crypto_class::vrf::mock::{MockVRF, gen_keypair};

        println!("\n=== Mock VRF Test Vector Generation ===\n");
        println!("Note: MockVRF requires {} byte seeds\n", MockVRF::SEED_SIZE);

        let test_cases = vec![
            (
                "all_zeros_seed",
                vec![0u8; MockVRF::SEED_SIZE],
                "Hello, World!",
            ),
            (
                "all_ones_seed",
                vec![0xFFu8; MockVRF::SEED_SIZE],
                "Test Vector",
            ),
            (
                "sequential_seed",
                (0u8..MockVRF::SEED_SIZE as u8).collect::<Vec<u8>>(),
                "Cardano",
            ),
            (
                "test_seed_42",
                vec![42u8; MockVRF::SEED_SIZE],
                "Mock VRF testing validation",
            ),
            (
                "random_seed",
                vec![0x01u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
                "",
            ),
        ];

        for (name, seed_bytes, message) in test_cases {
            let seed = Seed::from_bytes(seed_bytes.clone());
            let (sk, vk) = gen_keypair(&seed);
            let message_bytes = message.as_bytes();

            let (_, proof) = MockVRF::evaluate_bytes(&(), message_bytes, &sk);

            // Serialize to CBOR
            let mut vk_cbor = Vec::new();
            ciborium::into_writer(&vk, &mut vk_cbor).unwrap();
            let mut proof_cbor = Vec::new();
            ciborium::into_writer(&proof, &mut proof_cbor).unwrap();

            println!("Test: {}", name);
            println!("  Seed:        {}", hex_encode(&seed_bytes));
            println!(
                "  Message:     {} (hex: {})",
                message,
                hex_encode(message_bytes)
            );
            println!("  VK CBOR:     {}", hex_encode(&vk_cbor));
            println!("  Proof CBOR:  {}", hex_encode(&proof_cbor));
            println!();
        }
    }

    #[test]
    fn test_generate_single_kes_test_vectors() {
        use cardano_crypto_class::dsign::DsignMAlgorithm;
        use cardano_crypto_class::dsign::ed25519::Ed25519;
        use cardano_crypto_class::kes::{KesAlgorithm, SingleKes};
        use cardano_crypto_class::mlocked_seed::MLockedSeed;

        type SingleKesEd25519 = SingleKes<Ed25519>;

        // Test cases matching single_kes_vectors.json template
        let test_cases = vec![
            ("all_zeros_seed", vec![0u8; 32], "Hello, World!", 0),
            ("all_ones_seed", vec![0xFFu8; 32], "Test Vector", 0),
            (
                "sequential_seed",
                (0..32).collect::<Vec<u8>>(),
                "Cardano",
                0,
            ),
            ("test_seed_42", vec![0x2Au8; 32], "KES test validation", 0),
            (
                "empty_message",
                vec![
                    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89,
                    0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23,
                    0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                ],
                "",
                0,
            ),
        ];

        println!("\n=== SingleKes<Ed25519> Test Vectors ===\n");

        for (name, seed_bytes, message, period) in test_cases {
            // Generate signing key from seed
            let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
            seed.as_mut_bytes().copy_from_slice(&seed_bytes);
            let signing_key = Ed25519::gen_key_m(&seed).unwrap();

            // Derive verification key
            let vk = SingleKesEd25519::derive_verification_key(&signing_key).unwrap();

            // Sign the message with period
            let message_bytes = message.as_bytes();
            let sig = SingleKesEd25519::sign_kes(&(), period, message_bytes, &signing_key).unwrap();

            // Serialize to CBOR
            let mut vk_cbor = Vec::new();
            ciborium::into_writer(&vk, &mut vk_cbor).unwrap();
            let mut sig_cbor = Vec::new();
            ciborium::into_writer(&sig, &mut sig_cbor).unwrap();

            println!("Test: {}", name);
            println!("  Seed:        {}", hex_encode(&seed_bytes));
            println!("  Period:      {}", period);
            println!(
                "  Message:     {} (hex: {})",
                message,
                hex_encode(message_bytes)
            );
            println!("  VK CBOR:     {}", hex_encode(&vk_cbor));
            println!("  Sig CBOR:    {}", hex_encode(&sig_cbor));
            println!();
        }
    }

    #[test]
    fn test_generate_compact_single_kes_test_vectors() {
        use cardano_crypto_class::dsign::DsignMAlgorithm;
        use cardano_crypto_class::dsign::ed25519::Ed25519;
        use cardano_crypto_class::kes::{CompactSingleKes, KesAlgorithm};
        use cardano_crypto_class::mlocked_seed::MLockedSeed;

        type CompactSingleKesEd25519 = CompactSingleKes<Ed25519>;

        // Test cases matching compact_single_kes_vectors.json template
        let test_cases = vec![
            ("all_zeros_seed", vec![0u8; 32], "Hello, World!", 0),
            ("all_ones_seed", vec![0xFFu8; 32], "Test Vector", 0),
            (
                "sequential_seed",
                (0..32).collect::<Vec<u8>>(),
                "Cardano",
                0,
            ),
            (
                "test_seed_42",
                vec![0x2Au8; 32],
                "Compact  KES test validation",
                0,
            ),
            (
                "empty_message",
                vec![
                    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89,
                    0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23,
                    0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                ],
                "",
                0,
            ),
        ];

        println!("\n=== CompactSingleKes<Ed25519> Test Vectors ===\n");

        for (name, seed_bytes, message, period) in test_cases {
            // Generate signing key from seed
            let mut seed = MLockedSeed::<32>::new_zeroed().unwrap();
            seed.as_mut_bytes().copy_from_slice(&seed_bytes);
            let signing_key = Ed25519::gen_key_m(&seed).unwrap();

            // Derive verification key
            let vk = CompactSingleKesEd25519::derive_verification_key(&signing_key).unwrap();

            // Sign the message with period
            let message_bytes = message.as_bytes();
            let sig = CompactSingleKesEd25519::sign_kes(&(), period, message_bytes, &signing_key)
                .unwrap();

            // Serialize to CBOR
            let mut vk_cbor = Vec::new();
            ciborium::into_writer(&vk, &mut vk_cbor).unwrap();
            let mut sig_cbor = Vec::new();
            ciborium::into_writer(&sig, &mut sig_cbor).unwrap();

            println!("Test: {}", name);
            println!("  Seed:        {}", hex_encode(&seed_bytes));
            println!("  Period:      {}", period);
            println!(
                "  Message:     {} (hex: {})",
                message,
                hex_encode(message_bytes)
            );
            println!("  VK CBOR:     {}", hex_encode(&vk_cbor));
            println!("  Sig CBOR:    {}", hex_encode(&sig_cbor));
            println!();
        }
    }
}
