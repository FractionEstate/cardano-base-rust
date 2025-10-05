#![cfg_attr(test, allow(clippy::unwrap_used))]

use crate::error::BinaryError;
use serde::Serialize;
use std::io::Write;

/// Serialise a value into a vector of bytes using canonical CBOR semantics.
///
/// # Errors
///
/// Returns [`BinaryError::Serialization`] if the value cannot be serialized to CBOR.
pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf)?;
    Ok(buf)
}

/// Serialise a value and return an owned byte vector (strict variant).
///
/// # Errors
///
/// Returns [`BinaryError::Serialization`] if the value cannot be serialized to CBOR.
pub fn serialize_strict<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError> {
    serialize(value)
}

/// Serialise a value using an existing IO writer.
///
/// # Errors
///
/// Returns [`BinaryError::Serialization`] if:
/// - The value cannot be serialized to CBOR
/// - Writing to the output fails
pub fn serialize_into_writer<T, W>(value: &T, writer: W) -> Result<(), BinaryError>
where
    T: Serialize,
    W: Write,
{
    ciborium::into_writer(value, writer)?;
    Ok(())
}

/// Serialise into an existing byte buffer, reusing its allocation.
///
/// # Errors
///
/// Returns [`BinaryError::Serialization`] if the value cannot be serialized to CBOR.
pub fn serialize_into_vec<T: Serialize>(
    value: &T,
    buffer: &mut Vec<u8>,
) -> Result<(), BinaryError> {
    buffer.clear();
    ciborium::into_writer(value, buffer)?;
    Ok(())
}

/// Serialise into a vector after reserving the provided capacity hint.
///
/// # Errors
///
/// Returns [`BinaryError::Serialization`] if the value cannot be serialized to CBOR.
pub fn serialize_with_capacity<T: Serialize>(
    value: &T,
    capacity_hint: usize,
) -> Result<Vec<u8>, BinaryError> {
    let mut buffer = Vec::with_capacity(capacity_hint);
    serialize_into_vec(value, &mut buffer)?;
    Ok(buffer)
}

/// Produce a nested CBOR encoding using the semantic tag 24.
///
/// # Errors
///
/// Returns [`BinaryError::Serialization`] if the value cannot be serialized to CBOR.
pub fn encode_nested_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError> {
    let inner = serialize(value)?;
    encode_nested_cbor_bytes(&inner)
}

/// Wrap an existing CBOR payload with the semantic tag 24.
///
/// # Errors
///
/// Returns [`BinaryError::Serialization`] if the tagged value cannot be serialized to CBOR.
pub fn encode_nested_cbor_bytes(bytes: &[u8]) -> Result<Vec<u8>, BinaryError> {
    use ciborium::value::Value;

    let tagged = Value::Tag(24, Box::new(Value::Bytes(bytes.to_vec())));
    let mut buf = Vec::new();
    ciborium::into_writer(&tagged, &mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_bytes::ByteBuf;

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
        let decoded: Sample = ciborium::from_reader(&bytes[..]).unwrap();
        assert_eq!(sample, decoded);
    }

    #[test]
    fn encode_nested_wraps_tag() {
        use ciborium::value::Value;

        let payload = ByteBuf::from(vec![0x01, 0x02]);
        let tagged = encode_nested_cbor(&payload).unwrap();
        let decoded: Value = ciborium::from_reader(&tagged[..]).unwrap();

        match decoded {
            Value::Tag(24, boxed_value) => match *boxed_value {
                Value::Bytes(bytes) => {
                    let mut expected = Vec::new();
                    ciborium::into_writer(&payload, &mut expected).unwrap();
                    assert_eq!(bytes, expected);
                },
                other => panic!("unexpected inner value: {other:?}"),
            },
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

        let decoded: Sample = ciborium::from_reader(&buffer[..]).unwrap();
        assert_eq!(decoded, sample);
    }

    #[test]
    fn capacity_hint_serialises() {
        let sample = Sample {
            label: "hint".into(),
            value: 99,
        };
        let encoded = serialize_with_capacity(&sample, 128).unwrap();
        let decoded: Sample = ciborium::from_reader(&encoded[..]).unwrap();
        assert_eq!(decoded, sample);
        assert!(encoded.capacity() >= 128);
    }
}
