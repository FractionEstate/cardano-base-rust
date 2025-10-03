use crate::error::BinaryError;
use serde::Serialize;
use serde_cbor::tags::Tagged;
use serde_cbor::value::Value;
use serde_cbor::{to_vec, to_writer};
use std::io::Write;

/// Serialise a value into a vector of bytes using canonical CBOR semantics.
pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError> {
    to_vec(value).map_err(BinaryError::from)
}

/// Serialise a value and return an owned byte vector (strict variant).
pub fn serialize_strict<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError> {
    serialize(value)
}

/// Serialise a value using an existing IO writer.
pub fn serialize_into_writer<T, W>(value: &T, writer: W) -> Result<(), BinaryError>
where
    T: Serialize,
    W: Write,
{
    to_writer(writer, value).map_err(BinaryError::from)
}

/// Serialise into an existing byte buffer, reusing its allocation.
pub fn serialize_into_vec<T: Serialize>(
    value: &T,
    buffer: &mut Vec<u8>,
) -> Result<(), BinaryError> {
    buffer.clear();
    to_writer(buffer, value).map_err(BinaryError::from)
}

/// Serialise into a vector after reserving the provided capacity hint.
pub fn serialize_with_capacity<T: Serialize>(
    value: &T,
    capacity_hint: usize,
) -> Result<Vec<u8>, BinaryError> {
    let mut buffer = Vec::with_capacity(capacity_hint);
    serialize_into_vec(value, &mut buffer)?;
    Ok(buffer)
}

/// Produce a nested CBOR encoding using the semantic tag 24.
pub fn encode_nested_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError> {
    let inner = serialize(value)?;
    encode_nested_cbor_bytes(&inner)
}

/// Wrap an existing CBOR payload with the semantic tag 24.
pub fn encode_nested_cbor_bytes(bytes: &[u8]) -> Result<Vec<u8>, BinaryError> {
    let tagged: Tagged<Value> = Tagged {
        tag: Some(24),
        value: Value::Bytes(bytes.to_vec()),
    };
    to_vec(&tagged).map_err(BinaryError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_bytes::ByteBuf;
    use serde_cbor::tags::Tagged;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Sample {
        label: String,
        value: u32,
    }

    #[test]
    fn serialises_roundtrip() {
        let sample = Sample {
            label: "abc".into(),
            value: 42,
        };
        let bytes = serialize(&sample).unwrap();
        let decoded: Sample = serde_cbor::from_slice(&bytes).unwrap();
        assert_eq!(sample, decoded);
    }

    #[test]
    fn encode_nested_wraps_tag() {
        let payload = ByteBuf::from(vec![0x01, 0x02]);
        let tagged = encode_nested_cbor(&payload).unwrap();
        let Tagged { tag, value }: Tagged<Value> = serde_cbor::from_slice(&tagged).unwrap();
        match (tag, value) {
            (Some(24), Value::Bytes(bytes)) => {
                assert_eq!(bytes, serde_cbor::to_vec(&payload).unwrap())
            }
            other => panic!("unexpected value: {other:?}"),
        }
    }

    #[test]
    fn reuse_buffer_serialisation() {
        let sample = Sample {
            label: "reuse".into(),
            value: 7,
        };
        let mut buffer = vec![0xde, 0xad, 0xbe, 0xef];
        serialize_into_vec(&sample, &mut buffer).unwrap();
        assert_ne!(buffer, vec![0xde, 0xad, 0xbe, 0xef]);

        let decoded: Sample = serde_cbor::from_slice(&buffer).unwrap();
        assert_eq!(decoded, sample);
    }

    #[test]
    fn capacity_hint_serialises() {
        let sample = Sample {
            label: "hint".into(),
            value: 99,
        };
        let encoded = serialize_with_capacity(&sample, 128).unwrap();
        let decoded: Sample = serde_cbor::from_slice(&encoded).unwrap();
        assert_eq!(decoded, sample);
        assert!(encoded.capacity() >= 128);
    }
}
