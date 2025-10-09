//! Feature-gated lightweight instrumentation for mlocked memory allocations.
//!
//! Enabled via the `mlocked-metrics` feature. Provides approximate counters for:
//! - Successful allocations (count + total bytes rounded up for alignment)
//! - Failed lock attempts
//! - Zeroisations on drop (number of regions wiped)
//!
//! Mirrors the style of `kes::metrics` to keep a uniform instrumentation pattern.
//! All counters are global, monotonic for the life of the process, and use relaxed
//! ordering to minimise overhead. Intended only for diagnostics / benchmarking
//! builds; not a security boundary.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MLockedMetrics {
    pub allocations: u64,
    pub allocation_bytes: u64,
    pub failed_locks: u64,
    pub zeroizations: u64,
}

#[cfg(feature = "mlocked-metrics")]
use core::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "mlocked-metrics")]
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "mlocked-metrics")]
static ALLOCATION_BYTES: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "mlocked-metrics")]
static FAILED_LOCKS: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "mlocked-metrics")]
static ZEROIZATIONS: AtomicU64 = AtomicU64::new(0);

#[cfg(feature = "mlocked-metrics")]
#[inline]
pub(crate) fn record_allocation(size: usize) {
    ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    ALLOCATION_BYTES.fetch_add(size as u64, Ordering::Relaxed);
}

#[cfg(feature = "mlocked-metrics")]
#[inline]
pub(crate) fn record_failed_lock() {
    FAILED_LOCKS.fetch_add(1, Ordering::Relaxed);
}

#[cfg(feature = "mlocked-metrics")]
#[inline]
pub(crate) fn record_zeroization() {
    ZEROIZATIONS.fetch_add(1, Ordering::Relaxed);
}

/// Obtain a metrics snapshot. Returns zeros when the feature is disabled.
#[inline]
pub fn snapshot() -> MLockedMetrics {
    #[cfg(feature = "mlocked-metrics")]
    {
        MLockedMetrics {
            allocations: ALLOCATIONS.load(Ordering::Relaxed),
            allocation_bytes: ALLOCATION_BYTES.load(Ordering::Relaxed),
            failed_locks: FAILED_LOCKS.load(Ordering::Relaxed),
            zeroizations: ZEROIZATIONS.load(Ordering::Relaxed),
        }
    }
    #[cfg(not(feature = "mlocked-metrics"))]
    {
        MLockedMetrics::default()
    }
}

#[cfg(all(test, not(feature = "mlocked-metrics")))]
mod tests_disabled {
    use super::*;
    // When the feature is disabled the snapshot must be all zeros.
    #[test]
    fn snapshot_zeros_without_feature() {
        let snap = snapshot();
        assert_eq!(snap, MLockedMetrics::default());
    }
}
