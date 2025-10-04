// Test that KES types are properly exported at the top level

#[test]
fn test_kes_exports() {
    use cardano_crypto_class::{
        // Hash algorithms
        Blake2b256,
        Blake2b512,
        // Core traits
        KesAlgorithm,
        KesHashAlgorithm,
        // Sum type aliases
        Sum1Kes,
        Sum7Kes,
    };

    // Verify constants are accessible
    assert_eq!(
        Sum1Kes::VERIFICATION_KEY_SIZE,
        32,
        "Sum1Kes should use Blake2b256 (32 bytes)"
    );
    assert_eq!(
        Sum7Kes::VERIFICATION_KEY_SIZE,
        32,
        "Sum7Kes should use Blake2b256 (32 bytes)"
    );

    // Verify Blake2b256 hash size
    assert_eq!(Blake2b256::OUTPUT_SIZE, 32);
    assert_eq!(Blake2b512::OUTPUT_SIZE, 64);

    println!("✅ All KES types are properly exported at the top level");
    println!(
        "   Sum1Kes VK size: {} bytes",
        Sum1Kes::VERIFICATION_KEY_SIZE
    );
    println!(
        "   Sum7Kes VK size: {} bytes",
        Sum7Kes::VERIFICATION_KEY_SIZE
    );
}
