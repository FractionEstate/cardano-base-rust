//! Property-based tests for CBOR serialization roundtrips
//!
//! These tests use proptest to generate random inputs and verify
//! that serialization followed by deserialization returns the original value.

use cardano_binary::{decode_full, serialize};
use proptest::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SimpleStruct {
    field1: u32,
    field2: String,
    field3: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum SimpleEnum {
    Variant1,
    Variant2(u64),
    Variant3 { name: String, value: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct NestedStruct {
    inner: SimpleStruct,
    values: Vec<u8>,
    optional: Option<String>,
}

proptest! {
    /// Test that serialization and deserialization roundtrips for simple structs
    #[test]
    fn roundtrip_simple_struct(
        field1 in any::<u32>(),
        field2 in ".*",
        field3 in any::<bool>()
    ) {
        let original = SimpleStruct { field1, field2, field3 };
        let bytes = serialize(&original).expect("serialization failed");
        let decoded: SimpleStruct = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(original, decoded);
    }

    /// Test that u64 values roundtrip correctly
    #[test]
    fn roundtrip_u64(value in any::<u64>()) {
        let bytes = serialize(&value).expect("serialization failed");
        let decoded: u64 = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(value, decoded);
    }

    /// Test that i64 values roundtrip correctly
    #[test]
    fn roundtrip_i64(value in any::<i64>()) {
        let bytes = serialize(&value).expect("serialization failed");
        let decoded: i64 = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(value, decoded);
    }

    /// Test that strings roundtrip correctly
    #[test]
    fn roundtrip_string(value in ".*") {
        let bytes = serialize(&value).expect("serialization failed");
        let decoded: String = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(value, decoded);
    }

    /// Test that byte vectors roundtrip correctly
    #[test]
    fn roundtrip_bytes(value in prop::collection::vec(any::<u8>(), 0..1000)) {
        let bytes = serialize(&value).expect("serialization failed");
        let decoded: Vec<u8> = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(value, decoded);
    }

    /// Test that Options roundtrip correctly
    #[test]
    fn roundtrip_option(value in proptest::option::of(any::<u32>())) {
        let bytes = serialize(&value).expect("serialization failed");
        let decoded: Option<u32> = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(value, decoded);
    }

    /// Test that tuples roundtrip correctly
    #[test]
    fn roundtrip_tuple(
        a in any::<u32>(),
        b in ".*",
        c in any::<bool>()
    ) {
        let value = (a, b, c);
        let bytes = serialize(&value).expect("serialization failed");
        let decoded: (u32, String, bool) = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(value, decoded);
    }

    /// Test that nested structures roundtrip correctly
    #[test]
    fn roundtrip_nested_struct(
        field1 in any::<u32>(),
        field2 in ".*",
        field3 in any::<bool>(),
        values in prop::collection::vec(any::<u8>(), 0..100),
        optional in proptest::option::of(".*")
    ) {
        let original = NestedStruct {
            inner: SimpleStruct { field1, field2, field3 },
            values,
            optional,
        };
        let bytes = serialize(&original).expect("serialization failed");
        let decoded: NestedStruct = decode_full(&bytes).expect("deserialization failed");
        prop_assert_eq!(original, decoded);
    }
}

/// Additional deterministic tests for enum variants
#[test]
fn roundtrip_enum_variant1() {
    let original = SimpleEnum::Variant1;
    let bytes = serialize(&original).expect("serialization failed");
    let decoded: SimpleEnum = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(original, decoded);
}

#[test]
fn roundtrip_enum_variant2() {
    let original = SimpleEnum::Variant2(12345);
    let bytes = serialize(&original).expect("serialization failed");
    let decoded: SimpleEnum = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(original, decoded);
}

#[test]
fn roundtrip_enum_variant3() {
    let original = SimpleEnum::Variant3 {
        name: "test".to_string(),
        value: -42,
    };
    let bytes = serialize(&original).expect("serialization failed");
    let decoded: SimpleEnum = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(original, decoded);
}
