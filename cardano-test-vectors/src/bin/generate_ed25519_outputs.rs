use std::error::Error;
use std::fs;
use std::path::PathBuf;

use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use hex::encode_upper;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestVectorFile {
    description: Option<String>,
    algorithm: String,
    source: Option<String>,
    vectors: Vec<TestVector>,
}

#[derive(Debug, Deserialize)]
struct TestVector {
    #[serde(rename = "test_name")]
    test_name: String,
    seed: String,
    message: String,
    #[serde(rename = "expected_public_key")]
    expected_public_key: Option<String>,
    #[serde(rename = "expected_signature")]
    expected_signature: Option<String>,
    description: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Ed25519 DSIGN Reference Output Generator (Rust) ===\n");

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let vector_path = manifest_dir.join("test_vectors/ed25519_test_vectors.json");

    let data = fs::read(&vector_path)?;
    let file: TestVectorFile = serde_json::from_slice(&data)?;

    if file.algorithm.to_ascii_lowercase() != "ed25519" {
        return Err(format!(
            "unexpected algorithm '{}' in {}; expected 'ed25519'",
            file.algorithm,
            vector_path.display()
        )
        .into());
    }

    println!("Algorithm: {}", file.algorithm);
    println!("Vectors: {}", file.vectors.len());
    if let Some(description) = file.description.as_deref() {
        println!("Description: {}", description);
    }
    if let Some(source) = file.source.as_deref() {
        println!("Source: {}", source);
    }
    println!();

    for vector in &file.vectors {
        println!("Processing: {}", vector.test_name);
        if let Some(desc) = vector.description.as_deref() {
            println!("  {}", desc);
        }

        let seed_bytes = decode_hex(&vector.seed)?;
        let message_bytes = decode_hex(&vector.message)?;

        if seed_bytes.len() != <Ed25519 as DsignAlgorithm>::SEED_SIZE {
            return Err(format!(
                "seed '{}' is not {} bytes ({} bytes)",
                vector.seed,
                <Ed25519 as DsignAlgorithm>::SEED_SIZE,
                seed_bytes.len()
            )
            .into());
        }

        let signing_key = <Ed25519 as DsignAlgorithm>::gen_key_from_seed_bytes(&seed_bytes);
        let verification_key = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing_key);
        let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), &message_bytes, &signing_key);

        let vk_bytes =
            <Ed25519 as DsignAlgorithm>::raw_serialize_verification_key(&verification_key);
        let sig_bytes = <Ed25519 as DsignAlgorithm>::raw_serialize_signature(&signature);

        let vk_hex = encode_upper(&vk_bytes);
        let sig_hex = encode_upper(&sig_bytes);

        println!("  Seed:             {}", vector.seed);
        println!("  Message:          {}", vector.message);
        println!("  Verification Key: {}", vk_hex);
        println!("  Signature:        {}", sig_hex);

        if let Some(expected) = vector.expected_public_key.as_deref() {
            let expected_upper = expected.to_ascii_uppercase();
            if expected_upper == vk_hex {
                println!("  ✅ Verification key matches existing expected value");
            } else {
                println!(
                    "  ⚠️  Verification key differs! expected {}, got {}",
                    expected, vk_hex
                );
            }
        } else {
            println!("  ℹ️ No expected verification key stored in fixture");
        }

        if let Some(expected) = vector.expected_signature.as_deref() {
            let expected_upper = expected.to_ascii_uppercase();
            if expected_upper == sig_hex {
                println!("  ✅ Signature matches existing expected value");
            } else {
                println!(
                    "  ⚠️  Signature differs! expected {}, got {}",
                    expected, sig_hex
                );
            }
        } else {
            println!("  ℹ️ No expected signature stored in fixture");
        }

        match <Ed25519 as DsignAlgorithm>::verify_bytes(
            &(),
            &verification_key,
            &message_bytes,
            &signature,
        ) {
            Ok(()) => println!("  ✅ Verification succeeded"),
            Err(err) => println!("  ❌ Verification failed: {:?}", err),
        }

        println!();
    }

    println!("=== Generation Complete ===");
    println!();
    println!("Outputs above can be pasted into test_vectors/ed25519_test_vectors.json as needed.");

    Ok(())
}

fn decode_hex(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    Ok(hex::decode(input)?)
}
