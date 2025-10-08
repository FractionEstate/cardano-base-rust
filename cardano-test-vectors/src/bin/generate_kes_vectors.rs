use std::borrow::Cow;
use std::collections::BTreeSet;
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{
    CompactSingleKes, CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, CompactSum4Kes,
    CompactSum5Kes, CompactSum6Kes, CompactSum7Kes, KesAlgorithm, SingleKes, Sum1Kes, Sum2Kes,
    Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes, Sum7Kes,
};
use hex::encode_upper;
use serde::Serialize;

#[derive(Debug, Clone)]
struct VectorDefinition {
    test_name: Cow<'static, str>,
    seed_hex: Cow<'static, str>,
    message_hex: Cow<'static, str>,
    description: Cow<'static, str>,
}

fn generate_seed_hex(base_offset: u8, index: usize) -> String {
    let mut seed = [0u8; 32];
    let start = base_offset.wrapping_add((index as u8).wrapping_mul(17));
    for (i, byte) in seed.iter_mut().enumerate() {
        *byte = start.wrapping_add(i as u8);
    }
    encode_upper(seed)
}

fn generate_message_hex(base_offset: u8, index: usize, length: usize) -> String {
    let mut message = Vec::with_capacity(length);
    let start = base_offset.wrapping_add((index as u8).wrapping_mul(29));
    for offset in 0..length {
        message.push(start.wrapping_add(offset as u8));
    }
    encode_upper(message)
}

