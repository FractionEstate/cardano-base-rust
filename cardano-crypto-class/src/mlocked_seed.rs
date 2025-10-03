use rand::rngs::OsRng;
use rand::RngCore;

use crate::direct_serialise::{DirectDeserialise, DirectResult, DirectSerialise};
use crate::ffi::{SizedMutPtr, SizedPtr};
use crate::mlocked_bytes::{MLockedError, MLockedSizedBytes};

/// Seed stored in mlocked memory to avoid swapping secrets to disk.
pub struct MLockedSeed<const N: usize> {
    bytes: MLockedSizedBytes<N>,
}

impl<const N: usize> MLockedSeed<N> {
    /// Allocate a new seed without initialising the contents.
    pub fn new() -> Result<Self, MLockedError> {
        Ok(Self {
            bytes: MLockedSizedBytes::new()?,
        })
    }

    /// Allocate a new zeroed seed.
    pub fn new_zeroed() -> Result<Self, MLockedError> {
        Ok(Self {
            bytes: MLockedSizedBytes::new_zeroed()?,
        })
    }

    /// Create a copy backed by a fresh allocation.
    pub fn try_clone(&self) -> Result<Self, MLockedError> {
        Ok(Self {
            bytes: self.bytes.try_clone()?,
        })
    }

    /// Fill the seed with cryptographically secure random bytes.
    pub fn fill_random(&mut self) -> Result<(), MLockedError> {
        let mut rng = OsRng;
        rng.try_fill_bytes(self.bytes.as_mut_slice())
            .map_err(|err| MLockedError::RandomFailed { source: err })
    }

    /// Construct a fresh random seed.
    pub fn new_random() -> Result<Self, MLockedError> {
        let mut seed = Self::new_zeroed()?;
        seed.fill_random()?;
        Ok(seed)
    }

    /// Execute `f` with a raw pointer to the underlying bytes.
    pub fn with_c_ptr<R>(&self, f: impl FnOnce(*const u8, usize) -> R) -> R {
        self.bytes.with_c_ptr(|ptr| f(ptr, N))
    }

    /// Execute `f` with a mutable raw pointer to the underlying bytes.
    pub fn with_c_ptr_mut<R>(&mut self, f: impl FnOnce(*mut u8, usize) -> R) -> R {
        self.bytes.with_c_ptr_mut(|ptr| f(ptr, N))
    }

    /// Execute `f` with an FFI-sized pointer wrapper.
    pub fn with_sized_ptr<R>(&self, f: impl FnOnce(SizedPtr<'_, N>) -> R) -> R {
        self.bytes.with_sized_ptr(f)
    }

    /// Execute `f` with a mutable sized pointer wrapper.
    pub fn with_sized_ptr_mut<R>(&mut self, f: impl FnOnce(SizedMutPtr<'_, N>) -> R) -> R {
        self.bytes.with_sized_ptr_mut(f)
    }

    /// Explicitly zero and free the underlying memory.
    pub fn finalize(self) {
        drop(self);
    }

    /// Immutable view for testing and higher-level abstractions.
    pub fn as_bytes(&self) -> &[u8; N] {
        self.bytes.as_array()
    }

    /// Mutable view of the underlying bytes.
    pub fn as_mut_bytes(&mut self) -> &mut [u8; N] {
        self.bytes.as_mut_array()
    }
}

impl<const N: usize> DirectSerialise for MLockedSeed<N> {
    fn direct_serialise(
        &self,
        f: &mut dyn FnMut(*const u8, usize) -> DirectResult<()>,
    ) -> DirectResult<()> {
        self.bytes.with_c_ptr(|ptr| f(ptr, N))
    }
}

impl<const N: usize> DirectDeserialise for MLockedSeed<N> {
    fn direct_deserialise(
        f: &mut dyn FnMut(*mut u8, usize) -> DirectResult<()>,
    ) -> DirectResult<Self> {
        let mut seed = Self::new_zeroed().expect("failed to allocate mlocked seed");
        seed.bytes.with_c_ptr_mut(|ptr| f(ptr, N))?;
        Ok(seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direct_serialise::{direct_deserialise_buf_checked, direct_serialise_buf_checked};
    use std::ptr::NonNull;

    #[test]
    fn random_seed_has_content() {
        let seed = MLockedSeed::<32>::new_random().unwrap();
        assert!(seed.as_bytes().iter().any(|&b| b != 0));
    }

    #[test]
    fn direct_serialise_roundtrip() {
        let mut seed = MLockedSeed::<16>::new_zeroed().unwrap();
        seed.as_mut_bytes().copy_from_slice(b"0123456789abcdef");
        let mut buffer = [0u8; 16];
        let nn = NonNull::new(buffer.as_mut_ptr()).unwrap();
        direct_serialise_buf_checked(nn, buffer.len(), &seed).unwrap();
        assert_eq!(&buffer, b"0123456789abcdef");

        let roundtrip =
            direct_deserialise_buf_checked::<MLockedSeed<16>>(nn, buffer.len()).unwrap();
        assert_eq!(roundtrip.as_bytes(), seed.as_bytes());
    }
}
