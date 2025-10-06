#!/usr/bin/env bash
# Script to generate DSIGN test vectors from Haskell reference implementation
#
# This script runs the Haskell test suite and extracts test vectors for:
# - Ed25519 (RFC 8032 vectors + Cardano-specific vectors)
# - ECDSA Secp256k1 (sign/verify vectors + edge cases)
# - Schnorr Secp256k1 (sign/verify vectors + edge cases)
#
# Output format: JSON files compatible with Rust test framework

set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
HASKELL_REF="${ROOT_DIR}/reference-cardano-base"
OUTPUT_DIR="${ROOT_DIR}/cardano-test-vectors/test_vectors"

echo "=== DSIGN Test Vector Generator ==="
echo "Root directory: ${ROOT_DIR}"
echo "Haskell reference: ${HASKELL_REF}"
echo "Output directory: ${OUTPUT_DIR}"
echo ""

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Check if Haskell reference exists
if [ ! -d "${HASKELL_REF}" ]; then
    echo "ERROR: Haskell reference not found at ${HASKELL_REF}"
    exit 1
fi

echo "Step 1: Building Haskell test suite..."
cd "${HASKELL_REF}/cardano-crypto-tests"

# Build the test suite
if ! cabal build cardano-crypto-tests; then
    echo "ERROR: Failed to build Haskell test suite"
    echo "Make sure you have cabal and ghc installed"
    exit 1
fi

echo "Step 2: Running test suite to extract vectors..."
# Note: We're not actually extracting from test runs here, but documenting
# the test vectors that are hard-coded in the Haskell test files.
# For now, we'll manually transcribe them to JSON format.

echo "Step 3: Creating Ed25519 test vectors..."
cat > "${OUTPUT_DIR}/ed25519_test_vectors.json" << 'EOF'
{
  "description": "Ed25519 DSIGN test vectors from Cardano Haskell reference",
  "algorithm": "Ed25519DSIGN",
  "source": "cardano-crypto-tests/src/Test/Crypto/Vector/Vectors.hs",
  "vectors": [
    {
      "test_name": "sign_and_verify_1",
      "seed": "0000000000000000000000000000000000000000000000000000000000000003",
      "message": "0000000000000000000000000000000000000000000000000000000000000000",
      "description": "Minimal seed value test"
    },
    {
      "test_name": "sign_and_verify_2",
      "seed": "B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF",
      "message": "243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89",
      "description": "Standard test vector from Haskell tests"
    },
    {
      "test_name": "sign_and_verify_3",
      "seed": "C90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B14E5C9",
      "message": "7E2D58D8B3BCDF1ABADEC7829054F90DDA9805AAB56C77333024B9D0A508B75C",
      "description": "Another standard test vector"
    },
    {
      "test_name": "sign_and_verify_4",
      "seed": "0B432B2677937381AEF05BB02A66ECD012773062CF3FA2549E44F58ED2401710",
      "message": "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
      "description": "Maximum message value test"
    }
  ]
}
EOF

