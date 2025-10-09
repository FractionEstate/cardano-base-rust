use cardano_crypto_class::VRFAlgorithm;
use cardano_crypto_class::vrf::{
    PraosBatchCompatProof, PraosBatchCompatSigningKey, PraosBatchCompatVRF,
    PraosBatchCompatVerificationKey, PraosProof, PraosSigningKey, PraosVRF, PraosVerificationKey,
};
use cardano_test_vectors::vrf::{self, TestVector as RawTestVector};
use std::collections::BTreeMap;

struct TestVector {
    name: String,
    algorithm: String,
    version: String,
    signing_key: Vec<u8>,
    verifying_key: Vec<u8>,
    message: Vec<u8>,
    proof: Vec<u8>,
    output: Vec<u8>,
}

fn load_vectors(prefix: &str) -> Vec<TestVector> {
    let mut entries: Vec<&RawTestVector> = vrf::ALL
        .iter()
        .filter(|vector| vector.name.starts_with(prefix))
        .collect();
    entries.sort_by(|a, b| a.name.cmp(b.name));

    entries
        .into_iter()
        .map(|vector| parse_vector(vector.name, vector.contents))
        .collect()
}

fn parse_vector(name: &str, contents: &str) -> TestVector {
    let mut fields = BTreeMap::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (key, value) = trimmed
            .split_once(':')
            .expect("VRF test vectors lines must contain a ':' separator");
        fields.insert(key.trim().to_string(), value.trim().to_string());
    }

    let algorithm = fields
        .remove("vrf")
        .expect("VRF test vector should include vrf field");
    let version = fields
        .remove("ver")
        .expect("VRF test vector should include ver field");
    let signing_key = hex_field(&fields, "sk", name);
    let verifying_key = hex_field(&fields, "pk", name);
    let message = alpha_field(&fields, name);
    let proof = hex_field(&fields, "pi", name);
    let output = hex_field(&fields, "beta", name);

    TestVector {
        name: name.to_string(),
        algorithm,
        version,
        signing_key,
        verifying_key,
        message,
        proof,
        output,
    }
}

fn hex_field(fields: &BTreeMap<String, String>, key: &str, name: &str) -> Vec<u8> {
    let value = fields
        .get(key)
        .ok_or_else(|| format!("missing {key} field in {name}"))
        .expect("VRF test vector should define required field");
    if value.eq_ignore_ascii_case("empty") {
        Vec::new()
    } else {
        hex::decode(value)
            .map_err(|err| format!("{name}: invalid hex for {key}: {err}"))
            .expect("VRF test vector hex fields must decode")
    }
}

fn alpha_field(fields: &BTreeMap<String, String>, name: &str) -> Vec<u8> {
    let value = fields
        .get("alpha")
        .ok_or_else(|| format!("missing alpha field in {name}"))
        .expect("VRF test vector should define alpha field");
    if value.eq_ignore_ascii_case("empty") {
        Vec::new()
    } else {
        hex::decode(value)
            .map_err(|err| format!("{name}: invalid hex for alpha: {err}"))
            .expect("VRF test vector alpha must decode")
    }
}

#[test]
fn praos_vectors_match_reference() {
    let vectors = load_vectors("vrf_ver03");
    assert!(!vectors.is_empty(), "no ietfdraft03 test vectors found");

    for vector in vectors {
        assert_eq!(
            vector.algorithm, "PraosVRF",
            "{} algorithm mismatch",
            vector.name
        );
        assert_eq!(
            vector.version, "ietfdraft03",
            "{} version mismatch",
            vector.name
        );
        run_praos_vector(&vector);
    }
}

#[test]
fn praos_batch_vectors_match_reference() {
    let vectors = load_vectors("vrf_ver13");
    assert!(!vectors.is_empty(), "no ietfdraft13 test vectors found");

    for vector in vectors {
        assert_eq!(
            vector.algorithm, "PraosBatchCompatVRF",
            "{} algorithm mismatch",
            vector.name
        );
        assert_eq!(
            vector.version, "ietfdraft13",
            "{} version mismatch",
            vector.name
        );
        run_praos_batch_vector(&vector);
    }
}

