//! Binary serialisation and deserialisation helpers for Cardano components.
//!
//! The API mirrors the spirit of the original Haskell `cardano-binary`
//! package, offering high-level helpers to serialise values to CBOR, decode
//! complete payloads, and work with nested CBOR-in-CBOR structures.

#![cfg_attr(test, allow(clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::approx_constant))]

mod deserialize;
mod error;
mod serialize;

#[allow(deprecated)]
pub use crate::deserialize::{
    decode_full, decode_full_owned, decode_nested_cbor, decode_nested_cbor_bytes,
    unsafe_deserialize, unsafe_deserialize_owned,
};

pub use crate::error::BinaryError;

pub use crate::serialize::{
    encode_nested_cbor, encode_nested_cbor_bytes, serialize, serialize_into_vec,
    serialize_into_writer, serialize_strict, serialize_with_capacity,
};

#[cfg(test)]
mod roundtrip_tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Sample {
        id: u64,
        payload: String,
    }

    #[test]
    fn roundtrip_helpers_work() {
        let sample = Sample {
            id: 7,
            payload: "hello".into(),
        };
        let bytes = serialize(&sample).expect("serialize");
        let decoded = decode_full::<Sample>(&bytes).expect("decode");
        assert_eq!(sample, decoded);
    }
}
