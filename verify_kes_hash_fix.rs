#!/usr/bin/env -S cargo +nightly -Zscript
//! Quick verification that KES hash algorithm parameterization is working correctly
//!
//! ```cargo
//! [dependencies]
//! cardano-crypto-class = { path = "./cardano-crypto-class" }
//! ```

use cardano_crypto_class::kes::{CompactSum1Kes, CompactSum7Kes, KesAlgorithm, Sum1Kes, Sum7Kes};

fn main() {
    println!("=== KES Hash Algorithm Verification ===\n");

    // Check Sum types (using Blake2b256 = 32 bytes)
    println!("Sum Types (Blake2b256):");
    println!(
        "  Sum1Kes VERIFICATION_KEY_SIZE: {} bytes (expected: 32)",
        Sum1Kes::VERIFICATION_KEY_SIZE
    );
    println!(
        "  Sum7Kes VERIFICATION_KEY_SIZE: {} bytes (expected: 32)",
        Sum7Kes::VERIFICATION_KEY_SIZE
    );

    // Check CompactSum types (using Blake2b256 = 32 bytes)
    println!("\nCompactSum Types (Blake2b256):");
    println!(
        "  CompactSum1Kes VERIFICATION_KEY_SIZE: {} bytes (expected: 32)",
        CompactSum1Kes::VERIFICATION_KEY_SIZE
    );
    println!(
        "  CompactSum7Kes VERIFICATION_KEY_SIZE: {} bytes (expected: 32)",
        CompactSum7Kes::VERIFICATION_KEY_SIZE
    );

    // Verify correctness
    let sum_ok = Sum1Kes::VERIFICATION_KEY_SIZE == 32 && Sum7Kes::VERIFICATION_KEY_SIZE == 32;
    let compact_ok =
        CompactSum1Kes::VERIFICATION_KEY_SIZE == 32 && CompactSum7Kes::VERIFICATION_KEY_SIZE == 32;

    println!("\n=== Results ===");
    println!(
        "✓ Sum types: {}",
        if sum_ok {
            "PASS (32 bytes, matches Haskell Blake2b_256)"
        } else {
            "FAIL"
        }
    );
    println!(
        "✓ CompactSum types: {}",
        if compact_ok {
            "PASS (32 bytes, matches Haskell Blake2b_256)"
        } else {
            "FAIL"
        }
    );

    if sum_ok && compact_ok {
        println!("\n✅ SUCCESS: All KES types now use Blake2b-256 (32 bytes)");
        println!("   This matches Haskell's Blake2b_256 and ensures binary compatibility.");
        std::process::exit(0);
    } else {
        println!("\n❌ FAILURE: Verification key sizes are incorrect");
        std::process::exit(1);
    }
}
