//! Golden tests for CBOR serialization format stability
//!
//! These tests ensure that the CBOR encoding format remains stable across versions.
//! If these tests fail after a code change, it indicates a breaking change in serialization format.

use cardano_binary::{decode_full, serialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TestStruct {
    name: String,
    value: u32,
    flag: bool,
}

/// Test that a simple u64 serializes to the expected CBOR bytes
#[test]
fn golden_u64_42() {
    let value: u64 = 42;
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of unsigned integer 42 is: 0x18, 0x2a
    assert_eq!(bytes, vec![0x18, 0x2a], "CBOR format changed for u64");
    
    // Verify roundtrip
    let decoded: u64 = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, 42);
}

/// Test that a small u64 serializes to the expected CBOR bytes
#[test]
fn golden_u64_small() {
    let value: u64 = 10;
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of unsigned integer 10 is: 0x0a
    assert_eq!(bytes, vec![0x0a], "CBOR format changed for small u64");
    
    let decoded: u64 = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, 10);
}

/// Test that a simple string serializes to the expected CBOR bytes
#[test]
fn golden_string() {
    let value = "hello";
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of "hello" is:
    // 0x65 (text string of length 5)
    // followed by "hello" in UTF-8
    assert_eq!(bytes[0], 0x65, "CBOR format changed for string length");
    assert_eq!(&bytes[1..], b"hello", "CBOR format changed for string content");
    
    let decoded: String = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, "hello");
}

/// Test that an empty array serializes to the expected CBOR bytes
#[test]
fn golden_empty_array() {
    let value: Vec<u32> = vec![];
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of empty array is: 0x80
    assert_eq!(bytes, vec![0x80], "CBOR format changed for empty array");
    
    let decoded: Vec<u32> = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, vec![]);
}

/// Test that a simple array serializes to the expected CBOR bytes
#[test]
fn golden_array_1_2_3() {
    let value: Vec<u8> = vec![1, 2, 3];
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of [1, 2, 3] is:
    // 0x83 (array of length 3)
    // 0x01, 0x02, 0x03 (the three integers)
    assert_eq!(bytes, vec![0x83, 0x01, 0x02, 0x03], "CBOR format changed for simple array");
    
    let decoded: Vec<u8> = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, vec![1, 2, 3]);
}

/// Test that None serializes to the expected CBOR bytes
#[test]
fn golden_option_none() {
    let value: Option<u32> = None;
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of null is: 0xf6
    assert_eq!(bytes, vec![0xf6], "CBOR format changed for None");
    
    let decoded: Option<u32> = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, None);
}

/// Test that Some(value) serializes to the expected CBOR bytes
#[test]
fn golden_option_some() {
    let value: Option<u32> = Some(42);
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of Some(42) is just the encoding of 42: 0x18, 0x2a
    assert_eq!(bytes, vec![0x18, 0x2a], "CBOR format changed for Some");
    
    let decoded: Option<u32> = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, Some(42));
}

/// Test that true serializes to the expected CBOR bytes
#[test]
fn golden_bool_true() {
    let value = true;
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of true is: 0xf5
    assert_eq!(bytes, vec![0xf5], "CBOR format changed for true");
    
    let decoded: bool = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, true);
}

/// Test that false serializes to the expected CBOR bytes
#[test]
fn golden_bool_false() {
    let value = false;
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of false is: 0xf4
    assert_eq!(bytes, vec![0xf4], "CBOR format changed for false");
    
    let decoded: bool = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, false);
}

/// Test that byte arrays serialize with the correct CBOR type
#[test]
fn golden_bytes() {
    let value = vec![0xde, 0xad, 0xbe, 0xef];
    let bytes = serialize(&value).expect("serialization failed");
    
    // Should start with 0x84 (array of 4 elements) since Vec<u8> serializes as array
    assert_eq!(bytes[0], 0x84, "CBOR format changed for byte array");
    
    let decoded: Vec<u8> = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, vec![0xde, 0xad, 0xbe, 0xef]);
}

/// Test that a struct serializes to a consistent CBOR map
#[test]
fn golden_struct() {
    let value = TestStruct {
        name: "test".to_string(),
        value: 100,
        flag: true,
    };
    let bytes = serialize(&value).expect("serialization failed");
    
    // Struct should serialize as CBOR map (starts with 0xa3 for map of 3 entries)
    assert_eq!(bytes[0], 0xa3, "CBOR format changed for struct (should be map)");
    
    // Verify roundtrip
    let decoded: TestStruct = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, value);
}

/// Test that negative integers serialize correctly
#[test]
fn golden_negative_int() {
    let value: i32 = -42;
    let bytes = serialize(&value).expect("serialization failed");
    
    // CBOR encoding of -42 is: 0x38, 0x29 (negative integer 41, which represents -42)
    assert_eq!(bytes, vec![0x38, 0x29], "CBOR format changed for negative integer");
    
    let decoded: i32 = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, -42);
}

/// Test that tuples serialize as CBOR arrays
#[test]
fn golden_tuple() {
    let value = (1u8, 2u8, 3u8);
    let bytes = serialize(&value).expect("serialization failed");
    
    // Tuple should serialize as array: 0x83 (array of 3), 0x01, 0x02, 0x03
    assert_eq!(bytes, vec![0x83, 0x01, 0x02, 0x03], "CBOR format changed for tuple");
    
    let decoded: (u8, u8, u8) = decode_full(&bytes).expect("deserialization failed");
    assert_eq!(decoded, (1, 2, 3));
}
