use cardano_binary::{decode_full, serialize};
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use serde::{Deserialize, Serialize};

// Small structure - minimal overhead test
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SmallStruct {
    id: u64,
    flag: bool,
}

// Medium structure - typical transaction metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct MediumStruct {
    tx_id: [u8; 32],
    inputs: Vec<u64>,
    outputs: Vec<u64>,
    metadata: Vec<(String, String)>,
}

// Large structure - block-like data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LargeStruct {
    header: Vec<u8>,
    transactions: Vec<MediumStruct>,
    signatures: Vec<Vec<u8>>, // Changed from [u8; 64] to Vec<u8>
}

fn small_struct_roundtrip(c: &mut Criterion) {
    let data = SmallStruct { id: 42, flag: true };

    let mut group = c.benchmark_group("cbor_small");
    group.throughput(Throughput::Elements(1));

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&data)).unwrap();
            black_box(bytes);
        });
    });

    let serialized = serialize(&data).unwrap();
    group.throughput(Throughput::Bytes(serialized.len() as u64));

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let decoded: SmallStruct = decode_full(black_box(&serialized)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("roundtrip", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&data)).unwrap();
            let decoded: SmallStruct = decode_full(&bytes).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn medium_struct_roundtrip(c: &mut Criterion) {
    let data = MediumStruct {
        tx_id: [0x42; 32],
        inputs: vec![1, 2, 3, 4, 5, 10, 20, 30],
        outputs: vec![100, 200, 300, 400, 500],
        metadata: vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ],
    };

    let mut group = c.benchmark_group("cbor_medium");
    group.throughput(Throughput::Elements(1));

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&data)).unwrap();
            black_box(bytes);
        });
    });

    let serialized = serialize(&data).unwrap();
    group.throughput(Throughput::Bytes(serialized.len() as u64));

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let decoded: MediumStruct = decode_full(black_box(&serialized)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("roundtrip", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&data)).unwrap();
            let decoded: MediumStruct = decode_full(&bytes).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn large_struct_roundtrip(c: &mut Criterion) {
    // Create multiple medium transactions
    let transactions: Vec<MediumStruct> = (0..10)
        .map(|i| MediumStruct {
            tx_id: [i as u8; 32],
            inputs: vec![i * 10, i * 10 + 1, i * 10 + 2],
            outputs: vec![i * 100, i * 100 + 50],
            metadata: vec![(format!("key{}", i), format!("value{}", i))],
        })
        .collect();

    let data = LargeStruct {
        header: (0..100)
            .flat_map(|_| vec![0x00, 0x01, 0x02, 0x03])
            .collect(),
        transactions,
        signatures: vec![vec![0xff; 64]; 10],
    };

    let mut group = c.benchmark_group("cbor_large");
    group.throughput(Throughput::Elements(1));

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&data)).unwrap();
            black_box(bytes);
        });
    });

    let serialized = serialize(&data).unwrap();
    group.throughput(Throughput::Bytes(serialized.len() as u64));

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let decoded: LargeStruct = decode_full(black_box(&serialized)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("roundtrip", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&data)).unwrap();
            let decoded: LargeStruct = decode_full(&bytes).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn collections_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cbor_collections");

    // Vec of integers
    let vec_data: Vec<u64> = (0..1000).collect();
    let vec_bytes = serialize(&vec_data).unwrap();
    group.throughput(Throughput::Bytes(vec_bytes.len() as u64));

    group.bench_function("vec_1000_u64/serialize", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&vec_data)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("vec_1000_u64/deserialize", |b| {
        b.iter(|| {
            let decoded: Vec<u64> = decode_full(black_box(&vec_bytes)).unwrap();
            black_box(decoded);
        });
    });

    // Map (via Vec of tuples for deterministic ordering)
    let map_data: Vec<(String, u64)> = (0..100)
        .map(|i| (format!("key_{:03}", i), i as u64))
        .collect();
    let map_bytes = serialize(&map_data).unwrap();
    group.throughput(Throughput::Bytes(map_bytes.len() as u64));

    group.bench_function("map_100_entries/serialize", |b| {
        b.iter(|| {
            let bytes = serialize(black_box(&map_data)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("map_100_entries/deserialize", |b| {
        b.iter(|| {
            let decoded: Vec<(String, u64)> = decode_full(black_box(&map_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    small_struct_roundtrip,
    medium_struct_roundtrip,
    large_struct_roundtrip,
    collections_benchmark
);
criterion_main!(benches);
