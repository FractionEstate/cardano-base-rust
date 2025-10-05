//! Direct serialisation helpers mirroring `Cardano.Crypto.DirectSerialise`.
//!
//! These utilities expose type classes (traits) for serialising data directly
//! to raw memory without intermediate heap allocations. This supports
//! zero-copy communication of key material while maintaining safety checks on
//! buffer sizes.

use std::cell::Cell;
use std::ptr::NonNull;

use thiserror::Error;

/// Error raised when a direct serialisation or deserialisation operation
/// writes or reads more bytes than expected.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("size check failed: expected {expected_size}, actual {actual_size}")]
pub struct SizeCheckError {
    pub expected_size: usize,
    pub actual_size: usize,
}

/// Convenience alias for results produced by direct serialisation helpers.
pub type DirectResult<T> = Result<T, SizeCheckError>;

/// Trait for types that can expose their internal representation as raw
/// memory blocks for serialisation.
pub trait DirectSerialise {
    fn direct_serialise(
        &self,
        f: &mut dyn FnMut(*const u8, usize) -> DirectResult<()>,
    ) -> DirectResult<()>;
}

/// Trait for types that can be reconstructed from raw memory blocks during
/// deserialisation.
pub trait DirectDeserialise: Sized {
    fn direct_deserialise(
        f: &mut dyn FnMut(*mut u8, usize) -> DirectResult<()>,
    ) -> DirectResult<Self>;
}

/// Helper that writes into a destination buffer, ensuring no more than
/// `dst_len` bytes are produced. Returns the number of bytes written.
///
/// # Errors
///
/// Returns an error if more than `dst_len` bytes are written.
pub fn direct_serialise_to<T: DirectSerialise>(
    mut write: impl FnMut(usize, *const u8, usize) -> DirectResult<()>,
    dst_len: usize,
    value: &T,
) -> DirectResult<usize> {
    let pos = Cell::new(0usize);

    value.direct_serialise(&mut |ptr, len| {
        let current = pos.get();
        let next = current + len;
        if next > dst_len {
            return Err(SizeCheckError {
                expected_size: dst_len - current,
                actual_size: next - current,
            });
        }
        write(current, ptr, len)?;
        pos.set(next);
        Ok(())
    })?;

    Ok(pos.get())
}

/// Size-checked variant ensuring exactly `dst_len` bytes are written.
///
/// # Errors
///
/// Returns an error if:
/// - More than `dst_len` bytes are written
/// - Fewer than `dst_len` bytes are written
pub fn direct_serialise_to_checked<T: DirectSerialise>(
    write: impl FnMut(usize, *const u8, usize) -> DirectResult<()>,
    dst_len: usize,
    value: &T,
) -> DirectResult<()> {
    let written = direct_serialise_to(write, dst_len, value)?;
    if written != dst_len {
        Err(SizeCheckError {
            expected_size: dst_len,
            actual_size: written,
        })
    } else {
        Ok(())
    }
}

/// Serialise to an in-memory buffer.
///
/// # Errors
///
/// Returns an error if more than `dst_len` bytes are written.
pub fn direct_serialise_buf<T: DirectSerialise>(
    dst: NonNull<u8>,
    dst_len: usize,
    value: &T,
) -> DirectResult<usize> {
    let base = dst.as_ptr();
    direct_serialise_to(
        |offset, src, len| {
            // SAFETY: Caller guarantees dst is valid for dst_len bytes.
            // offset + len <= dst_len is verified by direct_serialise_to.
            // src is valid for len bytes (provided by value.direct_serialise).
            unsafe {
                std::ptr::copy_nonoverlapping(src, base.add(offset), len);
                Ok(())
            }
        },
        dst_len,
        value,
    )
}

/// Serialise to an in-memory buffer, ensuring the buffer is filled exactly.
///
/// # Errors
///
/// Returns an error if:
/// - More than `dst_len` bytes are written
/// - Fewer than `dst_len` bytes are written
pub fn direct_serialise_buf_checked<T: DirectSerialise>(
    dst: NonNull<u8>,
    dst_len: usize,
    value: &T,
) -> DirectResult<()> {
    direct_serialise_to_checked(
        |offset, src, len| {
            // SAFETY: Caller guarantees dst is valid for dst_len bytes.
            // offset + len <= dst_len is verified by direct_serialise_to_checked.
            // src is valid for len bytes (provided by value.direct_serialise).
            unsafe {
                std::ptr::copy_nonoverlapping(src, dst.as_ptr().add(offset), len);
                Ok(())
            }
        },
        dst_len,
        value,
    )
}