fn run_praos_vector(vector: &TestVector) {
    let signing_key_bytes = extend_praos_signing_key(&vector.signing_key, &vector.verifying_key);
    let signing_key = PraosSigningKey::from_bytes(&signing_key_bytes)
        .map_err(|err| format!("{}: failed to decode signing key: {err}", vector.name))
        .expect("Praos signing key should decode");
    let verifying_key = PraosVerificationKey::from_bytes(&vector.verifying_key)
        .map_err(|err| format!("{}: failed to decode verifying key: {err}", vector.name))
        .expect("Praos verifying key should decode");

    let derived_vk = PraosVRF::derive_verification_key(&signing_key);
    assert_eq!(
        derived_vk.as_bytes(),
        vector.verifying_key.as_slice(),
        "{}: derive_verification_key mismatch",
        vector.name
    );

    let proof = signing_key
        .prove(&vector.message)
        .map_err(|err| format!("{}: prove failed: {err}", vector.name))
        .expect("Praos proof generation should succeed");
    assert_eq!(
        proof.as_bytes(),
        vector.proof.as_slice(),
        "{}: prove mismatch",
        vector.name
    );

    let proof_from_bytes = PraosProof::from_bytes(&vector.proof)
        .map_err(|err| format!("{}: proof_from_bytes failed: {err}", vector.name))
        .expect("Praos proof decoding should succeed");
    assert_eq!(
        proof_from_bytes.as_bytes(),
        vector.proof.as_slice(),
        "{}: proof_from_bytes roundtrip",
        vector.name
    );

    let proof_output = proof_from_bytes
        .to_output_bytes()
        .map_err(|err| format!("{}: proof_to_hash failed: {err}", vector.name))
        .expect("Praos proof_to_hash should succeed")
        .expect("Praos proof_to_hash should return Some");
    assert_eq!(
        proof_output, vector.output,
        "{}: proof_to_hash output",
        vector.name
    );

    let verify_output = verifying_key
        .verify(&vector.message, &proof_from_bytes)
        .map_err(|err| format!("{}: verify failed: {err}", vector.name))
        .expect("Praos verification should succeed")
        .expect("Praos verification should return Some");
    assert_eq!(
        verify_output, vector.output,
        "{}: verify output",
        vector.name
    );

    let (output_vrf, cert) = PraosVRF::evaluate_bytes(&(), &vector.message, &signing_key);
    assert_eq!(
        output_vrf.as_bytes(),
        vector.output.as_slice(),
        "{}: evaluate output",
        vector.name
    );
    assert_eq!(
        cert.as_bytes(),
        vector.proof.as_slice(),
        "{}: evaluate certificate",
        vector.name
    );

    let verified = PraosVRF::verify_bytes(&(), &verifying_key, &vector.message, &cert)
        .expect("Praos VRF verification should succeed");
    assert_eq!(
        verified.as_bytes(),
        vector.output.as_slice(),
        "{}: VRF verification output",
        vector.name
    );
}

fn run_praos_batch_vector(vector: &TestVector) {
    let signing_key_bytes = extend_praos_signing_key(&vector.signing_key, &vector.verifying_key);
    let signing_key = PraosBatchCompatSigningKey::from_bytes(&signing_key_bytes)
        .map_err(|err| format!("{}: failed to decode batch signing key: {err}", vector.name))
        .expect("Praos batch signing key should decode");
    let verifying_key = PraosBatchCompatVerificationKey::from_bytes(&vector.verifying_key)
        .map_err(|err| {
            format!(
                "{}: failed to decode batch verifying key: {err}",
                vector.name
            )
        })
        .expect("Praos batch verifying key should decode");

    let derived_vk = PraosBatchCompatVRF::derive_verification_key(&signing_key);
    assert_eq!(
        derived_vk.as_bytes(),
        vector.verifying_key.as_slice(),
        "{}: derive_verification_key mismatch",
        vector.name
    );

    let proof = signing_key
        .prove(&vector.message)
        .map_err(|err| format!("{}: prove failed: {err}", vector.name))
        .expect("Praos batch proof generation should succeed");
    assert_eq!(
        proof.as_bytes(),
        vector.proof.as_slice(),
        "{}: prove mismatch",
        vector.name
    );

    let proof_from_bytes = PraosBatchCompatProof::from_bytes(&vector.proof)
        .map_err(|err| format!("{}: proof_from_bytes failed: {err}", vector.name))
        .expect("Praos batch proof decoding should succeed");
    assert_eq!(
        proof_from_bytes.as_bytes(),
        vector.proof.as_slice(),
        "{}: proof_from_bytes roundtrip",
        vector.name
    );

    let proof_output = proof_from_bytes
        .to_output_bytes()
        .map_err(|err| format!("{}: proof_to_hash failed: {err}", vector.name))
        .expect("Praos batch proof_to_hash should succeed")
        .expect("Praos batch proof_to_hash should return Some");
    assert_eq!(
        proof_output, vector.output,
        "{}: proof_to_hash output",
        vector.name
    );

    let verify_output = verifying_key
        .verify(&vector.message, &proof_from_bytes)
        .map_err(|err| format!("{}: verify failed: {err}", vector.name))
        .expect("Praos batch verification should succeed")
        .expect("Praos batch verification should return Some");
    assert_eq!(
        verify_output, vector.output,
        "{}: verify output",
        vector.name
    );

    let (output_vrf, cert) =
        PraosBatchCompatVRF::evaluate_bytes(&(), &vector.message, &signing_key);
    assert_eq!(
        output_vrf.as_bytes(),
        vector.output.as_slice(),
        "{}: evaluate output",
        vector.name
    );
    assert_eq!(
        cert.as_bytes(),
        vector.proof.as_slice(),
        "{}: evaluate certificate",
        vector.name
    );

    let verified = PraosBatchCompatVRF::verify_bytes(&(), &verifying_key, &vector.message, &cert)
        .expect("Praos batch VRF verification should succeed");
    assert_eq!(
        verified.as_bytes(),
        vector.output.as_slice(),
        "{}: VRF verification output",
        vector.name
    );
}

fn extend_praos_signing_key(signing_key: &[u8], verifying_key: &[u8]) -> Vec<u8> {
    assert!(
        signing_key.len() == 64 || (signing_key.len() == 32 && verifying_key.len() == 32),
        "praos signing key has unsupported length {}; expected 32 or 64 bytes",
        signing_key.len()
    );

    if signing_key.len() == 64 {
        signing_key.to_vec()
    } else {
        let mut extended = Vec::with_capacity(64);
        extended.extend_from_slice(signing_key);
        extended.extend_from_slice(verifying_key);
        extended
    }
}
