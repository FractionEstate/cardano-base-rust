#!/usr/bin/env bash
# Generate Haskell CBOR reference values for cross-compatibility testing
#
# This script provides a template for generating Haskell cardano-base CBOR
# values that match our Rust test vectors. It needs to be run in a Haskell
# environment with cardano-base installed.
#
# Prerequisites:
#   - Haskell Stack or Cabal installed
#   - cardano-base repository cloned and built
#   - jq (for JSON processing)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VECTORS_DIR="${SCRIPT_DIR}/tests/test_vectors"

echo "=== Cardano Crypto Haskell Reference Value Generator ==="
echo ""
echo "This script helps generate Haskell CBOR reference values for"
echo "cross-compatibility testing with our Rust implementation."
echo ""

# Check if we're in a dev container or can access Haskell
if ! command -v stack &> /dev/null && ! command -v cabal &> /dev/null; then
    echo "❌ Error: Neither 'stack' nor 'cabal' found in PATH"
    echo ""
    echo "To install Haskell Stack:"
    echo "  curl -sSL https://get.haskellstack.org/ | sh"
    echo ""
    echo "Or visit: https://www.haskell.org/get-started/"
    exit 1
fi

echo "✅ Haskell toolchain found"
echo ""

# Check if jq is available
if ! command -v jq &> /dev/null; then
    echo "⚠️  Warning: 'jq' not found. JSON processing will be manual."
    echo "   Install jq: apt-get install jq (Debian/Ubuntu)"
    HAS_JQ=false
else
    echo "✅ jq found"
    HAS_JQ=true
fi
echo ""

# Function to display test vector info
show_test_vector() {
    local file="$1"
    local name="$2"

    if [ "$HAS_JQ" = true ]; then
        echo "Test Vector: $name"
        jq -r ".vectors[] | select(.name == \"$name\") |
               \"  Seed: \\(.seed)\\n  Message: \\(.message)\"" "$file"
    else
        echo "Test Vector: $name (see $file)"
    fi
}

# Main menu
echo "Select an algorithm to generate Haskell reference values:"
echo ""
echo "1) Ed25519 DSIGN (5 test vectors)"
echo "2) Praos VRF (5 test vectors)"
echo "3) Simple VRF (5 test vectors)"
echo "4) Mock VRF (5 test vectors)"
echo "5) Single KES (5 test vectors)"
echo "6) Compact Single KES (5 test vectors)"
echo "7) All algorithms (30 test vectors)"
echo "8) Show Haskell code template"
echo "9) Exit"
echo ""
read -p "Enter choice [1-9]: " choice

case $choice in
    1)
        echo ""
        echo "=== Ed25519 DSIGN Test Vectors ==="
        echo ""
        if [ -f "${VECTORS_DIR}/ed25519_vectors.json" ]; then
            if [ "$HAS_JQ" = true ]; then
                echo "Test vectors found:"
                jq -r '.vectors[] | "  - \(.name): seed=\(.seed[0:16])..., message=\(.message)"' \
                    "${VECTORS_DIR}/ed25519_vectors.json"
            else
                cat "${VECTORS_DIR}/ed25519_vectors.json"
            fi
        else
            echo "❌ File not found: ${VECTORS_DIR}/ed25519_vectors.json"
        fi
        ;;

    8)
        echo ""
        echo "=== Haskell Code Template ==="
        echo ""
        cat << 'EOF'
-- HaskellReferenceGenerator.hs
-- Generate CBOR reference values for Rust cross-compatibility testing

{-# LANGUAGE DataKinds #-}
{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE ScopedTypeVariables #-}

module Main where

import qualified Data.ByteString.Base16 as B16
import qualified Data.ByteString as BS
import qualified Data.ByteString.Lazy as BSL
import Data.Aeson (encode, object, (.=))
import Cardano.Crypto.DSIGN.Ed25519 (Ed25519DSIGN)
import Cardano.Binary (serialize')
import qualified Cardano.Crypto.DSIGN as DSIGN

-- | Convert hex string to ByteString
fromHex :: String -> BS.ByteString
fromHex = fst . B16.decode . BS.pack . map (fromIntegral . fromEnum)

-- | Generate Ed25519 CBOR values
generateEd25519 :: String -> String -> IO ()
generateEd25519 seedHex msgHex = do
  let seed = fromHex seedHex
      msg = fromHex msgHex
      sk = DSIGN.genKeyDSIGN @Ed25519DSIGN seed
      vk = DSIGN.deriveVerKeyDSIGN sk
      sig = DSIGN.signDSIGN () msg sk

  putStrLn $ "  VK CBOR:  " ++ show (B16.encode $ serialize' vk)
  putStrLn $ "  Sig CBOR: " ++ show (B16.encode $ serialize' sig)

main :: IO ()
main = do
  putStrLn "=== Haskell Reference CBOR Values ==="
  putStrLn ""

  -- Example: all_zeros_seed test vector
  putStrLn "Test: all_zeros_seed"
  generateEd25519
    "0000000000000000000000000000000000000000000000000000000000000000"
    "48656c6c6f2c20576f726c6421"  -- "Hello, World!"

  putStrLn ""

  -- Add more test vectors here...

  putStrLn ""
  putStrLn "To use these values:"
  putStrLn "1. Copy the CBOR hex strings"
  putStrLn "2. Add to corresponding JSON files as 'haskell_cbor' fields"
  putStrLn "3. Run Rust tests to compare"

-- Build and run:
--   stack exec runghc HaskellReferenceGenerator.hs
-- or:
--   cabal run HaskellReferenceGenerator
EOF
        echo ""
        echo "To use this template:"
        echo "1. Save as HaskellReferenceGenerator.hs in cardano-base project"
        echo "2. Update imports based on your cardano-base version"
        echo "3. Add all test vectors from our JSON files"
        echo "4. Build and run to generate Haskell CBOR values"
        echo "5. Compare with our Rust values"
        ;;

    9)
        echo "Goodbye!"
        exit 0
        ;;

    *)
        echo "Invalid choice. Please run again."
        exit 1
        ;;
esac

echo ""
echo "=== Next Steps ==="
echo ""
echo "Option A: Use Haskell Template"
echo "  1. Copy the Haskell template (option 8)"
echo "  2. Setup Haskell environment with cardano-base"
echo "  3. Run generator to get CBOR values"
echo "  4. Add values to JSON test vector files"
echo ""
echo "Option B: Contact Haskell Team"
echo "  1. File issue at: https://github.com/IntersectMBO/cardano-base"
echo "  2. Share our test vector JSON files"
echo "  3. Request corresponding Haskell CBOR values"
echo "  4. Receive and integrate values"
echo ""
echo "For more details, see: PHASE3_HASKELL_INTEGRATION_GUIDE.md"
echo ""