fn single_vector_definitions() -> Vec<VectorDefinition> {
    let mut definitions = vec![
        VectorDefinition {
            test_name: Cow::Borrowed("single_kes_vector_1"),
            seed_hex: Cow::Borrowed(
                "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
            ),
            message_hex: Cow::Borrowed(""),
            description: Cow::Borrowed("Zero-message signing with sequential seed bytes"),
        },
        VectorDefinition {
            test_name: Cow::Borrowed("single_kes_vector_2"),
            seed_hex: Cow::Borrowed(
                "1F1E1D1C1B1A191817161514131211100F0E0D0C0B0A09080706050403020100",
            ),
            message_hex: Cow::Borrowed("4B45532053696E676C6520506572696F64"),
            description: Cow::Borrowed("ASCII message 'KES Single Period' with reversed seed"),
        },
        VectorDefinition {
            test_name: Cow::Borrowed("single_kes_vector_3"),
            seed_hex: Cow::Borrowed(
                "B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF",
            ),
            message_hex: Cow::Borrowed(
                "243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89",
            ),
            description: Cow::Borrowed("Cardano property-test seed with ChaCha-inspired message"),
        },
        VectorDefinition {
            test_name: Cow::Borrowed("single_kes_vector_4"),
            seed_hex: Cow::Borrowed(
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ),
            message_hex: Cow::Borrowed(
                "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            ),
            description: Cow::Borrowed("All-0xFF seed and message for upper-bound coverage"),
        },
    ];

    let target = 12;
    for index in definitions.len()..target {
        let seed = generate_seed_hex(0x20, index);
        let message = generate_message_hex(0x40, index, 24);
        definitions.push(VectorDefinition {
            test_name: Cow::Owned(format!("single_kes_vector_{}", index + 1)),
            seed_hex: Cow::Owned(seed),
            message_hex: Cow::Owned(message),
            description: Cow::Owned(format!(
                "Generated deterministic vector {} for broader parity coverage",
                index + 1
            )),
        });
    }

    definitions
}

fn hierarchical_vector_definitions() -> Vec<VectorDefinition> {
    let mut definitions = vec![
        VectorDefinition {
            test_name: Cow::Borrowed("hierarchical_vector_1"),
            seed_hex: Cow::Borrowed(
                "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
            ),
            message_hex: Cow::Borrowed(""),
            description: Cow::Borrowed("Zero-message signing with sequential seed bytes"),
        },
        VectorDefinition {
            test_name: Cow::Borrowed("hierarchical_vector_2"),
            seed_hex: Cow::Borrowed(
                "1F1E1D1C1B1A191817161514131211100F0E0D0C0B0A09080706050403020100",
            ),
            message_hex: Cow::Borrowed("4B45532053696E676C6520506572696F64"),
            description: Cow::Borrowed("ASCII message 'KES Single Period' with reversed seed"),
        },
    ];

    let target = 32;
    for index in definitions.len()..target {
        let seed = generate_seed_hex(0x80, index);
        let message = generate_message_hex(0xA0, index, 32);
        definitions.push(VectorDefinition {
            test_name: Cow::Owned(format!("hierarchical_vector_{}", index + 1)),
            seed_hex: Cow::Owned(seed),
            message_hex: Cow::Owned(message),
            description: Cow::Owned(format!(
                "Generated hierarchical vector {} for extended coverage",
                index + 1
            )),
        });
    }

    definitions
}

fn period_evolution_subset(definitions: &[VectorDefinition]) -> Vec<VectorDefinition> {
    let take = definitions.len().min(6);
    definitions.iter().take(take).cloned().collect()
}

#[derive(Serialize)]
struct SingleKesVectors {
    description: &'static str,
    algorithm: &'static str,
    source: &'static str,
    vectors: Vec<SingleKesVectorEntry>,
}

#[derive(Serialize)]
struct SingleKesVectorEntry {
    test_name: String,
    seed: String,
    message: String,
    period: u64,
    description: String,
    expected: SingleKesExpected,
}

#[derive(Serialize)]
struct SingleKesExpected {
    verification_key: String,
    signature: String,
    raw_signature: String,
}

#[derive(Serialize)]
struct CompactSingleKesVectors {
    description: &'static str,
    algorithm: &'static str,
    source: &'static str,
    vectors: Vec<CompactSingleKesVectorEntry>,
}

#[derive(Serialize)]
struct CompactSingleKesVectorEntry {
    test_name: String,
    seed: String,
    message: String,
    period: u64,
    description: String,
    expected: CompactSingleExpected,
}

#[derive(Serialize)]
struct CompactSingleExpected {
    derived_verification_key: String,
    embedded_verification_key: String,
    signature: String,
    raw_signature: String,
}

#[derive(Serialize)]
struct SumKesVectors {
    description: &'static str,
    algorithm: &'static str,
    source: &'static str,
    levels: Vec<SumKesLevel>,
}

#[derive(Serialize)]
struct CompactSumKesVectors {
    description: &'static str,
    algorithm: &'static str,
    source: &'static str,
    levels: Vec<SumKesLevel>,
}

#[derive(Serialize)]
struct SumKesLevel {
    level: u8,
    total_periods: u64,
    vectors: Vec<SumKesVectorEntry>,
}

#[derive(Serialize)]
struct SumKesVectorEntry {
    test_name: String,
    seed: String,
    description: String,
    verification_key: String,
    tracked_periods: Vec<PeriodVectorEntry>,
}

#[derive(Serialize, Clone)]
struct PeriodVectorEntry {
    period: u64,
    message: String,
    signature: String,
    raw_signature: String,
}

#[derive(Serialize)]
struct PeriodEvolutionVectors {
    description: &'static str,
    algorithm: &'static str,
    source: &'static str,
    levels: Vec<PeriodEvolutionLevel>,
}

#[derive(Serialize)]
struct PeriodEvolutionLevel {
    level: u8,
    total_periods: u64,
    vectors: Vec<PeriodEvolutionEntry>,
}

#[derive(Serialize)]
struct PeriodEvolutionEntry {
    test_name: String,
    seed: String,
    description: String,
    verification_key: String,
    periods: Vec<PeriodVectorEntry>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let single_definitions = single_vector_definitions();
    let hierarchical_definitions = hierarchical_vector_definitions();
    let period_defs = period_evolution_subset(&hierarchical_definitions);

    let single_vectors = build_single_kes_vectors(&single_definitions)?;
    let compact_vectors = build_compact_single_kes_vectors(&single_definitions)?;
    let sum_vectors = build_sum_kes_vectors(&hierarchical_definitions)?;
    let compact_sum_vectors = build_compact_sum_kes_vectors(&hierarchical_definitions)?;
    let sum_period_evolution_vectors = build_sum_kes_period_evolution_vectors(&period_defs)?;
    let compact_sum_period_evolution_vectors =
        build_compact_sum_kes_period_evolution_vectors(&period_defs)?;

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = manifest_dir.join("test_vectors");

    fs::create_dir_all(&output_dir)?;

    let single_path = output_dir.join("single_kes_test_vectors.json");
    let compact_single_path = output_dir.join("compact_single_kes_test_vectors.json");
    let sum_path = output_dir.join("sum_kes_test_vectors.json");
    let compact_sum_path = output_dir.join("compact_sum_kes_test_vectors.json");
    let sum_period_path = output_dir.join("sum_kes_period_evolution_vectors.json");
    let compact_sum_period_path = output_dir.join("compact_sum_kes_period_evolution_vectors.json");

    write_json(&single_path, &single_vectors)?;
    write_json(&compact_single_path, &compact_vectors)?;
    write_json(&sum_path, &sum_vectors)?;
    write_json(&compact_sum_path, &compact_sum_vectors)?;
    write_json(&sum_period_path, &sum_period_evolution_vectors)?;
    write_json(
        &compact_sum_period_path,
        &compact_sum_period_evolution_vectors,
    )?;

    println!(
        "Generated {}, {}, {}, {}, {}, and {}",
        single_path.display(),
        compact_single_path.display(),
        sum_path.display(),
        compact_sum_path.display(),
        sum_period_path.display(),
        compact_sum_period_path.display()
    );

    Ok(())
}

fn build_single_kes_vectors(
    definitions: &[VectorDefinition],
) -> Result<SingleKesVectors, Box<dyn std::error::Error>> {
    let mut vectors = Vec::with_capacity(definitions.len());

    for def in definitions {
        let seed_bytes = decode_seed(def.seed_hex.as_ref())?;
        let message_bytes = decode_hex(def.message_hex.as_ref())?;

        let signing_key = SingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed_bytes)?;
        let verification_key = SingleKes::<Ed25519>::derive_verification_key(&signing_key)?;
        let signature = SingleKes::<Ed25519>::sign_kes(&(), 0, &message_bytes, &signing_key)?;

        let vk_bytes = SingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
        let signature_bytes = SingleKes::<Ed25519>::raw_serialize_signature_kes(&signature);

        SingleKes::<Ed25519>::forget_signing_key_kes(signing_key);

        vectors.push(SingleKesVectorEntry {
            test_name: def.test_name.to_string(),
            seed: def.seed_hex.to_string(),
            message: def.message_hex.to_string(),
            period: 0,
            description: def.description.to_string(),
            expected: SingleKesExpected {
                verification_key: encode_upper(vk_bytes),
                signature: encode_upper(&signature_bytes),
                raw_signature: encode_upper(&signature_bytes),
            },
        });
    }

    Ok(SingleKesVectors {
        description: "SingleKES (Ed25519) deterministic signing vectors",
        algorithm: "SingleKES-Ed25519",
        source: "Generated by cardano-test-vectors/src/bin/generate_kes_vectors.rs",
        vectors,
    })
}

