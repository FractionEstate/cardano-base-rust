use std::marker::PhantomData;
use std::ptr::NonNull;

/// Pointer tagged with a compile-time size guaranteeing the backing region
/// contains exactly `N` bytes. The lifetime ensures the pointer cannot outlive
/// the allocation it references.
#[derive(Clone, Copy)]
pub struct SizedPtr<'a, const N: usize> {
    ptr: NonNull<u8>,
    _marker: PhantomData<&'a [u8; N]>,
}

impl<'a, const N: usize> SizedPtr<'a, N> {
    pub(crate) fn new(ptr: NonNull<u8>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Returns the raw pointer.
    pub fn as_ptr(self) -> *const u8 {
        self.ptr.as_ptr()
    }

    /// Returns the pointer as mutable. Callers must uphold aliasing rules.
    pub fn as_mut_ptr(self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Number of bytes referenced by this pointer.
    pub const fn len(self) -> usize {
        N
    }
}

/// Mutable sized pointer variant.
#[derive(Clone, Copy)]
pub struct SizedMutPtr<'a, const N: usize> {
    ptr: NonNull<u8>,
    _marker: PhantomData<&'a mut [u8; N]>,
}

impl<'a, const N: usize> SizedMutPtr<'a, N> {
    pub(crate) fn new(ptr: NonNull<u8>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(self) -> *const u8 {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    pub const fn len(self) -> usize {
        N
    }
}
