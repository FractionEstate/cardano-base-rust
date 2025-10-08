use cardano_crypto_class::hash::{self, Blake2b224, Blake2b256, Blake2b512};
use cardano_crypto_class::kes::hash::KesHashAlgorithm;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::time::Duration;

#[derive(Clone, Copy)]
struct HashBench {
    name: &'static str,
    func: fn(&[u8]) -> Vec<u8>,
}

const DATA_SIZES: &[(&str, usize)] = &[
    ("tiny_32b", 32),
    ("small_1kb", 1024),
    ("medium_64kb", 64 * 1024),
    ("large_1mb", 1024 * 1024),
];

const HASH_BENCHES: &[HashBench] = &[
    HashBench {
        name: "sha256",
        func: hash_sha256,
    },
    HashBench {
        name: "sha256d",
        func: hash_sha256d,
    },
    HashBench {
        name: "sha512",
        func: hash_sha512,
    },
    HashBench {
        name: "sha3_256",
        func: hash_sha3_256,
    },
    HashBench {
        name: "sha3_512",
        func: hash_sha3_512,
    },
    HashBench {
        name: "keccak256",
        func: hash_keccak256,
    },
    HashBench {
        name: "ripemd160",
        func: hash_ripemd160,
    },
    HashBench {
        name: "hash160",
        func: hash_hash160,
    },
    HashBench {
        name: "blake2b224",
        func: hash_blake2b224,
    },
    HashBench {
        name: "blake2b256",
        func: hash_blake2b256,
    },
    HashBench {
        name: "blake2b512",
        func: hash_blake2b512,
    },
];

fn patterned_data(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i % 251) as u8).collect()
}

fn hash_sha256(data: &[u8]) -> Vec<u8> {
    hash::sha256(data).to_vec()
}

fn hash_sha256d(data: &[u8]) -> Vec<u8> {
    hash::sha256d(data).to_vec()
}

fn hash_sha512(data: &[u8]) -> Vec<u8> {
    hash::sha512(data).to_vec()
}

fn hash_sha3_256(data: &[u8]) -> Vec<u8> {
    hash::sha3_256(data).to_vec()
}

fn hash_sha3_512(data: &[u8]) -> Vec<u8> {
    hash::sha3_512(data).to_vec()
}

fn hash_keccak256(data: &[u8]) -> Vec<u8> {
    hash::keccak256(data).to_vec()
}

fn hash_ripemd160(data: &[u8]) -> Vec<u8> {
    hash::ripemd160(data).to_vec()
}

fn hash_hash160(data: &[u8]) -> Vec<u8> {
    hash::hash160(data).to_vec()
}

fn hash_blake2b224(data: &[u8]) -> Vec<u8> {
    Blake2b224::hash(data)
}

fn hash_blake2b256(data: &[u8]) -> Vec<u8> {
    Blake2b256::hash(data)
}

fn hash_blake2b512(data: &[u8]) -> Vec<u8> {
    Blake2b512::hash(data)
}

fn bench_hashes(c: &mut Criterion) {
    let datasets: Vec<(String, Vec<u8>)> = DATA_SIZES
        .iter()
        .map(|(label, len)| ((*label).to_string(), patterned_data(*len)))
        .collect();

    for bench in HASH_BENCHES {
        let mut group = c.benchmark_group(format!("Hash/{}", bench.name));
        group.sample_size(30);
        group.warm_up_time(Duration::from_millis(200));
        group.measurement_time(Duration::from_secs(2));

        for (label, data) in &datasets {
            group.throughput(Throughput::Bytes(data.len() as u64));
            group.bench_with_input(BenchmarkId::new(bench.name, label), data, |b, input| {
                b.iter(|| {
                    let out = (bench.func)(black_box(input));
                    black_box(out);
                });
            });
        }

        group.finish();
    }
}

criterion_group! {
    name = hash_benches;
    config = Criterion::default();
    targets = bench_hashes
}
criterion_main!(hash_benches);
