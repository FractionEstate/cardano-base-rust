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
        let Some((key, value)) = trimmed.split_once(':') else {
            panic!("malformed line in {}: {}", name, line);
        };
        fields.insert(key.trim().to_string(), value.trim().to_string());
    }

    let algorithm = fields
        .remove("vrf")
        .unwrap_or_else(|| panic!("missing vrf field in {}", name));
    let version = fields
        .remove("ver")
        .unwrap_or_else(|| panic!("missing ver field in {}", name));
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
        .unwrap_or_else(|| panic!("missing {} field in {}", key, name));
    if value.eq_ignore_ascii_case("empty") {
        Vec::new()
    } else {
        hex::decode(value)
            .unwrap_or_else(|err| panic!("{}: invalid hex for {}: {}", name, key, err))
    }
}

fn alpha_field(fields: &BTreeMap<String, String>, name: &str) -> Vec<u8> {
    let value = fields
        .get("alpha")
        .unwrap_or_else(|| panic!("missing alpha field in {}", name));
    if value.eq_ignore_ascii_case("empty") {
        Vec::new()
    } else {
        hex::decode(value).unwrap_or_else(|err| panic!("{}: invalid hex for alpha: {}", name, err))
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
        .unwrap_or_else(|err| panic!("{}: failed to decode signing key: {}", vector.name, err));
    let verifying_key = PraosVerificationKey::from_bytes(&vector.verifying_key)
        .unwrap_or_else(|err| panic!("{}: failed to decode verifying key: {}", vector.name, err));

    let derived_vk = PraosVRF::derive_verification_key(&signing_key);
    assert_eq!(
        derived_vk.as_bytes(),
        vector.verifying_key.as_slice(),
        "{}: derive_verification_key mismatch",
        vector.name
    );

    let proof = signing_key
        .prove(&vector.message)
        .unwrap_or_else(|err| panic!("{}: prove failed: {}", vector.name, err));
    assert_eq!(
        proof.as_bytes(),
        vector.proof.as_slice(),
        "{}: prove mismatch",
        vector.name
    );

    let proof_from_bytes = PraosProof::from_bytes(&vector.proof)
        .unwrap_or_else(|err| panic!("{}: proof_from_bytes failed: {}", vector.name, err));
    assert_eq!(
        proof_from_bytes.as_bytes(),
        vector.proof.as_slice(),
        "{}: proof_from_bytes roundtrip",
        vector.name
    );

    let proof_output = proof_from_bytes
        .to_output_bytes()
        .unwrap_or_else(|err| panic!("{}: proof_to_hash failed: {}", vector.name, err))
        .expect("proof_to_hash should succeed");
    assert_eq!(
        proof_output, vector.output,
        "{}: proof_to_hash output",
        vector.name
    );

    let verify_output = verifying_key
        .verify(&vector.message, &proof_from_bytes)
        .unwrap_or_else(|err| panic!("{}: verify failed: {}", vector.name, err))
        .expect("verify should succeed");
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
        .unwrap_or_else(|| panic!("{}: VRF verification failed", vector.name));
    assert_eq!(
        verified.as_bytes(),
        vector.output.as_slice(),
        "{}: VRF verification output",
        vector.name
    );
}

fn run_praos_batch_vector(vector: &TestVector) {
    let signing_key_bytes = extend_praos_signing_key(&vector.signing_key, &vector.verifying_key);
    let signing_key =
        PraosBatchCompatSigningKey::from_bytes(&signing_key_bytes).unwrap_or_else(|err| {
            panic!(
                "{}: failed to decode batch signing key: {}",
                vector.name, err
            )
        });
    let verifying_key = PraosBatchCompatVerificationKey::from_bytes(&vector.verifying_key)
        .unwrap_or_else(|err| {
            panic!(
                "{}: failed to decode batch verifying key: {}",
                vector.name, err
            )
        });

    let derived_vk = PraosBatchCompatVRF::derive_verification_key(&signing_key);
    assert_eq!(
        derived_vk.as_bytes(),
        vector.verifying_key.as_slice(),
        "{}: derive_verification_key mismatch",
        vector.name
    );

    let proof = signing_key
        .prove(&vector.message)
        .unwrap_or_else(|err| panic!("{}: prove failed: {}", vector.name, err));
    assert_eq!(
        proof.as_bytes(),
        vector.proof.as_slice(),
        "{}: prove mismatch",
        vector.name
    );

    let proof_from_bytes = PraosBatchCompatProof::from_bytes(&vector.proof)
        .unwrap_or_else(|err| panic!("{}: proof_from_bytes failed: {}", vector.name, err));
    assert_eq!(
        proof_from_bytes.as_bytes(),
        vector.proof.as_slice(),
        "{}: proof_from_bytes roundtrip",
        vector.name
    );

    let proof_output = proof_from_bytes
        .to_output_bytes()
        .unwrap_or_else(|err| panic!("{}: proof_to_hash failed: {}", vector.name, err))
        .expect("proof_to_hash should succeed");
    assert_eq!(
        proof_output, vector.output,
        "{}: proof_to_hash output",
        vector.name
    );

    let verify_output = verifying_key
        .verify(&vector.message, &proof_from_bytes)
        .unwrap_or_else(|err| panic!("{}: verify failed: {}", vector.name, err))
        .expect("verify should succeed");
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
        .unwrap_or_else(|| panic!("{}: VRF verification failed", vector.name));
    assert_eq!(
        verified.as_bytes(),
        vector.output.as_slice(),
        "{}: VRF verification output",
        vector.name
    );
}

fn extend_praos_signing_key(signing_key: &[u8], verifying_key: &[u8]) -> Vec<u8> {
    match signing_key.len() {
        64 => signing_key.to_vec(),
        32 if verifying_key.len() == 32 => {
            let mut extended = Vec::with_capacity(64);
            extended.extend_from_slice(signing_key);
            extended.extend_from_slice(verifying_key);
            extended
        },
        len => panic!("praos signing key has unsupported length {len}; expected 32 or 64 bytes"),
    }
}
