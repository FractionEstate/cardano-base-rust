use crate::error::BinaryError;
use serde::de::{DeserializeOwned, Error as DeError};
use serde_cbor::de::{Deserializer, SliceRead};
use serde_cbor::tags::Tagged;
use serde_cbor::value::Value;
use serde_cbor::Error as CborError;
use std::borrow::Cow;

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
pub fn unsafe_deserialize_owned<T: DeserializeOwned>(bytes: Vec<u8>) -> T {
    #[allow(deprecated)]
    unsafe_deserialize(&bytes)
}

/// Deserialize a value from a byte slice, consuming the entire payload.
pub fn decode_full<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, BinaryError> {
    decode_full_decoder(std::any::type_name::<T>(), bytes, |de| T::deserialize(de))
}

/// Strict variant of [`decode_full`] operating on owned bytes.
pub fn decode_full_owned<T: DeserializeOwned>(bytes: Vec<u8>) -> Result<T, BinaryError> {
    decode_full(&bytes)
}

/// Run a custom decoder against the provided slice, ensuring all bytes are consumed.
pub fn decode_full_decoder<T, F>(
    label: impl Into<Cow<'static, str>>,
    bytes: &[u8],
    decode: F,
) -> Result<T, BinaryError>
where
    F: for<'de> FnOnce(&mut Deserializer<SliceRead<'de>>) -> Result<T, CborError>,
{
    let (value, leftover) = deserialise_decoder(bytes, decode)?;
    if leftover.is_empty() {
        Ok(value)
    } else {
        Err(BinaryError::leftover(
            label,
            leftover,
            <CborError as DeError>::custom("trailing data remaining after decode"),
        ))
    }
}

/// Run a custom decoder against the provided slice and return the decoded value alongside leftover bytes.
pub fn deserialise_decoder<'a, T, F>(
    bytes: &'a [u8],
    decode: F,
) -> Result<(T, Vec<u8>), BinaryError>
where
    F: for<'de> FnOnce(&mut Deserializer<SliceRead<'de>>) -> Result<T, CborError>,
{
    let mut deserializer = Deserializer::from_slice(bytes);
    let value = decode(&mut deserializer)?;
    let consumed = deserializer.byte_offset();
    let leftover = bytes[consumed..].to_vec();
    Ok((value, leftover))
}

/// Decode a nested CBOR payload wrapped in semantic tag 24 and deserialize it as type `T`.
pub fn decode_nested_cbor<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, BinaryError> {
    let raw = decode_nested_cbor_bytes(bytes)?;
    decode_full(&raw)
}

/// Decode a nested CBOR payload wrapped in semantic tag 24 and return the raw bytes.
pub fn decode_nested_cbor_bytes(bytes: &[u8]) -> Result<Vec<u8>, BinaryError> {
    let Tagged { tag, value }: Tagged<Value> = decode_full(bytes)?;
    match (tag, value) {
        (Some(24), Value::Bytes(inner)) => Ok(inner),
        (Some(24), _) => Err(BinaryError::NestedPayload),
        (Some(other), _) => Err(BinaryError::NestedTag {
            expected: 24,
            found: Some(other),
        }),
        (None, _) => Err(BinaryError::NestedTag {
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
        let bytes = serde_cbor::to_vec(&sample).unwrap();
        let decoded: Sample = decode_full(&bytes).unwrap();
        assert_eq!(sample, decoded);
    }

    #[test]
    fn decode_full_reports_leftover() {
        let sample = Sample {
            label: "abc".into(),
            value: 1,
        };
        let mut bytes = serde_cbor::to_vec(&sample).unwrap();
        bytes.extend_from_slice(&[0xff]); // extra data
        let err = decode_full::<Sample>(&bytes).unwrap_err();
        match err {
            BinaryError::Leftover { leftover_len, .. } => assert_eq!(leftover_len, 1),
            other => panic!("unexpected error: {other:?}"),
        }
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
        let payload = serde_cbor::to_vec(&ByteBuf::from(vec![1u8, 2, 3])).unwrap();
        let err = decode_nested_cbor_bytes(&payload).unwrap_err();
        match err {
            BinaryError::NestedTag { expected, found } => {
                assert_eq!(expected, 24);
                assert_eq!(found, None);
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn nested_requires_byte_payload() {
        let value = serde_cbor::value::Value::Tag(
            24,
            Box::new(serde_cbor::value::Value::Text("not-bytes".into())),
        );
        let encoded = serde_cbor::to_vec(&value).unwrap();
        let err = decode_nested_cbor_bytes(&encoded).unwrap_err();
        assert!(matches!(err, BinaryError::NestedPayload));
    }
}
