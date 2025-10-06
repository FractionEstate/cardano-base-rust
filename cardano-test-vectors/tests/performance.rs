//! Simple performance smoke test for Ed25519 signing and verification.
//! Mirrors the style used by `cardano-vrf-pure/tests/performance.rs`.
//! Run with `cargo test --release -p cardano-test-vectors --test performance -- --nocapture`
//! for more stable numbers.

use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use std::time::Instant;

#[test]
fn measure_ed25519_throughput() {
    const ITERATIONS: usize = 200;

    println!("\n=== Ed25519 Performance Measurement ===");
    println!("Iterations: {}\n", ITERATIONS);

    let seed = hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
        .expect("valid RFC 8032 seed");
    let message = b"";

    let signing_key = <Ed25519 as DsignAlgorithm>::gen_key_from_seed_bytes(&seed);
    let verification_key = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing_key);

    // Warm up
    for _ in 0..10 {
        let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), message, &signing_key);
        <Ed25519 as DsignAlgorithm>::verify_bytes(&(), &verification_key, message, &signature)
            .expect("verification succeeds");
    }

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), message, &signing_key);
    }
    let sign_duration = start.elapsed();
    let sign_avg = sign_duration.as_micros() as f64 / ITERATIONS as f64;

    println!("📊 Sign:");
    println!("  Total time: {:?}", sign_duration);
    println!("  Average: {:.2} μs/op", sign_avg);
    println!("  Throughput: {:.2} ops/sec\n", 1_000_000.0 / sign_avg);

    let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), message, &signing_key);

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        <Ed25519 as DsignAlgorithm>::verify_bytes(&(), &verification_key, message, &signature)
            .expect("verification succeeds");
    }
    let verify_duration = start.elapsed();
    let verify_avg = verify_duration.as_micros() as f64 / ITERATIONS as f64;

    println!("📊 Verify:");
    println!("  Total time: {:?}", verify_duration);
    println!("  Average: {:.2} μs/op", verify_avg);
    println!("  Throughput: {:.2} ops/sec\n", 1_000_000.0 / verify_avg);

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let sig = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), message, &signing_key);
        <Ed25519 as DsignAlgorithm>::verify_bytes(&(), &verification_key, message, &sig)
            .expect("verification succeeds");
    }
    let roundtrip_duration = start.elapsed();
    let roundtrip_avg = roundtrip_duration.as_micros() as f64 / ITERATIONS as f64;

    println!("📊 Roundtrip:");
    println!("  Total time: {:?}", roundtrip_duration);
    println!("  Average: {:.2} μs/op", roundtrip_avg);
    println!("  Throughput: {:.2} ops/sec\n", 1_000_000.0 / roundtrip_avg);

    println!("=== Ed25519 Performance Summary ===");
    println!("┌────────────┬──────────────┬──────────────┐");
    println!("│ Operation  │ Avg Time (μs)│ Throughput  │");
    println!("├────────────┼──────────────┼──────────────┤");
    println!(
        "│ Sign       │ {:>12.2} │ {:>8.2} ops/s │",
        sign_avg,
        1_000_000.0 / sign_avg
    );
    println!(
        "│ Verify     │ {:>12.2} │ {:>8.2} ops/s │",
        verify_avg,
        1_000_000.0 / verify_avg
    );
    println!(
        "│ Roundtrip  │ {:>12.2} │ {:>8.2} ops/s │",
        roundtrip_avg,
        1_000_000.0 / roundtrip_avg
    );
    println!("└────────────┴──────────────┴──────────────┘");
    println!("\n✅ Performance measurement complete");
}
