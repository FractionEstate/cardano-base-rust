// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Cross-algorithm verification tests.
//!
//! This test suite validates interoperability between different cryptographic
//! algorithms and ensures consistent behavior across the codebase.

use cardano_crypto_class::dsign::{
    DsignAlgorithm, ecdsa_secp256k1, ed25519::Ed25519, schnorr_secp256k1,
};
use cardano_crypto_class::hash::*;
use cardano_crypto_class::seed::Seed;

/// Test that all DSIGN algorithms can derive keys from the same seed deterministically.
#[test]
fn test_deterministic_key_derivation_across_algorithms() {
    let seed_bytes = [42u8; 32];
    let seed = Seed::from_bytes(&seed_bytes);

    // Ed25519
    let ed25519_key1 = Ed25519::gen_key(&seed);
    let ed25519_key2 = Ed25519::gen_key(&seed);
    assert_eq!(
        Ed25519::raw_serialize_signing_key(&ed25519_key1),
        Ed25519::raw_serialize_signing_key(&ed25519_key2)
    );

    // ECDSA Secp256k1
    let ecdsa_key1 = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);
    let ecdsa_key2 = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);
    assert_eq!(
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&ecdsa_key1),
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&ecdsa_key2)
    );

    // Schnorr Secp256k1
    let schnorr_key1 = schnorr_secp256k1::SchnorrSecp256k1DSIGN::gen_key(&seed);
    let schnorr_key2 = schnorr_secp256k1::SchnorrSecp256k1DSIGN::gen_key(&seed);
    assert_eq!(
        schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&schnorr_key1),
        schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&schnorr_key2)
    );
}
/// Test that different algorithms produce different signatures for the same message.
#[test]
fn test_algorithm_signature_uniqueness() {
    let seed = Seed::from_bytes(&[99u8; 32]);
    let message = b"Test message for signature uniqueness";

    // Generate keys
    let ed25519_sk = Ed25519::gen_key(&seed);
    let ecdsa_sk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);
    let schnorr_sk = schnorr_secp256k1::SchnorrSecp256k1DSIGN::gen_key(&seed);

    // Sign with each algorithm
    let ed25519_ctx = ();
    let ecdsa_ctx = ecdsa_secp256k1::Context;
    let schnorr_ctx = schnorr_secp256k1::Context;

    let ed25519_sig = Ed25519::sign_bytes(&ed25519_ctx, message, &ed25519_sk);
    let ecdsa_sig =
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::sign_bytes(&ecdsa_ctx, message, &ecdsa_sk);
    let schnorr_sig =
        schnorr_secp256k1::SchnorrSecp256k1DSIGN::sign_bytes(&schnorr_ctx, message, &schnorr_sk);

    // Signatures should be different (different algorithms)
    let ed25519_bytes = Ed25519::raw_serialize_signature(&ed25519_sig);
    let ecdsa_bytes = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_signature(&ecdsa_sig);
    let schnorr_bytes =
        schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_serialize_signature(&schnorr_sig);

    // All signatures are 64 bytes but content should be different
    assert_eq!(ed25519_bytes.len(), 64);
    assert_eq!(ecdsa_bytes.len(), 64);
    assert_eq!(schnorr_bytes.len(), 64);

    // Different algorithms should produce different signatures for same message
    assert_ne!(ed25519_bytes, ecdsa_bytes);
    assert_ne!(ed25519_bytes, schnorr_bytes);
    assert_ne!(ecdsa_bytes, schnorr_bytes);
}

