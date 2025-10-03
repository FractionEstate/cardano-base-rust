#!/bin/bash
# Regenerate VRF test vectors using pure Rust implementation

cd "$(dirname "$0")"

cat > /tmp/regen_vectors.rs << 'EOF'
use cardano_crypto_class::vrf::{
    PraosBatchCompatSigningKey, PraosBatchCompatVRF,
    PraosSigningKey, PraosVRF,
};
use cardano_crypto_class::VRFAlgorithm;

fn main() {
    println!("Regenerating VRF test vectors using pure Rust implementation...\n");
    
    // Test with one vector to see output
    let seed = hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let sk = PraosSigningKey::from_seed(&seed);
    let vk = PraosVRF::derive_verification_key(&sk);
    let alpha = hex::decode("00").unwrap();
    let proof = PraosVRF::prove(&sk, &alpha).unwrap();
    let beta = PraosVRF::proof_to_output(&vk, &proof).unwrap();
    
    println!("Sample Draft-03 output:");
    println!("sk: {}", hex::encode(&seed));
    println!("pk: {}", hex::encode(vk.as_bytes()));
    println!("alpha: {}", hex::encode(&alpha));
    println!("pi: {}", hex::encode(&proof.as_bytes()));
    println!("beta: {}", hex::encode(&beta.as_bytes()));
}
EOF

RUSTFLAGS="" cargo run --manifest-path cardano-crypto-class/Cargo.toml --example regen_test 2>/dev/null || {
    echo "Creating inline test..."
    cargo test --package cardano-crypto-class print_rust_proof -- --nocapture --ignored 2>&1 | tail -20
}