/// Helper that reads from a source buffer, ensuring no more than `src_len`
/// bytes are consumed. Returns the deserialised value and the number of bytes
/// read.
///
/// # Errors
///
/// Returns an error if more than `src_len` bytes are read.
pub fn direct_deserialise_from<T: DirectDeserialise>(
    mut read: impl FnMut(usize, *mut u8, usize) -> DirectResult<()>,
    src_len: usize,
) -> DirectResult<(T, usize)> {
    let pos = Cell::new(0usize);

    let value = T::direct_deserialise(&mut |ptr, len| {
        let current = pos.get();
        let next = current + len;
        if next > src_len {
            return Err(SizeCheckError {
                expected_size: src_len - current,
                actual_size: next - current,
            });
        }
        read(current, ptr, len)?;
        pos.set(next);
        Ok(())
    })?;

    Ok((value, pos.get()))
}

/// Size-checked variant ensuring all `src_len` bytes are consumed.
///
/// # Errors
///
/// Returns an error if:
/// - More than `src_len` bytes are read
/// - Fewer than `src_len` bytes are read
pub fn direct_deserialise_from_checked<T: DirectDeserialise>(
    read: impl FnMut(usize, *mut u8, usize) -> DirectResult<()>,
    src_len: usize,
) -> DirectResult<T> {
    let (value, read_len) = direct_deserialise_from(read, src_len)?;
    if read_len != src_len {
        Err(SizeCheckError {
            expected_size: src_len,
            actual_size: read_len,
        })
    } else {
        Ok(value)
    }
}

/// Deserialise from an in-memory buffer with bounds checking.
///
/// # Errors
///
/// Returns an error if more than `src_len` bytes are read.
pub fn direct_deserialise_buf<T: DirectDeserialise>(
    src: NonNull<u8>,
    src_len: usize,
) -> DirectResult<(T, usize)> {
    let base = src.as_ptr();
    direct_deserialise_from(
        |offset, dst, len| {
            // SAFETY: Caller guarantees src is valid for src_len bytes.
            // offset + len <= src_len is verified by direct_deserialise_from.
            // dst is valid for len bytes (provided by T::direct_deserialise).
            unsafe {
                std::ptr::copy_nonoverlapping(base.add(offset), dst, len);
                Ok(())
            }
        },
        src_len,
    )
}

/// Deserialise from an in-memory buffer, ensuring the buffer is consumed.
///
/// # Errors
///
/// Returns an error if:
/// - More than `src_len` bytes are read
/// - Fewer than `src_len` bytes are read
pub fn direct_deserialise_buf_checked<T: DirectDeserialise>(
    src: NonNull<u8>,
    src_len: usize,
) -> DirectResult<T> {
    direct_deserialise_from_checked(
        |offset, dst, len| {
            // SAFETY: Caller guarantees src is valid for src_len bytes.
            // offset + len <= src_len is verified by direct_deserialise_from_checked.
            // dst is valid for len bytes (provided by T::direct_deserialise).
            unsafe {
                std::ptr::copy_nonoverlapping(src.as_ptr().add(offset), dst, len);
                Ok(())
            }
        },
        src_len,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Pair([u8; 4], [u8; 4]);

    impl DirectSerialise for Pair {
        fn direct_serialise(
            &self,
            f: &mut dyn FnMut(*const u8, usize) -> DirectResult<()>,
        ) -> DirectResult<()> {
            f(self.0.as_ptr(), self.0.len())?;
            f(self.1.as_ptr(), self.1.len())
        }
    }

    impl DirectDeserialise for Pair {
        fn direct_deserialise(
            f: &mut dyn FnMut(*mut u8, usize) -> DirectResult<()>,
        ) -> DirectResult<Self> {
            let mut first = [0u8; 4];
            let mut second = [0u8; 4];
            f(first.as_mut_ptr(), first.len())?;
            f(second.as_mut_ptr(), second.len())?;
            Ok(Pair(first, second))
        }
    }

    #[test]
    fn serialise_to_buffer_roundtrip() {
        let pair = Pair(*b"ABCD", *b"WXYZ");
        let mut buffer = [0u8; 8];
        let nn = NonNull::new(buffer.as_mut_ptr()).unwrap();
        direct_serialise_buf_checked(nn, buffer.len(), &pair).unwrap();
        assert_eq!(&buffer, b"ABCDWXYZ");

        let (decoded, read) = direct_deserialise_buf::<Pair>(nn, buffer.len()).unwrap();
        assert_eq!(read, buffer.len());
        assert_eq!(decoded, pair);
    }

    #[test]
    fn serialise_size_mismatch_errors() {
        let pair = Pair(*b"AAAA", *b"BBBB");
        let mut buf = [0u8; 4];
        let nn = NonNull::new(buf.as_mut_ptr()).unwrap();
        let err = direct_serialise_buf(nn, buf.len(), &pair).unwrap_err();
        assert_eq!(
            err,
            SizeCheckError {
                expected_size: 0,
                actual_size: 4
            }
        );
    }

    #[test]
    fn deserialise_size_mismatch_errors() {
        let mut buf = [0u8; 4];
        let nn = NonNull::new(buf.as_mut_ptr()).unwrap();
        let err = direct_deserialise_buf_checked::<Pair>(nn, buf.len()).unwrap_err();
        assert_eq!(
            err,
            SizeCheckError {
                expected_size: 0,
                actual_size: 4
            }
        );
    }
}
