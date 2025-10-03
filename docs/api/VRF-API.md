# VRF API Reference

Cardano Base provides pure Rust implementations of Verifiable Random Functions (VRFs) following IETF specifications.

## Overview

The VRF implementation is split across two packages:

- **cardano-vrf-pure**: Low-level pure Rust VRF implementation
- **cardano-crypto-class**: High-level VRF API with Praos-specific types

## cardano-vrf-pure

### VrfDraft03

IETF VRF Draft-03 implementation (ECVRF-ED25519-SHA512-ELL2).

```rust
use cardano_vrf_pure::{VrfDraft03, keypair_from_seed_bytes};

// Generate keypair from seed
let seed = [0u8; 32];
let (vk, sk) = keypair_from_seed_bytes::<VrfDraft03>(&seed)?;

// Prove
let message = b"Hello, VRF!";
let proof = sk.prove(message)?;

// Verify
let output = vk.verify(message, &proof)?;
assert!(output.is_some());
```

**Features**:
- Suite ID: 0x04 (ECVRF-ED25519-SHA512-ELL2)
- Proof size: 80 bytes
- Hash-to-curve: Elligator2
- Cofactor multiplication: 8

### VrfDraft13

IETF VRF Draft-13 implementation (ECVRF-ED25519-SHA512-TAI), batch-compatible variant.

```rust
use cardano_vrf_pure::{VrfDraft13, keypair_from_seed_bytes};

// Generate keypair from seed
let seed = [0u8; 32];
let (vk, sk) = keypair_from_seed_bytes::<VrfDraft13>(&seed)?;

// Prove
let message = b"Hello, VRF!";
let proof = sk.prove(message)?;

// Verify
let output = vk.verify(message, &proof)?;
assert!(output.is_some());
```

**Features**:
- Suite ID: 0x04 (ECVRF-ED25519-SHA512-TAI)
- Proof size: 128 bytes
- Hash-to-curve: Try-And-Increment (TAI)
- Cofactor multiplication: 8
- Batch verification support

### Core Types

#### VerKey

Verification key (public key) for VRF operations.

```rust
pub struct VerKey {
    // 32-byte compressed Edwards point
}

impl VerKey {
    pub fn verify(
        &self,
        message: &[u8],
        proof: &Proof,
    ) -> Result<Option<Vec<u8>>, VrfError>;
}
```

#### SignKey

Signing key (private key) for VRF operations.

```rust
pub struct SignKey {
    // 32-byte scalar
}

impl SignKey {
    pub fn prove(&self, message: &[u8]) -> Result<Proof, VrfError>;
}
```

#### Proof

VRF proof structure.

```rust
pub struct Proof {
    // Gamma point (32 bytes)
    // c scalar (32 or 16 bytes depending on variant)
    // s scalar (32 bytes)
}
```

### Key Generation

```rust
use cardano_vrf_pure::{VrfDraft03, keypair_from_seed_bytes};

// From seed (deterministic)
let seed = [0u8; 32];
let (vk, sk) = keypair_from_seed_bytes::<VrfDraft03>(&seed)?;

// From random seed
use rand::RngCore;
let mut seed = [0u8; 32];
rand::thread_rng().fill_bytes(&mut seed);
let (vk, sk) = keypair_from_seed_bytes::<VrfDraft03>(&seed)?;
```

### Error Handling

```rust
pub enum VrfError {
    InvalidProof,
    InvalidKey,
    ProofConstruction,
    DecodingError,
}
```

## cardano-crypto-class

### Praos VRF

High-level API for Praos consensus VRF (Draft-03).

```rust
use cardano_crypto_class::vrf::praos::{PraosVerKey, PraosSignKey};

// Generate from seed
let seed = [0u8; 32];
let sk = PraosSignKey::from_seed(&seed);
let vk = sk.to_ver_key();

// Prove
let message = b"slot_123";
let proof = sk.prove(message)?;

// Verify
let output = vk.verify(message, &proof)?;
assert!(output.is_some());
```

### Batch Praos VRF

Batch-compatible VRF for Praos (Draft-13).