fn build_compact_single_kes_vectors(
    definitions: &[VectorDefinition],
) -> Result<CompactSingleKesVectors, Box<dyn std::error::Error>> {
    let mut vectors = Vec::with_capacity(definitions.len());
    let signature_len = <Ed25519 as DsignAlgorithm>::SIGNATURE_SIZE;

    for def in definitions {
        let seed_bytes = decode_seed(def.seed_hex.as_ref())?;
        let message_bytes = decode_hex(def.message_hex.as_ref())?;

        let signing_key = CompactSingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed_bytes)?;
        let verification_key = CompactSingleKes::<Ed25519>::derive_verification_key(&signing_key)?;
        let signature =
            CompactSingleKes::<Ed25519>::sign_kes(&(), 0, &message_bytes, &signing_key)?;

        let vk_bytes =
            CompactSingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
        let raw_signature = CompactSingleKes::<Ed25519>::raw_serialize_signature_kes(&signature);

        let (dsign_signature, embedded_vk) = raw_signature.split_at(signature_len);

        CompactSingleKes::<Ed25519>::forget_signing_key_kes(signing_key);

        vectors.push(CompactSingleKesVectorEntry {
            test_name: def.test_name.to_string(),
            seed: def.seed_hex.to_string(),
            message: def.message_hex.to_string(),
            period: 0,
            description: def.description.to_string(),
            expected: CompactSingleExpected {
                derived_verification_key: encode_upper(&vk_bytes),
                embedded_verification_key: encode_upper(embedded_vk),
                signature: encode_upper(dsign_signature),
                raw_signature: encode_upper(&raw_signature),
            },
        });
    }

    Ok(CompactSingleKesVectors {
        description: "CompactSingleKES (Ed25519) deterministic signing vectors",
        algorithm: "CompactSingleKES-Ed25519",
        source: "Generated by cardano-test-vectors/src/bin/generate_kes_vectors.rs",
        vectors,
    })
}

