//! Pure Rust implementation of VRF (Verifiable Random Function) for Cardano
//!
//! This crate provides a 100% Rust implementation of the VRF algorithms used in Cardano,
//! following the IETF specifications:
//! - ECVRF-ED25519-SHA512-Elligator2 (draft-03)
//! - ECVRF-ED25519-SHA512-TAI (draft-13 batch-compatible)
//!
//! All implementations are memory-safe and use constant-time operations where appropriate
//! to prevent timing attacks.

// Allow some clippy lints for cryptographic code
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![cfg_attr(test, allow(clippy::unwrap_used))]
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod common;
pub mod draft03;
pub mod draft13;

pub use draft03::VrfDraft03;
pub use draft13::VrfDraft13;

/// Error types for VRF operations
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum VrfError {
    /// Invalid proof provided
    #[error("Invalid VRF proof")]
    InvalidProof,

    /// Invalid public key
    #[error("Invalid public key")]
    InvalidPublicKey,

    /// Invalid secret key
    #[error("Invalid secret key")]
    InvalidSecretKey,

    /// Invalid point encoding
    #[error("Invalid point encoding")]
    InvalidPoint,

    /// Invalid scalar encoding
    #[error("Invalid scalar encoding")]
    InvalidScalar,

    /// Verification failed
    #[error("VRF verification failed")]
    VerificationFailed,
}

/// Result type for VRF operations
pub type VrfResult<T> = Result<T, VrfError>;
