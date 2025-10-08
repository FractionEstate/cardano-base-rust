{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE RecordWildCards #-}
{-# LANGUAGE TypeApplications #-}

-- | Recompute the hash vector corpus using the Haskell `cardano-crypto-class`
-- implementations so we can cross-check the Rust outputs.
--
-- Build prerequisites (inside the cloned Haskell repository):
--   cabal build cardano-crypto-class -f-secp256k1-support
--
-- Run from the Haskell repo root:
--   cabal exec -- runghc /path/to/generate_hash_vectors_haskell.hs \
--       /absolute/path/to/hash_test_vectors.json \
--       /absolute/path/to/hash_test_vectors_haskell.json
--
-- The script reads the Rust-authored vector definitions (names, descriptions,
-- and input payloads) and re-computes every digest using the Haskell lib. The
-- resulting JSON is formatted identically to the committed corpus so the
-- comparator can diff them directly.

import Cardano.Crypto.Hash
import Cardano.Crypto.Hash.Blake2b (blake2b_libsodium)
import Cardano.Crypto.Init (cryptoInit)
import Cardano.Crypto.Util (decodeHexString)

import Control.Monad (forM)
import Data.Aeson (FromJSON (..), ToJSON (..), Value (Object), (.!=), (.:), (.:?))
import qualified Data.Aeson as Aeson
import qualified Data.Aeson.Key as Key
import qualified Data.Aeson.KeyMap as KeyMap
import Data.ByteString (ByteString)
import qualified Data.ByteString as BS
import qualified Data.ByteString.Base16 as Base16
import qualified Data.ByteString.Lazy as BSL
import Data.Maybe (catMaybes)
import Data.Semigroup ((<>))
import Data.Text (Text)
import qualified Data.Text as Text
import qualified Data.Text.Encoding as Text
import GHC.Generics (Generic)
import System.Environment (getArgs)
import System.Exit (die)

--------------------------------------------------------------------------------
-- JSON payloads
--------------------------------------------------------------------------------

data Metadata = Metadata
  { metaDescription :: Maybe Text
  , metaVersion :: Maybe Int
  , metaNote :: Maybe Text
  , metaGenerator :: Maybe Text
  }
  deriving (Show, Generic)

data InputVector = InputVector
  { inName :: Text
  , inDescription :: Maybe Text
  , inInputHex :: Text
  }
  deriving (Show, Generic)

data OutputVector = OutputVector
  { outName :: Text
  , outDescription :: Maybe Text
  , outInputHex :: Text
  , outSha256 :: Text
  , outSha256d :: Text
  , outSha512 :: Text
  , outSha3_256 :: Text
  , outSha3_512 :: Text
  , outKeccak256 :: Text
  , outRipemd160 :: Text
  , outHash160 :: Text
  , outBlake2b224 :: Text
  , outBlake2b256 :: Text
  , outBlake2b512 :: Text
  }
  deriving (Show, Generic)

data HashVectorsIn = HashVectorsIn
  { hvMetadata :: Metadata
  , hvVectors :: [InputVector]
  }
  deriving (Show, Generic)

data HashVectorsOut = HashVectorsOut
  { hvOutMetadata :: Metadata
  , hvOutVectors :: [OutputVector]
  }
  deriving (Show, Generic)

--------------------------------------------------------------------------------
-- JSON instances (manual ordering preservation)
--------------------------------------------------------------------------------

instance FromJSON Metadata where
  parseJSON = Aeson.withObject "Metadata" $ \o ->
    Metadata
      <$> o .:? "description"
      <*> o .:? "version"
      <*> o .:? "note"
      <*> o .:? "generator"

instance ToJSON Metadata where
  toJSON Metadata{..} =
    Object . KeyMap.fromList $ catMaybes
      [ opt "description" metaDescription
      , opt "version" metaVersion
      , opt "note" metaNote
      , opt "generator" metaGenerator
      ]

instance FromJSON InputVector where
  parseJSON = Aeson.withObject "HashVector" $ \o ->
    InputVector
      <$> o .: "name"
      <*> o .:? "description"
      <*> o .: "input_hex"

instance ToJSON OutputVector where
  toJSON OutputVector{..} =
    Object . KeyMap.fromList $
      [ req "name" outName
      ]
        <> maybe [] ((:[]) . req "description") outDescription
        <> [ req "input_hex" outInputHex
           , req "sha256" outSha256
           , req "sha256d" outSha256d
           , req "sha512" outSha512
           , req "sha3_256" outSha3_256
           , req "sha3_512" outSha3_512
           , req "keccak256" outKeccak256
           , req "ripemd160" outRipemd160
           , req "hash160" outHash160
           , req "blake2b224" outBlake2b224
           , req "blake2b256" outBlake2b256
           , req "blake2b512" outBlake2b512
           ]

instance FromJSON HashVectorsIn where
  parseJSON = Aeson.withObject "HashVectors" $ \o ->
    HashVectorsIn
      <$> o .:? "metadata" .!= Metadata Nothing Nothing Nothing Nothing
      <*> o .: "vectors"

instance ToJSON HashVectorsOut where
  toJSON HashVectorsOut{..} =
    Object . KeyMap.fromList $
      [ req "metadata" hvOutMetadata
      , req "vectors" hvOutVectors
      ]

req :: ToJSON a => Text -> a -> (Aeson.Key, Value)
req name value = (Key.fromText name, toJSON value)

opt :: ToJSON a => Text -> Maybe a -> Maybe (Aeson.Key, Value)
opt name = fmap (req name)

--------------------------------------------------------------------------------
-- Hashing helpers
--------------------------------------------------------------------------------

hashRaw :: forall h. HashAlgorithm h => ByteString -> ByteString
hashRaw bytes = hashToBytes (hashWith @h id bytes)

hashHex :: forall h. HashAlgorithm h => ByteString -> Text
hashHex = toHexText . hashRaw @h

hashBytesHex :: ByteString -> Text
hashBytesHex = toHexText

hash160HexOf :: ByteString -> Text
hash160HexOf bytes =
  let sha256Once = hashRaw @SHA256 bytes
      ripemd = hashRaw @RIPEMD160 sha256Once
   in toHexText ripemd

doubleSha256HexOf :: ByteString -> Text
doubleSha256HexOf bytes =
  let firstRound = hashRaw @SHA256 bytes
      secondRound = hashRaw @SHA256 firstRound
   in toHexText secondRound

toHexText :: ByteString -> Text
toHexText = Text.decodeLatin1 . Base16.encode

--------------------------------------------------------------------------------
-- Execution
--------------------------------------------------------------------------------

main :: IO ()
main = do
  args <- getArgs
  (inputPath, outputPath) <- case args of
    [inp, out] -> pure (inp, out)
    _ ->
      die "Usage: generate_hash_vectors_haskell.hs <input.json> <output.json>"

  cryptoInit

  inputBytes <- BS.readFile inputPath
  payload <- either (die . ("Failed to parse input JSON: " <>)) pure $ Aeson.eitherDecodeStrict' inputBytes

  vectorsOut <- forM (hvVectors payload) computeVector

  let metadataIn = hvMetadata payload
      metadataOut = metadataIn{metaGenerator = Just "cabal exec -- runghc scripts/generate_hash_vectors_haskell.hs"}
      output = HashVectorsOut metadataOut vectorsOut
  BSL.writeFile outputPath (Aeson.encode output)
  putStrLn $ "Wrote " <> outputPath

computeVector :: InputVector -> IO OutputVector
computeVector InputVector{..} = do
  bytes <- either (die . formatDecodeError) pure $ decodeHexString (Text.unpack inInputHex)
  let sha256Hex = hashHex @SHA256 bytes
      doubleSha256Hex = doubleSha256HexOf bytes
      sha512Hex = hashHex @SHA512 bytes
      sha3_256Hex = hashHex @SHA3_256 bytes
      sha3_512Hex = hashHex @SHA3_512 bytes
      keccakHex = hashHex @Keccak256 bytes
      ripemdHex = hashHex @RIPEMD160 bytes
      hash160HexText = hash160HexOf bytes
      blake2b224Hex = hashHex @Blake2b_224 bytes
      blake2b256Hex = hashHex @Blake2b_256 bytes
      blake2b512Hex = hashBytesHex (blake2b_libsodium 64 bytes)
  pure
    OutputVector
      { outName = inName
      , outDescription = inDescription
      , outInputHex = inInputHex
      , outSha256 = sha256Hex
      , outSha256d = doubleSha256Hex
      , outSha512 = sha512Hex
      , outSha3_256 = sha3_256Hex
      , outSha3_512 = sha3_512Hex
      , outKeccak256 = keccakHex
      , outRipemd160 = ripemdHex
      , outHash160 = hash160HexText
      , outBlake2b224 = blake2b224Hex
      , outBlake2b256 = blake2b256Hex
      , outBlake2b512 = blake2b512Hex
      }

formatDecodeError :: String -> String
formatDecodeError err = "Invalid input_hex: " <> err
