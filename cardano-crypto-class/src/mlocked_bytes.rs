use std::io;
use std::ptr::{self, NonNull};
use std::slice;

use crate::ffi::{SizedMutPtr, SizedPtr};
use thiserror::Error;

/// Errors that can occur when working with mlocked memory.
#[derive(Debug, Error)]
pub enum MLockedError {
    #[error("allocation failed")]
    AllocationFailed,
    #[error("mlock failed: {code}")]
    LockFailed { code: i32 },
    #[error("alignment must be non-zero")]
    InvalidAlignment,
    #[error("requested size is too large")]
    AllocationTooLarge,
    #[error("random generator failure: {source}")]
    RandomFailed { source: rand::Error },
}

#[derive(Debug)]
struct MLockedRegion {
    ptr: NonNull<u8>,
    len: usize,
    locked: bool,
}

impl MLockedRegion {
    fn allocate(len: usize, zeroed: bool) -> Result<Self, MLockedError> {
        Self::allocate_aligned(len, zeroed, None)
    }

    fn allocate_aligned(
        len: usize,
        zeroed: bool,
        align: Option<usize>,
    ) -> Result<Self, MLockedError> {
        let requested = len;

        if requested == 0 {
            // SAFETY: malloc(1) always returns a valid pointer or NULL.
            // We check for NULL immediately and return an error.
            let ptr = unsafe { libc::malloc(1) } as *mut u8;
            if ptr.is_null() {
                return Err(MLockedError::AllocationFailed);
            }
            return Ok(Self {
                // SAFETY: We verified ptr is non-null above
                ptr: unsafe { NonNull::new_unchecked(ptr) },
                len: 0,
                locked: false,
            });
        }

        let alloc_len = match align {
            Some(alignment) => {
                if alignment == 0 {
                    return Err(MLockedError::InvalidAlignment);
                }
                round_up_to(requested, alignment)?
            }
            None => requested,
        };

        // SAFETY: malloc/calloc return either a valid pointer or NULL.
        // We check for NULL below and handle the error case.
        let ptr = unsafe {
            if zeroed {
                libc::calloc(1, alloc_len)
            } else {
                libc::malloc(alloc_len)
            }
        } as *mut u8;

        if ptr.is_null() {
            return Err(MLockedError::AllocationFailed);
        }

        // SAFETY: ptr is valid and alloc_len is the allocated size.
        // mlock may fail if we exceed ulimit, which we handle below.
        let lock_result = unsafe { libc::mlock(ptr.cast(), alloc_len) };
        if lock_result != 0 {
            let err = io::Error::last_os_error();
            // SAFETY: ptr was allocated by malloc/calloc above
            unsafe {
                libc::free(ptr.cast());
            }
            return Err(MLockedError::LockFailed {
                code: err.raw_os_error().unwrap_or_default(),
            });
        }

        Ok(Self {
            // SAFETY: We verified ptr is non-null above
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            len: alloc_len,
            locked: true,
        })
    }

