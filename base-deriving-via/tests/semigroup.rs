use base_deriving_via::{Monoid, Semigroup};
use core::time::Duration;

#[test]
fn tuples_combine_componentwise() {
    let left = (
        String::from("foo"),
        21_i64,
        vec![1_u32, 2, 3],
        Duration::from_millis(50),
    );
    let right = (
        String::from("bar"),
        21_i64,
        vec![4_u32, 5],
        Duration::from_millis(10),
    );

    let combined = Semigroup::combine(left, right);

    assert_eq!(combined.0, "foobar");
    assert_eq!(combined.1, 42);
    assert_eq!(combined.2, vec![1, 2, 3, 4, 5]);
    assert_eq!(combined.3, Duration::from_millis(60));
}

#[test]
fn tuples_respect_monoid_identity() {
    let value = (
        String::from("baz"),
        7_i64,
        vec![9_u8],
        Duration::from_millis(5),
    );

    let identity: (String, i64, Vec<u8>, Duration) = Monoid::empty();

    let left = Semigroup::combine(value.clone(), identity.clone());
    let right = Semigroup::combine(identity, value.clone());

    assert_eq!(left, value);
    assert_eq!(right, value);
}
