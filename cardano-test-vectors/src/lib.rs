//! Shared test vector fixtures for the cardano-base-rust workspace.
//!
//! This crate centralises the golden test vectors that were previously
//! duplicated across multiple packages. The files themselves are embedded at
//! compile time so consumers can rely on the fixtures without performing any
//! I/O at runtime or maintaining their own copies.

/// Internal debugging helpers mirroring the strategy used by `cardano-vrf-pure`.
/// Enable the `ed25519-debug` feature (and optionally set the
/// `CARDANO_ED25519_DEBUG` environment variable) to surface detailed logs.
pub mod debug;

/// VRF (Verifiable Random Function) fixtures originating from the Haskell
/// `cardano-base` repository.
pub mod vrf {
    /// Metadata describing an embedded VRF test vector file.
    #[derive(Clone, Copy, Debug)]
    pub struct TestVector {
        /// File name of the vector (for consistency with the upstream repo).
        pub name: &'static str,
        /// Raw file contents as UTF-8.
        pub contents: &'static str,
    }

    /// All embedded VRF test vectors.
    pub const ALL: &[TestVector] = &[
        TestVector {
            name: "vrf_ver03_generated_1",
            contents: include_str!("../test_vectors/vrf_ver03_generated_1"),
        },
        TestVector {
            name: "vrf_ver03_generated_2",
            contents: include_str!("../test_vectors/vrf_ver03_generated_2"),
        },
        TestVector {
            name: "vrf_ver03_generated_3",
            contents: include_str!("../test_vectors/vrf_ver03_generated_3"),
        },
        TestVector {
            name: "vrf_ver03_generated_4",
            contents: include_str!("../test_vectors/vrf_ver03_generated_4"),
        },
        TestVector {
            name: "vrf_ver03_standard_10",
            contents: include_str!("../test_vectors/vrf_ver03_standard_10"),
        },
        TestVector {
            name: "vrf_ver03_standard_11",
            contents: include_str!("../test_vectors/vrf_ver03_standard_11"),
        },
        TestVector {
            name: "vrf_ver03_standard_12",
            contents: include_str!("../test_vectors/vrf_ver03_standard_12"),
        },
        TestVector {
            name: "vrf_ver13_generated_1",
            contents: include_str!("../test_vectors/vrf_ver13_generated_1"),
        },
        TestVector {
            name: "vrf_ver13_generated_2",
            contents: include_str!("../test_vectors/vrf_ver13_generated_2"),
        },
        TestVector {
            name: "vrf_ver13_generated_3",
            contents: include_str!("../test_vectors/vrf_ver13_generated_3"),
        },
        TestVector {
            name: "vrf_ver13_generated_4",
            contents: include_str!("../test_vectors/vrf_ver13_generated_4"),
        },
        TestVector {
            name: "vrf_ver13_standard_10",
            contents: include_str!("../test_vectors/vrf_ver13_standard_10"),
        },
        TestVector {
            name: "vrf_ver13_standard_11",
            contents: include_str!("../test_vectors/vrf_ver13_standard_11"),
        },
        TestVector {
            name: "vrf_ver13_standard_12",
            contents: include_str!("../test_vectors/vrf_ver13_standard_12"),
        },
    ];

    /// Look up a VRF test vector by its file name.
    #[must_use]
    pub fn get(name: &str) -> Option<&'static str> {
        ALL.iter()
            .find(|vector| vector.name == name)
            .map(|vector| vector.contents)
    }

    /// Convenience helper that returns the list of vector names.
    #[must_use = "Iterate to consume the VRF vector names"]
    pub fn names() -> impl Iterator<Item = &'static str> {
        ALL.iter().map(|vector| vector.name)
    }
}

/// DSIGN (Digital Signature) fixtures extracted from the Haskell
/// `cardano-crypto-tests` repository.
pub mod dsign {
    /// Metadata describing an embedded DSIGN test vector file.
    #[derive(Clone, Copy, Debug)]
    pub struct TestVector {
        /// File name of the vector (for consistency with the upstream repo).
        pub name: &'static str,
        /// Raw file contents as JSON.
        pub contents: &'static str,
    }

    /// All embedded DSIGN test vectors.
    pub const ALL: &[TestVector] = &[
        TestVector {
            name: "ed25519_test_vectors.json",
            contents: include_str!("../test_vectors/ed25519_test_vectors.json"),
        },
        TestVector {
            name: "ecdsa_secp256k1_test_vectors.json",
            contents: include_str!("../test_vectors/ecdsa_secp256k1_test_vectors.json"),
        },
        TestVector {
            name: "schnorr_secp256k1_test_vectors.json",
            contents: include_str!("../test_vectors/schnorr_secp256k1_test_vectors.json"),
        },
    ];