/// Test that key size constants are consistent across algorithms.
#[test]
fn test_algorithm_key_size_constants() {
    // Ed25519
    assert_eq!(Ed25519::SEED_SIZE, 32);
    assert_eq!(Ed25519::SIGNING_KEY_SIZE, 32);
    assert_eq!(Ed25519::VERIFICATION_KEY_SIZE, 32);
    assert_eq!(Ed25519::SIGNATURE_SIZE, 64);

    // ECDSA Secp256k1
    assert_eq!(ecdsa_secp256k1::EcdsaSecp256k1DSIGN::SEED_SIZE, 32);
    assert_eq!(ecdsa_secp256k1::EcdsaSecp256k1DSIGN::SIGNING_KEY_SIZE, 32);
    assert_eq!(
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::VERIFICATION_KEY_SIZE,
        33
    ); // Compressed
    assert_eq!(ecdsa_secp256k1::EcdsaSecp256k1DSIGN::SIGNATURE_SIZE, 64);

    // Schnorr Secp256k1
    assert_eq!(schnorr_secp256k1::SchnorrSecp256k1DSIGN::SEED_SIZE, 32);
    assert_eq!(
        schnorr_secp256k1::SchnorrSecp256k1DSIGN::SIGNING_KEY_SIZE,
        32
    );
    assert_eq!(
        schnorr_secp256k1::SchnorrSecp256k1DSIGN::VERIFICATION_KEY_SIZE,
        32
    ); // X-only
    assert_eq!(schnorr_secp256k1::SchnorrSecp256k1DSIGN::SIGNATURE_SIZE, 64);
}

/// Test that verification fails when using wrong verification key.
#[test]
fn test_cross_key_verification_fails() {
    let seed1 = Seed::from_bytes(&[1u8; 32]);
    let seed2 = Seed::from_bytes(&[2u8; 32]);
    let message = b"Test message";

    // Ed25519
    let ed25519_sk1 = Ed25519::gen_key(&seed1);
    let ed25519_vk1 = Ed25519::derive_verification_key(&ed25519_sk1);
    let ed25519_vk2 = Ed25519::derive_verification_key(&Ed25519::gen_key(&seed2));

    let ed25519_ctx = ();
    let ed25519_sig = Ed25519::sign_bytes(&ed25519_ctx, message, &ed25519_sk1);

    assert!(Ed25519::verify_bytes(&ed25519_ctx, &ed25519_vk1, message, &ed25519_sig).is_ok());
    assert!(Ed25519::verify_bytes(&ed25519_ctx, &ed25519_vk2, message, &ed25519_sig).is_err());

    // ECDSA
    let ecdsa_sk1 = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed1);
    let ecdsa_vk1 = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::derive_verification_key(&ecdsa_sk1);
    let ecdsa_vk2 = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::derive_verification_key(
        &ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed2),
    );

    let ecdsa_ctx = ecdsa_secp256k1::Context;
    let ecdsa_sig =
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::sign_bytes(&ecdsa_ctx, message, &ecdsa_sk1);

    assert!(
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::verify_bytes(
            &ecdsa_ctx, &ecdsa_vk1, message, &ecdsa_sig
        )
        .is_ok()
    );
    assert!(
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::verify_bytes(
            &ecdsa_ctx, &ecdsa_vk2, message, &ecdsa_sig
        )
        .is_err()
    );
}

/// Test hash function output sizes are correct.
#[test]
fn test_hash_output_sizes() {
    let data = b"test data";

    assert_eq!(sha256(data).len(), 32);
    assert_eq!(sha256d(data).len(), 32);
    assert_eq!(sha512(data).len(), 64);
    assert_eq!(sha3_256(data).len(), 32);
    assert_eq!(sha3_512(data).len(), 64);
    assert_eq!(keccak256(data).len(), 32);
    assert_eq!(ripemd160(data).len(), 20);
    assert_eq!(hash160(data).len(), 20);
}

/// Test that hash functions are deterministic.
#[test]
fn test_hash_determinism() {
    let data = b"deterministic test data";

    // Each hash should produce the same output every time
    assert_eq!(sha256(data), sha256(data));
    assert_eq!(sha256d(data), sha256d(data));
    assert_eq!(sha512(data), sha512(data));
    assert_eq!(sha3_256(data), sha3_256(data));
    assert_eq!(sha3_512(data), sha3_512(data));
    assert_eq!(keccak256(data), keccak256(data));
    assert_eq!(ripemd160(data), ripemd160(data));
    assert_eq!(hash160(data), hash160(data));
}

/// Test that different hash functions produce different outputs.
#[test]
fn test_hash_uniqueness() {
    let data = b"test data for uniqueness";

    let sha256_hash = sha256(data);
    let sha3_256_hash = sha3_256(data);
    let keccak256_hash = keccak256(data);

    // All 32-byte hashes but should be different
    assert_ne!(sha256_hash, sha3_256_hash);
    assert_ne!(sha256_hash, keccak256_hash);
    assert_ne!(sha3_256_hash, keccak256_hash);
}

