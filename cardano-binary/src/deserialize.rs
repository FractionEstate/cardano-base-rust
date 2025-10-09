#![cfg_attr(test, allow(clippy::unwrap_used))]

use crate::error::BinaryError;
use serde::de::DeserializeOwned;
use std::io::Cursor;

/// Deserialize a value, panicking if decoding fails.
///
/// # Panics
///
/// This function will panic if the input is not valid CBOR or cannot be
/// deserialized into type `T`. For production code, prefer using [`decode_full`]
/// which returns a `Result` instead.
///
/// # Deprecated
///
/// This function is deprecated and will be removed in a future version.
/// Use [`decode_full`] for proper error handling.
#[deprecated(
    since = "0.1.1",
    note = "Use decode_full() instead for proper error handling"
)]
#[must_use]
pub fn unsafe_deserialize<T: DeserializeOwned>(bytes: &[u8]) -> T {
    decode_full(bytes).expect("CBOR deserialization failed - use decode_full() for error handling")
}

/// Deserialize a value from an owned vector, panicking if decoding fails.
///
/// # Panics
///
/// This function will panic if the input is not valid CBOR.
///
/// # Deprecated
///
/// This function is deprecated and will be removed in a future version.
/// Use [`decode_full`] for proper error handling.
#[deprecated(
    since = "0.1.1",
    note = "Use decode_full() instead for proper error handling"
)]
#[must_use]
pub fn unsafe_deserialize_owned<T: DeserializeOwned>(bytes: Vec<u8>) -> T {
    #[allow(deprecated)]
    unsafe_deserialize(&bytes)
}

/// Deserialize a value from a byte slice, consuming the entire payload.
///
/// # Errors
///
/// Returns [`BinaryError::Deserialization`] if:
/// - The input is not valid CBOR
/// - The CBOR structure doesn't match the expected type
/// - There are leftover bytes after deserialization
pub fn decode_full<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, BinaryError> {
    let mut cursor = Cursor::new(bytes);
    let value: T = ciborium::from_reader(&mut cursor)?;

    let position = cursor.position() as usize;
    if position < bytes.len() {
        let leftover = bytes[position..].to_vec();
        return Err(BinaryError::leftover(std::any::type_name::<T>(), leftover));
    }

    Ok(value)
}

/// Strict variant of [`decode_full`] operating on owned bytes.
///
/// # Errors
///
/// Returns [`BinaryError::Deserialization`] if:
/// - The input is not valid CBOR
/// - The CBOR structure doesn't match the expected type
/// - There are leftover bytes after deserialization
pub fn decode_full_owned<T: DeserializeOwned>(bytes: Vec<u8>) -> Result<T, BinaryError> {
    decode_full(&bytes)
}

/// Decode a nested CBOR payload wrapped in semantic tag 24 and deserialize it as type `T`.
///
/// # Errors
///
/// Returns an error if:
/// - The outer CBOR payload is invalid
/// - The tag is not 24
/// - The inner payload is not bytes
/// - The inner bytes cannot be deserialized as type `T`
pub fn decode_nested_cbor<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, BinaryError> {
    let raw = decode_nested_cbor_bytes(bytes)?;
    decode_full(&raw)
}

/// Decode a nested CBOR payload wrapped in semantic tag 24 and return the raw bytes.
///
/// # Errors
///
/// Returns an error if:
/// - The CBOR payload is invalid
/// - The tag is not 24
/// - The payload is not bytes
pub fn decode_nested_cbor_bytes(bytes: &[u8]) -> Result<Vec<u8>, BinaryError> {
    use ciborium::value::Value;

    let value: Value = ciborium::from_reader(bytes)?;

    match value {
        Value::Tag(24, boxed_value) => match *boxed_value {
            Value::Bytes(inner) => Ok(inner),
            _ => Err(BinaryError::NestedPayload),
        },
        Value::Tag(other, _) => Err(BinaryError::NestedTag {
            expected: 24,
            found: Some(other),
        }),
        _ => Err(BinaryError::NestedTag {
            expected: 24,
            found: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_bytes::ByteBuf;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Sample {
        label: String,
        value: u32,
    }

    #[test]
    fn decode_full_consumes_payload() {
        let sample = Sample {
            label: "abc".into(),
            value: 99,
        };
        let mut bytes = Vec::new();
        ciborium::into_writer(&sample, &mut bytes).unwrap();
        let decoded: Sample = decode_full(&bytes).unwrap();
        assert_eq!(sample, decoded);
    }

    #[test]
    fn decode_full_reports_leftover() {
        let sample = Sample {
            label: "abc".into(),
            value: 1,
        };
        let mut bytes = Vec::new();
        ciborium::into_writer(&sample, &mut bytes).unwrap();
        bytes.extend_from_slice(&[0xff]); // extra data
        let err = decode_full::<Sample>(&bytes).unwrap_err();
        let leftover_len = match err {
            BinaryError::Leftover { leftover_len, .. } => Ok(leftover_len),
            _ => Err(()),
        }
        .expect("expected leftover error");
        assert_eq!(leftover_len, 1);
    }

    #[test]
    fn nested_roundtrip() {
        let payload = ByteBuf::from(vec![0xde, 0xad, 0xbe, 0xef]);
        let tagged = crate::serialize::encode_nested_cbor(&payload).unwrap();
        let roundtrip: ByteBuf = decode_nested_cbor(&tagged).unwrap();
        assert_eq!(roundtrip, payload);
    }

    #[test]
    fn nested_requires_tag() {
        let payload_data = ByteBuf::from(vec![1u8, 2, 3]);
        let mut payload = Vec::new();
        ciborium::into_writer(&payload_data, &mut payload).unwrap();
        let err = decode_nested_cbor_bytes(&payload).unwrap_err();
        let (expected, found) = match err {
            BinaryError::NestedTag { expected, found } => Ok((expected, found)),
            _ => Err(()),
        }
        .expect("expected nested tag error");
        assert_eq!(expected, 24);
        assert_eq!(found, None);
    }

    #[test]
    fn nested_requires_byte_payload() {
        use ciborium::value::Value;

        let value = Value::Tag(24, Box::new(Value::Text("not-bytes".into())));
        let mut encoded = Vec::new();
        ciborium::into_writer(&value, &mut encoded).unwrap();
        let err = decode_nested_cbor_bytes(&encoded).unwrap_err();
        assert!(matches!(err, BinaryError::NestedPayload));
    }
}
