use base_deriving_via::{InstantiatedAt, Monoid, Semigroup, impl_generic_for_struct};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Metrics {
    total: i64,
    label: String,
    samples: Vec<u32>,
}

impl_generic_for_struct!(
    struct Metrics {
        total: i64,
        label: String,
        samples: Vec<u32>,
    }
);

#[test]
fn combines_record_fields_componentwise() {
    let left = InstantiatedAt::new(Metrics {
        total: 21,
        label: "foo".to_owned(),
        samples: vec![1, 2, 3],
    });
    let right = InstantiatedAt::new(Metrics {
        total: 21,
        label: "bar".to_owned(),
        samples: vec![4, 5],
    });

    let combined = left.combine(right).into_inner();

    assert_eq!(combined.total, 42);
    assert_eq!(combined.label, "foobar");
    assert_eq!(combined.samples, vec![1, 2, 3, 4, 5]);
}

#[test]
fn monoid_identity_behaves_as_expected() {
    let value = InstantiatedAt::new(Metrics {
        total: 7,
        label: "hello".to_owned(),
        samples: vec![10, 11],
    });

    let identity = InstantiatedAt::<Metrics>::empty();
    let left = value.clone().combine(identity.clone()).into_inner();
    let right = identity.combine(value.clone()).into_inner();

    assert_eq!(left, value.clone().into_inner());
    assert_eq!(right, value.into_inner());
}
