use std::borrow::Cow;
use std::io;
use thiserror::Error;

/// High-level errors produced when encoding or decoding CBOR data within
/// the Cardano binary helpers.
#[derive(Debug, Error)]
pub enum BinaryError {
    #[error("CBOR serialization failed: {0}")]
    Serialization(#[from] ciborium::ser::Error<io::Error>),

    #[error("CBOR deserialization failed: {0}")]
    Deserialization(#[from] ciborium::de::Error<io::Error>),

    #[error("decoding `{label}` left {leftover_len} trailing bytes")]
    Leftover {
        label: Cow<'static, str>,
        leftover: Vec<u8>,
        leftover_len: usize,
    },

    #[error("nested CBOR expects tag {expected}, found {found:?}")]
    NestedTag { expected: u64, found: Option<u64> },

    #[error("nested CBOR expects a byte string payload")]
    NestedPayload,

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

impl BinaryError {
    pub(crate) fn leftover(label: impl Into<Cow<'static, str>>, leftover: Vec<u8>) -> Self {
        let leftover_len = leftover.len();
        BinaryError::Leftover {
            label: label.into(),
            leftover,
            leftover_len,
        }
    }
}
