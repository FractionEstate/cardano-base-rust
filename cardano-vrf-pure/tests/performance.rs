//! Simple performance measurement for VRF operations
//!
//! Run with: cargo test --release -p cardano-vrf-pure --test performance -- --nocapture

use cardano_vrf_pure::cardano_compat::{cardano_vrf_prove, cardano_vrf_verify};
use std::time::Instant;

#[test]
fn measure_vrf_performance() {
    const ITERATIONS: usize = 1000;

    println!("\n=== VRF Performance Measurement ===");
    println!("Iterations: {}\n", ITERATIONS);

    // Setup test vectors
    let sk_seed = hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
        .expect("valid hex");
    let pk_bytes = hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
        .expect("valid hex");

    let mut sk = [0u8; 64];
    sk[0..32].copy_from_slice(&sk_seed);
    sk[32..64].copy_from_slice(&pk_bytes);

    let mut pk = [0u8; 32];
    pk.copy_from_slice(&pk_bytes);

    let message = b"";

    // Warm-up
    for _ in 0..10 {
        let proof = cardano_vrf_prove(&sk, message).expect("prove succeeds");
        let _ = cardano_vrf_verify(&pk, &proof, message).expect("verify succeeds");
    }

    // Measure prove performance
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = cardano_vrf_prove(&sk, message).expect("prove succeeds");
    }
    let prove_duration = start.elapsed();
    let prove_avg = prove_duration.as_micros() as f64 / ITERATIONS as f64;

    println!("📊 VRF Prove:");
    println!("  Total time: {:?}", prove_duration);
    println!("  Average: {:.2} μs per operation", prove_avg);
    println!("  Throughput: {:.2} ops/sec\n", 1_000_000.0 / prove_avg);

    // Pre-generate proof for verify benchmarks
    let proof = cardano_vrf_prove(&sk, message).expect("prove succeeds");

    // Measure verify performance
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = cardano_vrf_verify(&pk, &proof, message).expect("verify succeeds");
    }
    let verify_duration = start.elapsed();
    let verify_avg = verify_duration.as_micros() as f64 / ITERATIONS as f64;

    println!("📊 VRF Verify:");
    println!("  Total time: {:?}", verify_duration);
    println!("  Average: {:.2} μs per operation", verify_avg);
    println!("  Throughput: {:.2} ops/sec\n", 1_000_000.0 / verify_avg);

    // Measure roundtrip performance
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let proof = cardano_vrf_prove(&sk, message).expect("prove succeeds");
        let _ = cardano_vrf_verify(&pk, &proof, message).expect("verify succeeds");
    }
    let roundtrip_duration = start.elapsed();
    let roundtrip_avg = roundtrip_duration.as_micros() as f64 / ITERATIONS as f64;

    println!("📊 VRF Roundtrip (Prove + Verify):");
    println!("  Total time: {:?}", roundtrip_duration);
    println!("  Average: {:.2} μs per operation", roundtrip_avg);
    println!("  Throughput: {:.2} ops/sec\n", 1_000_000.0 / roundtrip_avg);

    // Performance summary
    println!("=== Performance Summary ===");
    println!("┌─────────────────┬──────────────┬─────────────┐");
    println!("│ Operation       │ Avg Time (μs)│ Throughput  │");
    println!("├─────────────────┼──────────────┼─────────────┤");
    println!(
        "│ Prove           │ {:>12.2} │ {:>7.2} ops/s │",
        prove_avg,
        1_000_000.0 / prove_avg
    );
    println!(
        "│ Verify          │ {:>12.2} │ {:>7.2} ops/s │",
        verify_avg,
        1_000_000.0 / verify_avg
    );
    println!(
        "│ Roundtrip       │ {:>12.2} │ {:>7.2} ops/s │",
        roundtrip_avg,
        1_000_000.0 / roundtrip_avg
    );
    println!("└─────────────────┴──────────────┴─────────────┘");
    println!("\n✅ Performance measurement complete");
}

#[test]
fn measure_vrf_with_different_message_sizes() {
    const ITERATIONS: usize = 100;

    println!("\n=== VRF Performance by Message Size ===\n");

    let sk_seed = hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
        .expect("valid hex");
    let pk_bytes = hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
        .expect("valid hex");

    let mut sk = [0u8; 64];
    sk[0..32].copy_from_slice(&sk_seed);
    sk[32..64].copy_from_slice(&pk_bytes);

    let mut pk = [0u8; 32];
    pk.copy_from_slice(&pk_bytes);

    println!("┌──────────────┬──────────────┬──────────────┐");
    println!("│ Message Size │ Prove (μs)   │ Verify (μs)  │");
    println!("├──────────────┼──────────────┼──────────────┤");

    for msg_size in [0, 32, 256, 1024, 4096] {
        let message = vec![0u8; msg_size];

        // Measure prove
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = cardano_vrf_prove(&sk, &message).expect("prove succeeds");
        }
        let prove_avg = start.elapsed().as_micros() as f64 / ITERATIONS as f64;

        // Measure verify
        let proof = cardano_vrf_prove(&sk, &message).expect("prove succeeds");
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = cardano_vrf_verify(&pk, &proof, &message).expect("verify succeeds");
        }
        let verify_avg = start.elapsed().as_micros() as f64 / ITERATIONS as f64;

        println!(
            "│ {:>12} │ {:>12.2} │ {:>12.2} │",
            format!("{} bytes", msg_size),
            prove_avg,
            verify_avg
        );
    }

    println!("└──────────────┴──────────────┴──────────────┘");
    println!("\n✅ Message size performance measurement complete");
}
