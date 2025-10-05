// Test for hashVerKeyKES convenience method

#[test]
fn test_hash_verification_key_kes() {
    use cardano_crypto_class::kes::{
        Blake2b256, Blake2b512, KesAlgorithm, KesHashAlgorithm, SingleKes,
    };
    use cardano_crypto_class::Ed25519;

    // For this test, we just need any verification key
    // We'll create a dummy one by using the serialization/deserialization
    // since SingleKes doesn't have a simple way to generate keys in tests

    // Create a test verification key by deserializing known bytes (32 bytes for Ed25519)
    let test_vk_bytes = vec![1u8; 32]; // Simple test vector
    let verification_key =
        SingleKes::<Ed25519>::raw_deserialize_verification_key_kes(&test_vk_bytes)
            .expect("Failed to deserialize test vk");

    // Test hashing with Blake2b256
    let hash_256 = SingleKes::<Ed25519>::hash_verification_key_kes::<Blake2b256>(&verification_key);
    assert_eq!(hash_256.len(), 32, "Blake2b256 hash should be 32 bytes");

    // Test hashing with Blake2b512
    let hash_512 = SingleKes::<Ed25519>::hash_verification_key_kes::<Blake2b512>(&verification_key);
    assert_eq!(hash_512.len(), 64, "Blake2b512 hash should be 64 bytes");

    // Verify the hash is deterministic
    let hash_256_again =
        SingleKes::<Ed25519>::hash_verification_key_kes::<Blake2b256>(&verification_key);
    assert_eq!(hash_256, hash_256_again, "Hash should be deterministic");

    // Verify manual hash matches convenience method
    let serialized = SingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
    let manual_hash = Blake2b256::hash(&serialized);
    assert_eq!(
        hash_256, manual_hash,
        "Convenience method should match manual hashing"
    );

    println!("✅ hash_verification_key_kes works correctly");
    println!("   Blake2b256 hash length: {} bytes", hash_256.len());
    println!("   Blake2b512 hash length: {} bytes", hash_512.len());
}
