# CBOR Cross-Compatibility Test Vectors

This directory contains test vectors for validating that Rust CBOR serialization matches the Haskell `cardano-base` implementation byte-for-byte.

## Test Vector Format

Test vectors are stored as JSON files with the following structure:

```json
{
  "description": "Human-readable description",
  "algorithm": "Ed25519|PraosVRF|SimpleVRF|MockVRF|SingleKes|CompactSingleKes",
  "vectors": [
    {
      "name": "test case name",
      "seed": "hex-encoded seed bytes",
      "message": "hex-encoded message (for signatures/proofs)",
      "expected_vk_cbor": "hex-encoded CBOR bytes for verification key",
      "expected_sig_cbor": "hex-encoded CBOR bytes for signature/proof",
      "notes": "optional notes"
    }
  ]
}
```

## Generating Test Vectors

### From Haskell cardano-base

To generate compatible test vectors from Haskell:

```haskell
-- Example for Ed25519
import Cardano.Crypto.DSIGN.Ed25519
import Codec.Serialise (serialise)
import qualified Data.ByteString.Lazy as LBS
import qualified Data.ByteString.Base16 as B16

-- Generate key from seed
let seed = mkSeedFromBytes (replicate 32 42)
let sk = genKey seed
let vk = deriveVerKey sk

-- Serialize to CBOR and hex encode
let vkCBOR = LBS.toStrict $ serialise vk
putStrLn $ B16.encode vkCBOR
```

### Manual Test Vectors

For initial testing, we can create known-good test vectors manually by:

1. Using deterministic seeds
2. Generating keys/signatures in Rust
3. Manually verifying the CBOR structure
4. Later validating against Haskell output

## Test Vector Files

- `ed25519_vectors.json` - Ed25519 DSIGN test vectors
- `praos_vrf_vectors.json` - Praos VRF test vectors
- `simple_vrf_vectors.json` - Simple VRF test vectors
- `mock_vrf_vectors.json` - Mock VRF test vectors
- `single_kes_vectors.json` - Single KES test vectors
- `compact_single_kes_vectors.json` - Compact Single KES test vectors
- `compact_sum_kes_test_vectors.json` - Compact Sum KES (levels 1–7) test vectors

## Validation Process

The cross-compatibility test suite will:

1. Load test vectors from JSON
2. Generate keys/signatures from the provided seeds
3. Serialize to CBOR
4. Compare hex-encoded output byte-for-byte with expected values
5. Report any mismatches with detailed diff information

## Adding New Test Vectors

To add new test vectors:

1. Choose deterministic seed values
2. Generate in both Rust and Haskell
3. Verify CBOR structure manually
4. Add to appropriate JSON file
5. Run cross-compatibility tests

## CBOR Structure Reference

### Ed25519 Verification Key

- CBOR Type: bytes (major type 2)
- Length: 32 bytes
- Encoding: `58 20` (tag + length) + 32 bytes data
- Total: 34 bytes

### Ed25519 Signature

- CBOR Type: bytes (major type 2)
- Length: 64 bytes
- Encoding: `58 40` (tag + length) + 64 bytes data
- Total: 66 bytes

### Praos VRF Verification Key

- CBOR Type: bytes
- Length: 32 bytes (compressed Ristretto255 point)
- Total: 34 bytes

### Praos VRF Proof

- CBOR Type: bytes
- Length: 80 bytes (π, c, s components)
- Total: 82 bytes

### CompactSingle KES Signature

- CBOR Type: array (major type 4)
- Elements: [signature, verification_key]
- Structure: tuple of two byte strings
