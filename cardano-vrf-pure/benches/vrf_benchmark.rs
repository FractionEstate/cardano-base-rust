//! Performance benchmarks for VRF operations
//!
//! Run with: cargo bench --bench vrf_benchmark

use cardano_vrf_pure::cardano_compat::{cardano_vrf_prove, cardano_vrf_verify};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

fn bench_vrf_prove(c: &mut Criterion) {
    let mut group = c.benchmark_group("vrf_prove");
    group.measurement_time(Duration::from_secs(10));

    // Standard test vector setup
    let sk_seed = hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
        .expect("valid hex");
    let pk = hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
        .expect("valid hex");

    let mut sk = [0u8; 64];
    sk[0..32].copy_from_slice(&sk_seed);
    sk[32..64].copy_from_slice(&pk);

    // Benchmark with different message sizes
    for msg_size in [0, 32, 256, 1024].iter() {
        let message = vec![0u8; *msg_size];

        group.bench_with_input(
            BenchmarkId::new("message_size", msg_size),
            &message,
            |b, msg| {
                b.iter(|| {
                    let proof =
                        cardano_vrf_prove(black_box(&sk), black_box(msg)).expect("prove succeeds");
                    black_box(proof);
                });
            },
        );
    }

    group.finish();
}

fn bench_vrf_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("vrf_verify");
    group.measurement_time(Duration::from_secs(10));

    // Standard test vector setup
    let sk_seed = hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
        .expect("valid hex");
    let pk_bytes = hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
        .expect("valid hex");

    let mut sk = [0u8; 64];
    sk[0..32].copy_from_slice(&sk_seed);
    sk[32..64].copy_from_slice(&pk_bytes);

    let mut pk = [0u8; 32];
    pk.copy_from_slice(&pk_bytes);

    // Benchmark with different message sizes
    for msg_size in [0, 32, 256, 1024].iter() {
        let message = vec![0u8; *msg_size];
        let proof = cardano_vrf_prove(&sk, &message).expect("prove succeeds");

        group.bench_with_input(
            BenchmarkId::new("message_size", msg_size),
            &(&pk, &proof, &message),
            |b, (pk, proof, msg)| {
                b.iter(|| {
                    let output =
                        cardano_vrf_verify(black_box(pk), black_box(proof), black_box(msg))
                            .expect("verify succeeds");
                    black_box(output);
                });
            },
        );
    }

    group.finish();
}

fn bench_vrf_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("vrf_roundtrip");
    group.measurement_time(Duration::from_secs(10));

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

    group.bench_function("prove_and_verify", |b| {
        b.iter(|| {
            let proof =
                cardano_vrf_prove(black_box(&sk), black_box(message)).expect("prove succeeds");
            let output = cardano_vrf_verify(black_box(&pk), black_box(&proof), black_box(message))
                .expect("verify succeeds");
            black_box(output);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_vrf_prove,
    bench_vrf_verify,
    bench_vrf_roundtrip
);
criterion_main!(benches);