    fn as_slice(&self) -> &[u8] {
        // SAFETY: self.ptr is valid for self.len bytes, allocated by malloc/calloc,
        // and remains valid for the lifetime of this MLockedRegion.
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        // SAFETY: self.ptr is valid for self.len bytes, allocated by malloc/calloc,
        // we have exclusive access (&mut self), and it remains valid for 'self lifetime.
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    fn as_non_null(&self) -> NonNull<u8> {
        self.ptr
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Drop for MLockedRegion {
    fn drop(&mut self) {
        if self.len > 0 {
            // SAFETY: self.ptr is valid for self.len bytes.
            // Zeroing memory for security before deallocation.
            unsafe {
                ptr::write_bytes(self.ptr.as_ptr(), 0, self.len);
            }
        }

        if self.locked {
            // SAFETY: self.ptr was locked with mlock() in allocate_aligned.
            // Unlocking before freeing to avoid resource leaks.
            unsafe {
                libc::munlock(self.ptr.as_ptr().cast(), self.len);
            }
        }

        // SAFETY: self.ptr was allocated by malloc/calloc in allocate_aligned.
        // This is the final cleanup, and ptr won't be used after this.
        unsafe {
            libc::free(self.ptr.as_ptr().cast());
        }
    }
}

fn round_up_to(value: usize, align: usize) -> Result<usize, MLockedError> {
    if align == 0 {
        return Err(MLockedError::InvalidAlignment);
    }

    let remainder = value % align;
    if remainder == 0 {
        Ok(value)
    } else {
        value
            .checked_add(align - remainder)
            .ok_or(MLockedError::AllocationTooLarge)
    }
}

/// Heap allocation backed by `mlock(2)` with a runtime length.
pub struct MLockedBytes {
    region: MLockedRegion,
}

impl MLockedBytes {
    /// Allocate a new mlocked buffer with undefined contents.
    pub fn new(len: usize) -> Result<Self, MLockedError> {
        Ok(Self {
            region: MLockedRegion::allocate(len, false)?,
        })
    }

    /// Allocate a new zeroed mlocked buffer.
    pub fn new_zeroed(len: usize) -> Result<Self, MLockedError> {
        Ok(Self {
            region: MLockedRegion::allocate(len, true)?,
        })
    }

    /// Allocate a new buffer rounding the size up to a multiple of `align`.
    pub fn new_aligned(len: usize, align: usize) -> Result<Self, MLockedError> {
        Ok(Self {
            region: MLockedRegion::allocate_aligned(len, false, Some(align))?,
        })
    }

    /// Length in bytes.
    pub fn len(&self) -> usize {
        self.region.len()
    }

    /// Returns `true` if the allocation is empty.
    pub fn is_empty(&self) -> bool {
        self.region.is_empty()
    }

    /// Immutable view of the underlying bytes.
    pub fn as_slice(&self) -> &[u8] {
        self.region.as_slice()
    }

    /// Mutable view of the underlying bytes.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.region.as_mut_slice()
    }

    /// Raw pointer to the allocation.
    pub fn as_ptr(&self) -> *const u8 {
        self.region.as_ptr()
    }

    /// Mutable raw pointer to the allocation.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.region.as_mut_ptr()
    }

    /// Call `f` with a raw pointer to the allocation.
    pub fn with_c_ptr<R>(&self, f: impl FnOnce(*const u8) -> R) -> R {
        f(self.as_ptr())
    }

    /// Call `f` with a mutable raw pointer to the allocation.
    pub fn with_c_ptr_mut<R>(&mut self, f: impl FnOnce(*mut u8) -> R) -> R {
        f(self.as_mut_ptr())
    }

    /// Create a deep copy of this buffer.
    pub fn try_clone(&self) -> Result<Self, MLockedError> {
        let mut cloned = Self::new(self.len())?;
        if self.len() > 0 {
            // SAFETY: Both self.as_ptr() and cloned.as_mut_ptr() are valid for self.len() bytes.
            // Regions don't overlap (cloned was just allocated), satisfying copy_nonoverlapping requirements.
            unsafe {
                ptr::copy_nonoverlapping(self.as_ptr(), cloned.as_mut_ptr(), self.len());
            }
        }
        Ok(cloned)
    }

    /// Explicitly zero and unlock the memory before dropping it.
    pub fn finalize(self) {
        drop(self);
    }
}

/// Secure heap allocation backed by `mlock(2)` to prevent swapping.
pub struct MLockedSizedBytes<const N: usize> {
    region: MLockedRegion,
}

impl<const N: usize> MLockedSizedBytes<N> {
    fn allocate(zeroed: bool) -> Result<Self, MLockedError> {
        Ok(Self {
            region: MLockedRegion::allocate(N, zeroed)?,
        })
    }

    /// Allocate a new mlocked buffer with undefined contents.
    pub fn new() -> Result<Self, MLockedError> {
        Self::allocate(false)
    }

    /// Allocate a new zeroed mlocked buffer.
    pub fn new_zeroed() -> Result<Self, MLockedError> {
        Self::allocate(true)
    }

    /// Create a deep copy of this buffer.
    pub fn try_clone(&self) -> Result<Self, MLockedError> {
        let mut cloned = Self::new()?;
        if N > 0 {
            cloned.as_mut_slice().copy_from_slice(self.as_slice());
        }
        Ok(cloned)
    }

    /// Length in bytes.
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns true if this represents an empty allocation.
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Immutable view of the underlying bytes.
    pub fn as_slice(&self) -> &[u8] {
        self.region.as_slice()
    }

    /// Immutable view as a fixed-size array reference.
    pub fn as_array(&self) -> &[u8; N] {
        // SAFETY: self.region.as_ptr() points to at least N bytes (enforced by MLockedRegion::allocate(N)).
        // Casting to *const [u8; N] is valid because the memory layout matches.
        unsafe { &*(self.region.as_ptr() as *const [u8; N]) }
    }

    /// Mutable view of the underlying bytes.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.region.as_mut_slice()
    }

    /// Mutable view as a fixed-size array reference.
    pub fn as_mut_array(&mut self) -> &mut [u8; N] {
        // SAFETY: self.region.as_mut_ptr() points to at least N bytes (enforced by MLockedRegion::allocate(N)).
        // We have exclusive access via &mut self. Casting to *mut [u8; N] is valid.
        unsafe { &mut *(self.region.as_mut_ptr() as *mut [u8; N]) }
    }

    /// Call `f` with a raw pointer to the memory block.
    pub fn with_c_ptr<R>(&self, f: impl FnOnce(*const u8) -> R) -> R {
        f(self.region.as_ptr())
    }

    /// Call `f` with a mutable raw pointer to the memory block.
    pub fn with_c_ptr_mut<R>(&mut self, f: impl FnOnce(*mut u8) -> R) -> R {
        f(self.region.as_mut_ptr())
    }