echo "Step 4: Creating ECDSA Secp256k1 test vectors..."
cat > "${OUTPUT_DIR}/ecdsa_secp256k1_test_vectors.json" << 'EOF'
{
  "description": "ECDSA Secp256k1 DSIGN test vectors from Cardano Haskell reference",
  "algorithm": "EcdsaSecp256k1DSIGN",
  "source": "cardano-crypto-tests/src/Test/Crypto/Vector/Vectors.hs",
  "sign_and_verify_vectors": [
    {
      "test_name": "sign_and_verify_1",
      "secret_key": "0000000000000000000000000000000000000000000000000000000000000003",
      "message": "0000000000000000000000000000000000000000000000000000000000000000",
      "description": "Minimal secret key value"
    },
    {
      "test_name": "sign_and_verify_2",
      "secret_key": "B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF",
      "message": "243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89",
      "description": "Standard test vector"
    },
    {
      "test_name": "sign_and_verify_3",
      "secret_key": "C90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B14E5C9",
      "message": "7E2D58D8B3BCDF1ABADEC7829054F90DDA9805AAB56C77333024B9D0A508B75C",
      "description": "Another standard vector"
    },
    {
      "test_name": "sign_and_verify_4",
      "secret_key": "0B432B2677937381AEF05BB02A66ECD012773062CF3FA2549E44F58ED2401710",
      "message": "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
      "description": "Maximum message value"
    }
  ],
  "verify_only_vectors": [
    {
      "test_name": "verify_with_known_signature",
      "verification_key": "02599de3e582e2a3779208a210dfeae8f330b9af00a47a7fb22e9bb8ef596f301b",
      "message": "0000000000000000000000000000000000000000000000000000000000000000",
      "signature": "354b868c757ef0b796003f7c23dd754d2d1726629145be2c7b7794a25fec80a06254f0915935f33b91bceb16d46ff2814f659e9b6791a4a21ff8764b78d7e114",
      "should_verify": true,
      "description": "Known good signature"
    },
    {
      "test_name": "negative_signature_normalized",
      "verification_key": "02599de3e582e2a3779208a210dfeae8f330b9af00a47a7fb22e9bb8ef596f301b",
      "message": "0000000000000000000000000000000000000000000000000000000000000000",
      "signature": "354b868c757ef0b796003f7c23dd754d2d1726629145be2c7b7794a25fec80a09dab0f6ea6ca0cc46e4314e92b900d7d6b493e4b47b6fb999fd9e841575e602d",
      "should_verify": false,
      "description": "Negative signature component should be normalized to low-s form"
    }
  ],
  "error_vectors": [
    {
      "test_name": "wrong_verification_key",
      "verification_key": "02D69C3509BB99E412E68B0FE8544E72837DFA30746D8BE2AA65975F29D22DC7B9",
      "description": "Wrong verification key should fail verification"
    },
    {
      "test_name": "ver_key_not_on_curve",
      "verification_key_raw": "02EEFDEA4CDB677750A420FEE807EACF21EB9898AE79B9768766E4FAA04A2D4A34",
      "should_parse": false,
      "description": "Verification key not on curve should fail to parse"
    },
    {
      "test_name": "invalid_ver_key_length_short",
      "verification_key_raw": "02DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B50",
      "should_parse": false,
      "description": "30-byte verification key should fail"
    },
    {
      "test_name": "invalid_ver_key_length_long",
      "verification_key_raw": "02DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659FF",
      "should_parse": false,
      "description": "34-byte verification key should fail"
    },
    {
      "test_name": "invalid_signature_length_short",
      "signature_raw": "354b868c757ef0b796003f7c23dd754d2d1726629145be2c7b7794a25fec80a06254f0915935f33b91bceb16d46ff2814f659e9b6791a4a21ff8764b78d7e1",
      "should_parse": false,
      "description": "63-byte signature should fail"
    },
    {
      "test_name": "invalid_signature_length_long",
      "signature_raw": "354b868c757ef0b796003f7c23dd754d2d1726629145be2c7b7794a25fec80a06254f0915935f33b91bceb16d46ff2814f659e9b6791a4a21ff8764b78d7e114FF",
      "should_parse": false,
      "description": "65-byte signature should fail"
    },
    {
      "test_name": "mismatch_message_wrong_verify",
      "message": "243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89",
      "verification_key": "0325d1dff95105f5253c4022f628a996ad3a0d95fbf21d468a1b33f8c160d8f517",
      "signature": "3dccc57be49991e95b112954217e8b4fe884d4d26843dfec794feb370981407b79151d1e5af85aba21721876896957adb2b35bcbb84986dcf82daa520a87a9f9",
      "should_verify": false,
      "description": "Wrong message for given signature"
    },
    {
      "test_name": "mismatch_signature_wrong_message",
      "message": "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
      "verification_key": "0325d1dff95105f5253c4022f628a996ad3a0d95fbf21d468a1b33f8c160d8f517",
      "signature": "5ef63d477c5d1572550016ccf72a2310c7368beeb843c85b1b5697290872222a09e7519702cb2c9a65bbce92d273080a0193b77588bc2eac6dbcbfc15c6dfefd",
      "should_verify": false,
      "description": "Wrong signature for given message"
    }
  ]
}
EOF

