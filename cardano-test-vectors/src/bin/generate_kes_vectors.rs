use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{CompactSingleKes, KesAlgorithm, SingleKes};
use hex::encode_upper;
use serde::Serialize;

#[derive(Debug, Clone)]
struct VectorDefinition {
    test_name: &'static str,
    seed_hex: &'static str,
    message_hex: &'static str,
    description: &'static str,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let definitions = vector_definitions();

    let single_vectors = build_single_kes_vectors(&definitions)?;
    let compact_vectors = build_compact_single_kes_vectors(&definitions)?;

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = manifest_dir.join("test_vectors");

    fs::create_dir_all(&output_dir)?;

    let single_path = output_dir.join("single_kes_test_vectors.json");
    let compact_path = output_dir.join("compact_single_kes_test_vectors.json");

    write_json(&single_path, &single_vectors)?;
    write_json(&compact_path, &compact_vectors)?;

    println!(
        "Generated {} and {}",
        single_path.display(),
        compact_path.display()
    );

    Ok(())
}

fn vector_definitions() -> Vec<VectorDefinition> {
    vec![
        VectorDefinition {
            test_name: "single_kes_vector_1",
            seed_hex: "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
            message_hex: "",
            description: "Zero-message signing with sequential seed bytes",
        },
        VectorDefinition {
            test_name: "single_kes_vector_2",
            seed_hex: "1F1E1D1C1B1A191817161514131211100F0E0D0C0B0A09080706050403020100",
            message_hex: "4B45532053696E676C6520506572696F64",
            description: "ASCII message 'KES Single Period' with reversed seed",
        },
        VectorDefinition {
            test_name: "single_kes_vector_3",
            seed_hex: "B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF",
            message_hex: "243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89",
            description: "Cardano property-test seed with ChaCha-inspired message",
        },
        VectorDefinition {
            test_name: "single_kes_vector_4",
            seed_hex: "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            message_hex: "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            description: "All-0xFF seed and message for upper-bound coverage",
        },
    ]
}

fn build_single_kes_vectors(
    definitions: &[VectorDefinition],
) -> Result<SingleKesVectors, Box<dyn std::error::Error>> {
    let mut vectors = Vec::with_capacity(definitions.len());

    for def in definitions {
        let seed_bytes = decode_seed(def.seed_hex)?;
        let message_bytes = decode_hex(def.message_hex)?;

        let signing_key = SingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed_bytes)?;
        let verification_key = SingleKes::<Ed25519>::derive_verification_key(&signing_key)?;
        let signature = SingleKes::<Ed25519>::sign_kes(&(), 0, &message_bytes, &signing_key)?;

        let vk_bytes = SingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
        let signature_bytes = SingleKes::<Ed25519>::raw_serialize_signature_kes(&signature);

        SingleKes::<Ed25519>::forget_signing_key_kes(signing_key);

        vectors.push(SingleKesVectorEntry {
            test_name: def.test_name.to_owned(),
            seed: def.seed_hex.to_owned(),
            message: def.message_hex.to_owned(),
            period: 0,
            description: def.description.to_owned(),
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
        let seed_bytes = decode_seed(def.seed_hex)?;
        let message_bytes = decode_hex(def.message_hex)?;

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
            test_name: def.test_name.to_owned(),
            seed: def.seed_hex.to_owned(),
            message: def.message_hex.to_owned(),
            period: 0,
            description: def.description.to_owned(),
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
