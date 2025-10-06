//! Internal debugging utilities for DSIGN and generator workflows.
//!
//! Logging is disabled by default so binaries remain quiet during normal
//! execution. Enable the `ed25519-debug` feature (and optionally set the
//! `CARDANO_ED25519_DEBUG` environment variable) to print diagnostic output.

#[cfg(feature = "ed25519-debug")]
use std::sync::OnceLock;

#[cfg(feature = "ed25519-debug")]
fn is_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("CARDANO_ED25519_DEBUG").is_ok())
}

#[cfg(not(feature = "ed25519-debug"))]
#[inline(always)]
fn is_enabled() -> bool {
    false
}

/// Emit a lazily constructed debug message when DSIGN debugging is enabled.
#[inline(always)]
pub fn log<F>(message: F)
where
    F: FnOnce() -> String,
{
    if is_enabled() {
        eprintln!("{}", message());
    }
}
