use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{CompactSum4Kes, KesAlgorithm, SingleKes, Sum4Kes};
use criterion::{BenchmarkId, Criterion, SamplingMode, criterion_group, criterion_main};
use std::sync::Once;

// Deterministic seed bytes helper
fn seed_bytes<K: KesAlgorithm>() -> Vec<u8> {
    vec![0x42; K::SEED_SIZE]
}

fn bench_kes_alg<K>(c: &mut Criterion, name: &str)
where
    K: KesAlgorithm<Context = ()>,
    K::VerificationKey: Clone,
{
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        println!("== KES Size Baseline ==");
    });

    let mut group = c.benchmark_group(format!("KES/{}", name));
    group.sampling_mode(SamplingMode::Flat);

    // Key generation
    group.bench_function(BenchmarkId::new("keygen", name), |b| {
        b.iter(|| {
            let seed = seed_bytes::<K>();
            let _sk = K::gen_key_kes_from_seed_bytes(&seed).expect("signing key");
        })
    });

    // Sign / Verify across all periods (or limited sample if very large)
    let seed = seed_bytes::<K>();
    let sk_opt = K::gen_key_kes_from_seed_bytes(&seed).expect("signing key");
    let vk = K::derive_verification_key(&sk_opt).expect("verification key");

    let total = K::total_periods();
    let sample_periods: Vec<u64> = if total > 16 {
        // Sample first 8 and last 8 periods to keep runtime bounded
        (0..8).chain((total - 8)..total).collect()
    } else {
        (0..total).collect()
    };

    for period in sample_periods {
        // Pre-evolve a signing key to the target period once (outside timing) so that the
        // `sign` benchmark isolates signing cost and the `verify` benchmark isolates
        // verification cost without bundling period evolution overhead.
        let seed_p = seed_bytes::<K>();
        let mut sk_period = K::gen_key_kes_from_seed_bytes(&seed_p).expect("signing key");
        for e in 0..period {
            sk_period = K::update_kes(&(), sk_period, e)
                .expect("evolve")
                .expect("not expired before target period");
        }

        // Message used for signing throughput benchmark
        let sign_msg: Vec<u8> = format!("bench-sign-msg-{period}").into_bytes();
        // Message/signature pair used for verification throughput benchmark (distinct message
        // so verify does not share references with sign benchmark iterations).
        let verify_msg: Vec<u8> = format!("bench-verify-msg-{period}").into_bytes();
        let verify_sig = K::sign_kes(&(), period, &verify_msg, &sk_period).expect("sig");

        group.bench_function(BenchmarkId::new("sign", period), |b| {
            b.iter(|| {
                // Sign repeatedly with the pre-evolved key for this period.
                let _sig = K::sign_kes(&(), period, &sign_msg, &sk_period).expect("sign");
            })
        });

        group.bench_function(BenchmarkId::new("verify", period), |b| {
            b.iter(|| {
                K::verify_kes(&(), &vk, period, &verify_msg, &verify_sig).expect("verify");
            })
        });
    }

    // Evolution cost benchmark (sign + evolve loop)
    group.bench_function(BenchmarkId::new("evolve+sign", name), |b| {
        b.iter_custom(|iters| {
            use std::time::Instant;
            let start = Instant::now();
            for i in 0..iters {
                let seed_local = seed_bytes::<K>();
                let sk0 = K::gen_key_kes_from_seed_bytes(&seed_local).expect("signing key");
                let mut sk_state = Some(sk0);
                for period in 0..K::total_periods().min(16) {
                    // cap to 16 periods to bound runtime
                    let msg = format!("evo-{i}-{period}").into_bytes();
                    let active = sk_state.take().expect("active key must be present");
                    let _sig = K::sign_kes(&(), period, &msg, &active).expect("sign");
                    sk_state = K::update_kes(&(), active, period).expect("evolve");
                    if sk_state.is_none() {
                        break;
                    }
                }
            }
            start.elapsed()
        });
    });

    // Serialized size benchmarking (one-shot, reported as throughput over 1 item)
    group.bench_function(BenchmarkId::new("serialized_sizes", name), |b| {
        b.iter_custom(|_| {
            use std::time::Instant;
            let start = Instant::now();
            let seed = seed_bytes::<K>();
            let sk = K::gen_key_kes_from_seed_bytes(&seed).expect("sk");
            let vk = K::derive_verification_key(&sk).expect("vk");
            let msg = b"size-probe".to_vec();
            let sig = K::sign_kes(&(), 0, &msg, &sk).expect("sig");
            // We intentionally do NOT expose signing key raw serialization in benchmarks
            // because that requires the UnsoundKesAlgorithm trait (not implemented for
            // production types). Only public, sound serializations are reported.
            let vk_len = K::raw_serialize_verification_key_kes(&vk).len();
            let sig_len = K::raw_serialize_signature_kes(&sig).len();
            println!(
                "[size] {name}: vk={}B sig={}B periods={}",
                vk_len,
                sig_len,
                K::total_periods()
            );
            start.elapsed()
        })
    });

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_kes_alg::<SingleKes<Ed25519>>(c, "Single");
    bench_kes_alg::<Sum4Kes>(c, "Sum4");
    bench_kes_alg::<CompactSum4Kes>(c, "CompactSum4");
}

criterion_group! {
    name = kes;
    config = Criterion::default().warm_up_time(std::time::Duration::from_millis(200)).measurement_time(std::time::Duration::from_secs(2)).sample_size(30);
    targets = criterion_benchmark
}
criterion_main!(kes);