/// Test composite hash functions (sha256d, hash160).
#[test]
fn test_composite_hash_functions() {
    let data = b"test composite hashing";

    // SHA256d should be SHA256(SHA256(data))
    let sha256d_result = sha256d(data);
    let manual_sha256d = sha256(&sha256(data));
    assert_eq!(sha256d_result, manual_sha256d);

    // Hash160 should be RIPEMD160(SHA256(data))
    let hash160_result = hash160(data);
    let manual_hash160 = ripemd160(&sha256(data));
    assert_eq!(hash160_result, manual_hash160);
}

/// Test that serialization roundtrip works for all algorithms.
#[test]
fn test_serialization_roundtrip_all_algorithms() {
    let seed = Seed::from_bytes(&[123u8; 32]);

    // Ed25519
    {
        let sk = Ed25519::gen_key(&seed);
        let vk = Ed25519::derive_verification_key(&sk);

        let sk_bytes = Ed25519::raw_serialize_signing_key(&sk);
        let vk_bytes = Ed25519::raw_serialize_verification_key(&vk);

        let sk_restored = Ed25519::raw_deserialize_signing_key(&sk_bytes)
            .expect("ed25519 signing key deserialise");
        let vk_restored = Ed25519::raw_deserialize_verification_key(&vk_bytes)
            .expect("ed25519 verification key deserialise");

        assert_eq!(Ed25519::raw_serialize_signing_key(&sk_restored), sk_bytes);
        assert_eq!(vk, vk_restored);
    }

    // ECDSA
    {
        let sk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);
        let vk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::derive_verification_key(&sk);

        let sk_bytes = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&sk);
        let vk_bytes = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_verification_key(&vk);

        let sk_restored =
            ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_deserialize_signing_key(&sk_bytes)
                .expect("ecdsa signing key deserialise");
        let vk_restored =
            ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_deserialize_verification_key(&vk_bytes)
                .expect("ecdsa verification key deserialise");

        assert_eq!(
            ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&sk_restored),
            sk_bytes
        );
        assert_eq!(vk, vk_restored);
    }

    // Schnorr
    {
        let sk = schnorr_secp256k1::SchnorrSecp256k1DSIGN::gen_key(&seed);
        let vk = schnorr_secp256k1::SchnorrSecp256k1DSIGN::derive_verification_key(&sk);

        let sk_bytes = schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&sk);
        let vk_bytes =
            schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_serialize_verification_key(&vk);

        let sk_restored =
            schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_deserialize_signing_key(&sk_bytes)
                .expect("schnorr signing key deserialise");
        let vk_restored =
            schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_deserialize_verification_key(&vk_bytes)
                .expect("schnorr verification key deserialise");

        assert_eq!(
            schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&sk_restored),
            sk_bytes
        );
        assert_eq!(vk, vk_restored);
    }
}

/// Test that message tampering is detected by all algorithms.
#[test]
fn test_message_tampering_detection() {
    let seed = Seed::from_bytes(&[55u8; 32]);
    let original_message = b"Original message";
    let tampered_message = b"Tampered message";

    // Ed25519
    {
        let sk = Ed25519::gen_key(&seed);
        let vk = Ed25519::derive_verification_key(&sk);
        let ctx = ();

        let sig = Ed25519::sign_bytes(&ctx, original_message, &sk);
        assert!(Ed25519::verify_bytes(&ctx, &vk, original_message, &sig).is_ok());
        assert!(Ed25519::verify_bytes(&ctx, &vk, tampered_message, &sig).is_err());
    } // ECDSA
    {
        let sk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);
        let vk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::derive_verification_key(&sk);
        let ctx = ecdsa_secp256k1::Context;

        let sig = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::sign_bytes(&ctx, original_message, &sk);
        assert!(
            ecdsa_secp256k1::EcdsaSecp256k1DSIGN::verify_bytes(&ctx, &vk, original_message, &sig)
                .is_ok()
        );
        assert!(
            ecdsa_secp256k1::EcdsaSecp256k1DSIGN::verify_bytes(&ctx, &vk, tampered_message, &sig)
                .is_err()
        );
    }

    // Schnorr
    {
        let sk = schnorr_secp256k1::SchnorrSecp256k1DSIGN::gen_key(&seed);
        let vk = schnorr_secp256k1::SchnorrSecp256k1DSIGN::derive_verification_key(&sk);
        let ctx = schnorr_secp256k1::Context;

        let sig = schnorr_secp256k1::SchnorrSecp256k1DSIGN::sign_bytes(&ctx, original_message, &sk);
        assert!(
            schnorr_secp256k1::SchnorrSecp256k1DSIGN::verify_bytes(
                &ctx,
                &vk,
                original_message,
                &sig
            )
            .is_ok()
        );
        assert!(
            schnorr_secp256k1::SchnorrSecp256k1DSIGN::verify_bytes(
                &ctx,
                &vk,
                tampered_message,
                &sig
            )
            .is_err()
        );
    }
}