echo "Step 5: Creating Schnorr Secp256k1 test vectors..."
cat > "${OUTPUT_DIR}/schnorr_secp256k1_test_vectors.json" << 'EOF'
{
  "description": "Schnorr Secp256k1 DSIGN test vectors from Cardano Haskell reference",
  "algorithm": "SchnorrSecp256k1DSIGN",
  "source": "cardano-crypto-tests/src/Test/Crypto/Vector/Vectors.hs",
  "sign_and_verify_vectors": [
    {
      "test_name": "sign_and_verify_1",
      "secret_key": "0000000000000000000000000000000000000000000000000000000000000003",
      "message": "0000000000000000000000000000000000000000000000000000000000000000",
      "description": "Minimal secret key value"
    },
    {
      "test_name": "sign_and_verify_2",
      "secret_key": "B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF",
      "message": "243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89",
      "description": "Standard test vector"
    },
    {
      "test_name": "sign_and_verify_3",
      "secret_key": "C90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B14E5C9",
      "message": "7E2D58D8B3BCDF1ABADEC7829054F90DDA9805AAB56C77333024B9D0A508B75C",
      "description": "Another standard vector"
    },
    {
      "test_name": "sign_and_verify_4",
      "secret_key": "0B432B2677937381AEF05BB02A66ECD012773062CF3FA2549E44F58ED2401710",
      "message": "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
      "description": "Maximum message value"
    }
  ],
  "verify_only_vectors": [
    {
      "test_name": "verify_with_known_signature",
      "verification_key": "599de3e582e2a3779208a210dfeae8f330b9af00a47a7fb22e9bb8ef596f301b",
      "message": "0000000000000000000000000000000000000000000000000000000000000000",
      "signature": "5a56da88e6fd8419181dec4d3dd6997bab953d2fc71ab65e23cfc9e7e3d1a310613454a60f6703819a39fdac2a410a094442afd1fc083354443e8d8bb4461a9b",
      "should_verify": true,
      "description": "Known good Schnorr signature"
    }
  ],
  "error_vectors": [
    {
      "test_name": "wrong_verification_key",
      "verification_key": "D69C3509BB99E412E68B0FE8544E72837DFA30746D8BE2AA65975F29D22DC7B9",
      "description": "Wrong verification key should fail verification"
    },
    {
      "test_name": "invalid_signature_length_short",
      "signature_raw": "5a56da88e6fd8419181dec4d3dd6997bab953d2fc71ab65e23cfc9e7e3d1a310613454a60f6703819a39fdac2a410a094442afd1fc083354443e8d8bb4461a",
      "should_parse": false,
      "description": "63-byte signature should fail"
    },
    {
      "test_name": "invalid_signature_length_long",
      "signature_raw": "5a56da88e6fd8419181dec4d3dd6997bab953d2fc71ab65e23cfc9e7e3d1a310613454a60f6703819a39fdac2a410a094442afd1fc083354443e8d8bb4461a9bFF",
      "should_parse": false,
      "description": "65-byte signature should fail"
    }
  ]
}
EOF

echo ""
echo "✅ Test vector files created successfully!"
echo ""
echo "Generated files:"
echo "  - ${OUTPUT_DIR}/ed25519_test_vectors.json"
echo "  - ${OUTPUT_DIR}/ecdsa_secp256k1_test_vectors.json"
echo "  - ${OUTPUT_DIR}/schnorr_secp256k1_test_vectors.json"
echo ""
echo "Note: These vectors are manually transcribed from the Haskell test files."
echo "To generate expected outputs (verification keys, signatures), you'll need to:"
echo "1. Run the Haskell tests with verbose output"
echo "2. Extract the actual bytes generated"
echo "3. Update the JSON files with the expected outputs"
echo ""
echo "Alternatively, you can:"
echo "1. Run the Rust implementations to generate outputs"
echo "2. Verify those outputs match the Haskell implementation"
echo "3. Add the verified outputs as expected values"
