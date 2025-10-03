//! Trait glue mirroring the Haskell `orphans-deriving-via` package.
//!
//! In the original code base this package only supplied orphan instances that
//! connected `base-deriving-via` with the `deepseq` and `nothunks` libraries.
//! Rust's trait coherence rules prevent those implementations from living in a
//! third crate, so the actual trait impls live alongside the traits
//! themselves. This crate exists to preserve the package boundary from the
//! Haskell project and re-exports the relevant functionality.

pub use deepseq::{deepseq, force, NFData};
pub use nothunks::{no_thunks, unsafe_no_thunks, NoThunks, NoThunksResult, ThunkInfo};

pub mod prelude {
    //! Convenience re-exports mirroring the Haskell module hierarchy.
    pub use super::{deepseq, force, no_thunks, unsafe_no_thunks, NFData, NoThunks};
    pub use base_deriving_via::{impl_generic_for_struct, InstantiatedAt};
}

#[cfg(test)]
mod tests {
    use base_deriving_via::{impl_generic_for_struct, InstantiatedAt};
    use deepseq::force;
    use nothunks::unsafe_no_thunks;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Metrics {
        count: u64,
        name: String,
    }

    impl_generic_for_struct!(
        struct Metrics {
            count: u64,
            name: String,
        }
    );

    #[test]
    fn instantiated_at_implements_nfdata_via_generic_repr() {
        let metrics = InstantiatedAt::new(Metrics {
            count: 10,
            name: "example".to_owned(),
        });

        let forced = force(metrics);
        assert_eq!(forced.as_ref().count, 10);
        assert_eq!(forced.as_ref().name, "example");
    }

    #[test]
    fn instantiated_at_is_no_thunks_when_fields_are() {
        let metrics = InstantiatedAt::new(Metrics {
            count: 10,
            name: "example".to_owned(),
        });

        assert!(unsafe_no_thunks(&metrics).is_none());
    }
}