/// Test integration: Bitcoin-style transaction signing workflow.
#[test]
fn test_bitcoin_workflow_integration() {
    let seed = Seed::from_bytes(&[88u8; 32]);

    // Generate ECDSA key for Bitcoin
    let sk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);
    let vk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::derive_verification_key(&sk);

    // Simulate Bitcoin transaction
    let tx_data = b"mock bitcoin transaction data";

    // Bitcoin uses double SHA-256 for transaction hashing
    let tx_hash = sha256d(tx_data);

    // Sign the transaction hash
    let ctx = ecdsa_secp256k1::Context;
    let signature = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::sign_bytes(&ctx, &tx_hash, &sk);

    // Verify signature
    assert!(
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::verify_bytes(&ctx, &vk, &tx_hash, &signature).is_ok()
    );

    // Generate Bitcoin address (Hash160 of public key)
    let vk_bytes = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_verification_key(&vk);
    let address_hash = hash160(&vk_bytes);
    assert_eq!(address_hash.len(), 20);
}

/// Test integration: Ethereum-style transaction signing workflow.
#[test]
fn test_ethereum_workflow_integration() {
    let seed = Seed::from_bytes(&[99u8; 32]);

    // Generate ECDSA key for Ethereum
    let sk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);
    let vk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::derive_verification_key(&sk);

    // Simulate Ethereum transaction (RLP-encoded)
    let tx_data = b"mock ethereum transaction data";

    // Ethereum uses Keccak-256 for transaction hashing
    let tx_hash = keccak256(tx_data);

    // Sign the transaction hash
    let ctx = ecdsa_secp256k1::Context;
    let signature = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::sign_bytes(&ctx, &tx_hash, &sk);

    // Verify signature
    assert!(
        ecdsa_secp256k1::EcdsaSecp256k1DSIGN::verify_bytes(&ctx, &vk, &tx_hash, &signature).is_ok()
    );

    // Generate Ethereum address (last 20 bytes of Keccak-256 of public key)
    let vk_bytes = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_verification_key(&vk);
    // Skip first byte (compression flag) for Ethereum
    let vk_hash = keccak256(&vk_bytes[1..]);
    let eth_address = &vk_hash[12..]; // Last 20 bytes
    assert_eq!(eth_address.len(), 20);
}

/// Test integration: Bitcoin Taproot (Schnorr) workflow.
#[test]
fn test_bitcoin_taproot_workflow_integration() {
    let seed = Seed::from_bytes(&[77u8; 32]);

    // Generate Schnorr key for Taproot
    let sk = schnorr_secp256k1::SchnorrSecp256k1DSIGN::gen_key(&seed);
    let vk = schnorr_secp256k1::SchnorrSecp256k1DSIGN::derive_verification_key(&sk);

    // Simulate Taproot transaction
    let tx_data = b"mock taproot transaction data";
    let tx_hash = sha256d(tx_data);

    // Sign with Schnorr
    let ctx = schnorr_secp256k1::Context;
    let signature = schnorr_secp256k1::SchnorrSecp256k1DSIGN::sign_bytes(&ctx, &tx_hash, &sk);

    // Verify signature
    assert!(
        schnorr_secp256k1::SchnorrSecp256k1DSIGN::verify_bytes(&ctx, &vk, &tx_hash, &signature)
            .is_ok()
    );

    // Taproot uses x-only public key (32 bytes)
    let vk_bytes = schnorr_secp256k1::SchnorrSecp256k1DSIGN::raw_serialize_verification_key(&vk);
    assert_eq!(vk_bytes.len(), 32);
}