```rust
use cardano_crypto_class::vrf::praos_batch::{
    BatchVerKey, BatchSignKey
};

// Generate from seed
let seed = [0u8; 32];
let sk = BatchSignKey::from_seed(&seed);
let vk = sk.to_ver_key();

// Prove
let message = b"slot_123";
let proof = sk.prove(message)?;

// Verify
let output = vk.verify(message, &proof)?;
assert!(output.is_some());
```

### VRF Output

The VRF output is used for leader election in Praos consensus:

```rust
use cardano_crypto_class::vrf::praos::PraosVerKey;

let message = b"slot_123";
let output = vk.verify(message, &proof)?;

if let Some(hash) = output {
    // hash is 64 bytes (SHA-512 output)
    // Use for leader election
    let leader_value = u64::from_le_bytes(hash[0..8].try_into()?);
    let threshold = calculate_threshold(stake_ratio);

    if leader_value < threshold {
        // This slot is a valid leader slot
    }
}
```

## Serialization

### Raw Bytes

```rust
// Serialize verification key
let vk_bytes = vk.as_bytes(); // 32 bytes

// Serialize signing key
let sk_bytes = sk.as_bytes(); // 32 bytes

// Serialize proof (Draft-03)
let proof_bytes = proof.as_bytes(); // 80 bytes

// Serialize proof (Draft-13)
let proof_bytes = proof.as_bytes(); // 128 bytes

// Deserialize
let vk = VerKey::from_bytes(&vk_bytes)?;
let sk = SignKey::from_bytes(&sk_bytes)?;
let proof = Proof::from_bytes(&proof_bytes)?;
```

### Hex Encoding

```rust
use hex;

// Encode to hex
let vk_hex = hex::encode(vk.as_bytes());
let proof_hex = hex::encode(proof.as_bytes());

// Decode from hex
let vk_bytes = hex::decode(&vk_hex)?;
let vk = VerKey::from_bytes(&vk_bytes)?;
```

## Security Properties

### Correctness

A honestly generated proof always verifies:

```rust
let (vk, sk) = keypair_from_seed_bytes::<VrfDraft03>(&seed)?;
let message = b"test";
let proof = sk.prove(message)?;
let output = vk.verify(message, &proof)?;
assert!(output.is_some());
```

### Soundness

Invalid proofs are rejected:

```rust
let mut proof_bytes = proof.as_bytes().to_vec();
proof_bytes[0] ^= 0x01; // Corrupt proof
let bad_proof = Proof::from_bytes(&proof_bytes)?;
let output = vk.verify(message, &bad_proof)?;
assert!(output.is_none()); // Invalid proof rejected
```

### Uniqueness

Each (key, message) pair produces deterministic output:

```rust
let proof1 = sk.prove(message)?;
let proof2 = sk.prove(message)?;
let output1 = vk.verify(message, &proof1)?;
let output2 = vk.verify(message, &proof2)?;
assert_eq!(output1, output2); // Same output hash
```

### Pseudorandomness

VRF outputs are indistinguishable from random without the proof:

```rust
// Without proof, output appears random
let output = vk.verify(message, &proof)?;
// With proof, output is verifiable
```

## Performance Considerations

### Proof Size

- **Draft-03**: 80 bytes (compact)
- **Draft-13**: 128 bytes (batch-compatible)

Choose Draft-03 for smaller proofs, Draft-13 for batch verification.

### Computation

- **Proving**: ~1-2ms (depends on variant and message size)
- **Verification**: ~2-3ms (includes hash-to-curve)
- **Batch verification**: Amortized cost for multiple proofs (Draft-13 only)

### Memory

All operations use constant memory, no allocations during crypto operations.

## Testing

```bash
# Run VRF internal tests
cargo test --package cardano-vrf-pure

# Run VRF integration tests
cargo test --package cardano-crypto-class --lib vrf

# Run VRF test vectors
cargo test --package cardano-crypto-class --test vrf_praos_vectors
```

## Examples

See `cardano-crypto-class/tests/vrf_praos_vectors.rs` for complete examples with test vectors.

## References

- [IETF VRF Draft-03](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-03)
- [IETF VRF Draft-13](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf-13)
- [Ouroboros Praos Paper](https://eprint.iacr.org/2017/573.pdf)
- [curve25519-dalek Documentation](https://docs.rs/curve25519-dalek/)