    /// Look up a DSIGN test vector by its file name.
    #[must_use]
    pub fn get(name: &str) -> Option<&'static str> {
        ALL.iter()
            .find(|vector| vector.name == name)
            .map(|vector| vector.contents)
    }

    /// Convenience helper that returns the list of vector names.
    #[must_use = "Iterate to consume the DSIGN vector names"]
    pub fn names() -> impl Iterator<Item = &'static str> {
        ALL.iter().map(|vector| vector.name)
    }
}

/// KES (Key Evolving Signature) fixtures derived from deterministic Rust generation.
pub mod kes {
    /// Metadata describing an embedded KES test vector file.
    #[derive(Clone, Copy, Debug)]
    pub struct TestVector {
        /// File name of the vector (for consistency with the upstream repo).
        pub name: &'static str,
        /// Raw file contents as JSON.
        pub contents: &'static str,
    }

    /// All embedded KES test vectors.
    pub const ALL: &[TestVector] = &[
        TestVector {
            name: "single_kes_test_vectors.json",
            contents: include_str!("../test_vectors/single_kes_test_vectors.json"),
        },
        TestVector {
            name: "compact_single_kes_test_vectors.json",
            contents: include_str!("../test_vectors/compact_single_kes_test_vectors.json"),
        },
        TestVector {
            name: "sum_kes_test_vectors.json",
            contents: include_str!("../test_vectors/sum_kes_test_vectors.json"),
        },
        TestVector {
            name: "compact_sum_kes_test_vectors.json",
            contents: include_str!("../test_vectors/compact_sum_kes_test_vectors.json"),
        },
        TestVector {
            name: "sum_kes_period_evolution_vectors.json",
            contents: include_str!("../test_vectors/sum_kes_period_evolution_vectors.json"),
        },
        TestVector {
            name: "compact_sum_kes_period_evolution_vectors.json",
            contents: include_str!("../test_vectors/compact_sum_kes_period_evolution_vectors.json"),
        },
    ];

    /// Look up a KES test vector by its file name.
    #[must_use]
    pub fn get(name: &str) -> Option<&'static str> {
        ALL.iter()
            .find(|vector| vector.name == name)
            .map(|vector| vector.contents)
    }

    /// Convenience helper that returns the list of vector names.
    #[must_use = "Iterate to consume the KES vector names"]
    pub fn names() -> impl Iterator<Item = &'static str> {
        ALL.iter().map(|vector| vector.name)
    }
}

/// BLS12-381 (pairings and signature operations) fixtures from the Haskell
/// `cardano-crypto-tests` repository.
pub mod bls12_381 {
    /// Metadata describing an embedded BLS12-381 test vector file.
    #[derive(Clone, Copy, Debug)]
    pub struct TestVector {
        /// File name of the vector (matching the upstream naming convention).
        pub name: &'static str,
        /// Raw file contents as plain text.
        pub contents: &'static str,
    }

    /// All embedded BLS12-381 test vectors.
    pub const ALL: &[TestVector] = &[
        TestVector {
            name: "bls_sig_aug_test_vectors",
            contents: include_str!("../test_vectors/bls12-381/bls_sig_aug_test_vectors"),
        },
        TestVector {
            name: "ec_operations_test_vectors",
            contents: include_str!("../test_vectors/bls12-381/ec_operations_test_vectors"),
        },
        TestVector {
            name: "h2c_large_dst",
            contents: include_str!("../test_vectors/bls12-381/h2c_large_dst"),
        },
        TestVector {
            name: "pairing_test_vectors",
            contents: include_str!("../test_vectors/bls12-381/pairing_test_vectors"),
        },
        TestVector {
            name: "serde_test_vectors",
            contents: include_str!("../test_vectors/bls12-381/serde_test_vectors"),
        },
    ];

    /// Look up a BLS12-381 test vector by its file name.
    #[must_use]
    pub fn get(name: &str) -> Option<&'static str> {
        ALL.iter()
            .find(|vector| vector.name == name)
            .map(|vector| vector.contents)
    }

    /// Convenience helper that returns the list of vector names.
    #[must_use = "Iterate to consume the BLS12-381 vector names"]
    pub fn names() -> impl Iterator<Item = &'static str> {
        ALL.iter().map(|vector| vector.name)
    }
}
