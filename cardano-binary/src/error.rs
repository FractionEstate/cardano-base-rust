use std::borrow::Cow;
use thiserror::Error;

/// High-level errors produced when encoding or decoding CBOR data within
/// the Cardano binary helpers.
#[derive(Debug, Error)]
pub enum BinaryError {
    #[error("CBOR processing failed: {0}")]
    Cbor(#[from] serde_cbor::Error),

    #[error("decoding `{label}` left {leftover_len} trailing bytes")]
    Leftover {
        label: Cow<'static, str>,
        #[source]
        source: serde_cbor::Error,
        leftover: Vec<u8>,
        leftover_len: usize,
    },

    #[error("nested CBOR expects tag {expected}, found {found:?}")]
    NestedTag { expected: u64, found: Option<u64> },

    #[error("nested CBOR expects a byte string payload")]
    NestedPayload,
}

impl BinaryError {
    pub(crate) fn leftover(
        label: impl Into<Cow<'static, str>>,
        leftover: Vec<u8>,
        source: serde_cbor::Error,
    ) -> Self {
        let leftover_len = leftover.len();
        BinaryError::Leftover {
            label: label.into(),
            source,
            leftover,
            leftover_len,
        }
    }
}
