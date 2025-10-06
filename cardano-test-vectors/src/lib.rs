//! Shared test vector fixtures for the cardano-base-rust workspace.
//!
//! This crate centralises the golden test vectors that were previously
//! duplicated across multiple packages. The files themselves are embedded at
//! compile time so consumers can rely on the fixtures without performing any
//! I/O at runtime or maintaining their own copies.

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
    #[must_use]
    pub fn names() -> impl Iterator<Item = &'static str> {
        ALL.iter().map(|vector| vector.name)
    }
}