fn build_sum_kes_vectors(
    definitions: &[VectorDefinition],
) -> Result<SumKesVectors, Box<dyn std::error::Error>> {
    let hierarchical_defs: Vec<_> = definitions.to_vec();
    let mut levels = Vec::new();

    levels.push(build_hierarchical_level_vectors::<Sum1Kes>(
        1,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<Sum2Kes>(
        2,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<Sum3Kes>(
        3,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<Sum4Kes>(
        4,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<Sum5Kes>(
        5,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<Sum6Kes>(
        6,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<Sum7Kes>(
        7,
        &hierarchical_defs,
    )?);

    Ok(SumKesVectors {
        description: "SumKES hierarchical deterministic vectors",
        algorithm: "SumKES-Ed25519",
        source: "Generated by cardano-test-vectors/src/bin/generate_kes_vectors.rs",
        levels,
    })
}

fn build_compact_sum_kes_vectors(
    definitions: &[VectorDefinition],
) -> Result<CompactSumKesVectors, Box<dyn std::error::Error>> {
    let hierarchical_defs: Vec<_> = definitions.to_vec();
    let mut levels = Vec::new();

    levels.push(build_hierarchical_level_vectors::<CompactSum1Kes>(
        1,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<CompactSum2Kes>(
        2,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<CompactSum3Kes>(
        3,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<CompactSum4Kes>(
        4,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<CompactSum5Kes>(
        5,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<CompactSum6Kes>(
        6,
        &hierarchical_defs,
    )?);
    levels.push(build_hierarchical_level_vectors::<CompactSum7Kes>(
        7,
        &hierarchical_defs,
    )?);

    Ok(CompactSumKesVectors {
        description: "CompactSumKES hierarchical deterministic vectors",
        algorithm: "CompactSumKES-Ed25519",
        source: "Generated by cardano-test-vectors/src/bin/generate_kes_vectors.rs (levels: 1-7)",
        levels,
    })
}

fn build_hierarchical_level_vectors<K>(
    level: u8,
    definitions: &[VectorDefinition],
) -> Result<SumKesLevel, Box<dyn std::error::Error>>
where
    K: KesAlgorithm<Context = ()>,
{
    let total_periods = K::total_periods();
    let tracked_periods = select_periods(total_periods);
    let tracked_set: BTreeSet<u64> = tracked_periods.iter().copied().collect();

    let mut vectors = Vec::with_capacity(definitions.len());

    for (index, def) in definitions.iter().enumerate() {
        let (verification_key, all_periods) = generate_period_entries::<K>(def)?;
        let tracked = all_periods
            .into_iter()
            .filter(|entry| tracked_set.contains(&entry.period))
            .collect();

        vectors.push(SumKesVectorEntry {
            test_name: format!("level{}_kes_vector_{}", level, index + 1),
            seed: def.seed_hex.to_string(),
            description: format!(
                "{} – tracked periods {:?}",
                def.description.as_ref(),
                &tracked_periods
            ),
            verification_key,
            tracked_periods: tracked,
        });
    }

    Ok(SumKesLevel {
        level,
        total_periods,
        vectors,
    })
}

fn build_sum_kes_period_evolution_vectors(
    definitions: &[VectorDefinition],
) -> Result<PeriodEvolutionVectors, Box<dyn std::error::Error>> {
    let hierarchical_defs: Vec<_> = definitions.to_vec();
    let mut levels = Vec::new();

    levels.push(build_period_evolution_level::<Sum1Kes>(
        1,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<Sum2Kes>(
        2,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<Sum3Kes>(
        3,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<Sum4Kes>(
        4,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<Sum5Kes>(
        5,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<Sum6Kes>(
        6,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<Sum7Kes>(
        7,
        &hierarchical_defs,
    )?);

    Ok(PeriodEvolutionVectors {
        description: "SumKES full period evolution sequences",
        algorithm: "SumKES-Ed25519",
        source: "Generated by cardano-test-vectors/src/bin/generate_kes_vectors.rs",
        levels,
    })
}

fn build_compact_sum_kes_period_evolution_vectors(
    definitions: &[VectorDefinition],
) -> Result<PeriodEvolutionVectors, Box<dyn std::error::Error>> {
    let hierarchical_defs: Vec<_> = definitions.to_vec();
    let mut levels = Vec::new();

    levels.push(build_period_evolution_level::<CompactSum1Kes>(
        1,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<CompactSum2Kes>(
        2,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<CompactSum3Kes>(
        3,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<CompactSum4Kes>(
        4,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<CompactSum5Kes>(
        5,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<CompactSum6Kes>(
        6,
        &hierarchical_defs,
    )?);
    levels.push(build_period_evolution_level::<CompactSum7Kes>(
        7,
        &hierarchical_defs,
    )?);

    Ok(PeriodEvolutionVectors {
        description: "CompactSumKES full period evolution sequences",
        algorithm: "CompactSumKES-Ed25519",
        source: "Generated by cardano-test-vectors/src/bin/generate_kes_vectors.rs",
        levels,
    })
}

fn build_period_evolution_level<K>(
    level: u8,
    definitions: &[VectorDefinition],
) -> Result<PeriodEvolutionLevel, Box<dyn std::error::Error>>
where
    K: KesAlgorithm<Context = ()>,
{
    let total_periods = K::total_periods();
    let mut vectors = Vec::with_capacity(definitions.len());

    for (index, def) in definitions.iter().enumerate() {
        let (verification_key, periods) = generate_period_entries::<K>(def)?;
        vectors.push(PeriodEvolutionEntry {
            test_name: format!("level{}_kes_vector_{}", level, index + 1),
            seed: def.seed_hex.to_string(),
            description: format!(
                "{} – full period coverage 0..{}",
                def.description.as_ref(),
                total_periods.saturating_sub(1)
            ),
            verification_key,
            periods,
        });
    }

    Ok(PeriodEvolutionLevel {
        level,
        total_periods,
        vectors,
    })
}

fn generate_period_entries<K>(
    def: &VectorDefinition,
) -> Result<(String, Vec<PeriodVectorEntry>), Box<dyn std::error::Error>>
where
    K: KesAlgorithm<Context = ()>,
{
    let seed_bytes = decode_seed(def.seed_hex.as_ref())?;
    let base_message = decode_hex(def.message_hex.as_ref())?;

    let mut signing_key = K::gen_key_kes_from_seed_bytes(&seed_bytes)?;
    let verification_key = K::derive_verification_key(&signing_key)?;
    let vk_bytes = K::raw_serialize_verification_key_kes(&verification_key);
    let total_periods = K::total_periods();

    let mut periods = Vec::with_capacity(total_periods as usize);

    for period in 0..total_periods {
        let message_bytes = message_for_period(&base_message, period);
        let signature = K::sign_kes(&(), period, &message_bytes, &signing_key)?;
        let raw_signature = K::raw_serialize_signature_kes(&signature);

        K::verify_kes(&(), &verification_key, period, &message_bytes, &signature)?;

        let deserialised =
            K::raw_deserialize_signature_kes(&raw_signature).expect("signature decode");
        K::verify_kes(
            &(),
            &verification_key,
            period,
            &message_bytes,
            &deserialised,
        )?;

        periods.push(PeriodVectorEntry {
            period,
            message: encode_upper(&message_bytes),
            signature: encode_upper(&raw_signature),
            raw_signature: encode_upper(&raw_signature),
        });

        if period + 1 != total_periods {
            signing_key = K::update_kes(&(), signing_key, period)?
                .ok_or_else(|| format!("unexpected key expiry at period {period}"))?;
        }
    }

    K::forget_signing_key_kes(signing_key);

    Ok((encode_upper(&vk_bytes), periods))
}

fn select_periods(total_periods: u64) -> Vec<u64> {
    let mut periods = BTreeSet::new();
    periods.insert(0);

    if total_periods > 1 {
        periods.insert(1);
        periods.insert(total_periods - 1);
    }

    let half = total_periods / 2;
    if half > 0 {
        periods.insert(half);
        if half > 0 {
            periods.insert(half.saturating_sub(1));
        }
    }

    periods.into_iter().collect()
}

fn message_for_period(base: &[u8], period: u64) -> Vec<u8> {
    let mut message = base.to_vec();
    message.extend_from_slice(&period.to_be_bytes());
    message
}

fn decode_seed(hex_seed: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let bytes = decode_hex(hex_seed)?;
    let array: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
        format!(
            "seed '{}' is not 32 bytes ({} bytes)",
            hex_seed,
            bytes.len()
        )
    })?;
    Ok(array)
}

fn decode_hex(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    Ok(hex::decode(input)?)
}

fn write_json<T: Serialize>(path: &PathBuf, value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, json + "\n")?;
    Ok(())
}