    /// Call `f` with a sized pointer wrapper.
    pub fn with_sized_ptr<R>(&self, f: impl FnOnce(SizedPtr<'_, N>) -> R) -> R {
        f(SizedPtr::new(self.region.as_non_null()))
    }

    /// Call `f` with a mutable sized pointer wrapper.
    pub fn with_sized_ptr_mut<R>(&mut self, f: impl FnOnce(SizedMutPtr<'_, N>) -> R) -> R {
        f(SizedMutPtr::new(self.region.as_non_null()))
    }

    /// Explicitly zero and unlock the memory before dropping it.
    pub fn finalize(self) {
        drop(self);
    }
}

/// Allocator mirroring the Haskell `MLockedAllocator` abstraction.
#[derive(Clone, Copy, Default)]
pub struct MLockedAllocator;

impl MLockedAllocator {
    /// Allocate a buffer with undefined contents.
    pub fn allocate(&self, len: usize) -> Result<MLockedBytes, MLockedError> {
        MLockedBytes::new(len)
    }

    /// Allocate a zeroed buffer.
    pub fn allocate_zeroed(&self, len: usize) -> Result<MLockedBytes, MLockedError> {
        MLockedBytes::new_zeroed(len)
    }

    /// Allocate a buffer rounding the size up to the requested alignment.
    pub fn allocate_aligned(&self, len: usize, align: usize) -> Result<MLockedBytes, MLockedError> {
        Ok(MLockedBytes {
            region: MLockedRegion::allocate_aligned(len, false, Some(align))?,
        })
    }
}

/// Default allocator equivalent to `mlockedMalloc`.
pub fn mlocked_allocator() -> MLockedAllocator {
    MLockedAllocator::default()
}

/// Allocate an mlocked region with undefined contents.
pub fn mlocked_alloc_bytes(len: usize) -> Result<MLockedBytes, MLockedError> {
    MLockedAllocator::default().allocate(len)
}

/// Allocate a zeroed mlocked region.
pub fn mlocked_alloc_bytes_zeroed(len: usize) -> Result<MLockedBytes, MLockedError> {
    MLockedAllocator::default().allocate_zeroed(len)
}

/// Allocate an mlocked region rounding up to the nearest multiple of `align`.
pub fn mlocked_alloc_bytes_aligned(len: usize, align: usize) -> Result<MLockedBytes, MLockedError> {
    Ok(MLockedBytes {
        region: MLockedRegion::allocate_aligned(len, false, Some(align))?,
    })
}

/// Zero `len` bytes starting at `ptr`.
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is valid for writes of at least `len` bytes
/// - `ptr` is properly aligned
/// - The memory region [ptr, ptr+len) doesn't overlap with any borrowed references
pub unsafe fn zero_mem(ptr: *mut u8, len: usize) {
    if len == 0 {
        return;
    }
    ptr::write_bytes(ptr, 0, len);
}

/// Copy `len` bytes from `src` to `dst`. The regions must not overlap.
///
/// # Safety
///
/// The caller must ensure that:
/// - `src` is valid for reads of at least `len` bytes
/// - `dst` is valid for writes of at least `len` bytes
/// - Both pointers are properly aligned
/// - The memory regions [src, src+len) and [dst, dst+len) do NOT overlap
pub unsafe fn copy_mem(dst: *mut u8, src: *const u8, len: usize) {
    if len == 0 {
        return;
    }
    ptr::copy_nonoverlapping(src, dst, len);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_zeroed() {
        let buffer = MLockedSizedBytes::<16>::new_zeroed().unwrap();
        assert_eq!(buffer.as_slice(), &[0u8; 16]);
    }

    #[test]
    fn clone_copies_contents() {
        let mut buffer = MLockedSizedBytes::<8>::new_zeroed().unwrap();
        buffer.as_mut_slice().copy_from_slice(b"DEADBEEF");
        let cloned = buffer.try_clone().unwrap();
        assert_eq!(cloned.as_slice(), b"DEADBEEF");
    }

    #[test]
    fn dynamic_allocate_and_clone() {
        let mut buffer = MLockedBytes::new(12).unwrap();
        buffer.as_mut_slice().fill(0xAA);
        let cloned = buffer.try_clone().unwrap();
        assert_eq!(cloned.as_slice(), &[0xAA; 12]);
    }

    #[test]
    fn aligned_allocation_rounds_up() {
        let buffer = MLockedBytes::new_aligned(13, 8).unwrap();
        assert_eq!(buffer.len(), 16);
    }

    #[test]
    fn zero_mem_clears_region() {
        let mut buffer = MLockedBytes::new(4).unwrap();
        buffer.as_mut_slice().copy_from_slice(&[1, 2, 3, 4]);
        unsafe { zero_mem(buffer.as_mut_ptr(), buffer.len()) };
        assert_eq!(buffer.as_slice(), &[0, 0, 0, 0]);
    }

    #[test]
    fn copy_mem_moves_bytes() {
        let mut src = MLockedBytes::new_zeroed(4).unwrap();
        src.as_mut_slice().copy_from_slice(&[9, 8, 7, 6]);
        let mut dst = MLockedBytes::new_zeroed(4).unwrap();
        unsafe { copy_mem(dst.as_mut_ptr(), src.as_ptr(), dst.len()) };
        assert_eq!(dst.as_slice(), &[9, 8, 7, 6]);
    }
}
