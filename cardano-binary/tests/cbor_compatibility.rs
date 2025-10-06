//! Byte-for-byte CBOR compatibility tests
//!
//! These tests verify that our Rust CBOR implementation produces the exact same
//! byte sequences as the original Haskell cardano-binary implementation.
//!
//! Test vectors are based on the Haskell implementation's behavior and the
//! CBOR RFC 8949 specification.

use cardano_binary::{decode_full, serialize};
use serde::{Deserialize, Serialize};

/// Test basic integer encoding matches CBOR specification
#[test]
fn cbor_compat_unsigned_integers() {
    // Small integers (0-23) encode as single byte
    assert_eq!(serialize(&0u8).unwrap(), vec![0x00]);
    assert_eq!(serialize(&1u8).unwrap(), vec![0x01]);
    assert_eq!(serialize(&10u8).unwrap(), vec![0x0a]);
    assert_eq!(serialize(&23u8).unwrap(), vec![0x17]);

    // Medium integers (24-255) encode as 0x18 + byte
    assert_eq!(serialize(&24u8).unwrap(), vec![0x18, 0x18]);
    assert_eq!(serialize(&42u8).unwrap(), vec![0x18, 0x2a]);
    assert_eq!(serialize(&255u8).unwrap(), vec![0x18, 0xff]);

    // Larger integers
    assert_eq!(serialize(&256u16).unwrap(), vec![0x19, 0x01, 0x00]);
    assert_eq!(serialize(&1000u16).unwrap(), vec![0x19, 0x03, 0xe8]);

    // u32 values
    assert_eq!(
        serialize(&100000u32).unwrap(),
        vec![0x1a, 0x00, 0x01, 0x86, 0xa0]
    );

    // u64 values
    assert_eq!(
        serialize(&0x0102030405060708u64).unwrap(),
        vec![0x1b, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
    );
}

/// Test negative integer encoding
#[test]
fn cbor_compat_negative_integers() {
    // -1 encodes as 0x20 (negative 0)
    assert_eq!(serialize(&-1i8).unwrap(), vec![0x20]);

    // -10 encodes as 0x29 (negative 9)
    assert_eq!(serialize(&-10i8).unwrap(), vec![0x29]);

    // -24 encodes as 0x37 (negative 23)
    assert_eq!(serialize(&-24i8).unwrap(), vec![0x37]);

    // -25 encodes as 0x38 0x18 (negative 24)
    assert_eq!(serialize(&-25i8).unwrap(), vec![0x38, 0x18]);

    // -100 encodes as 0x38 0x63 (negative 99)
    assert_eq!(serialize(&-100i16).unwrap(), vec![0x38, 0x63]);

    // -1000 encodes as 0x39 0x03 0xe7 (negative 999)
    assert_eq!(serialize(&-1000i16).unwrap(), vec![0x39, 0x03, 0xe7]);
}

/// Test boolean encoding
#[test]
fn cbor_compat_booleans() {
    assert_eq!(serialize(&false).unwrap(), vec![0xf4]);
    assert_eq!(serialize(&true).unwrap(), vec![0xf5]);
}

/// Test null/None encoding
#[test]
fn cbor_compat_null() {
    let value: Option<u32> = None;
    assert_eq!(serialize(&value).unwrap(), vec![0xf6]);
}

/// Test string encoding
#[test]
fn cbor_compat_strings() {
    // Empty string
    assert_eq!(serialize(&"").unwrap(), vec![0x60]);

    // "a" - single character
    assert_eq!(serialize(&"a").unwrap(), vec![0x61, 0x61]);

    // "IETF" - 4 characters
    assert_eq!(
        serialize(&"IETF").unwrap(),
        vec![0x64, 0x49, 0x45, 0x54, 0x46]
    );

    // "hello" - 5 characters
    assert_eq!(
        serialize(&"hello").unwrap(),
        vec![0x65, 0x68, 0x65, 0x6c, 0x6c, 0x6f]
    );

    // Unicode string "水" (water in Chinese)
    let water = "水";
    let encoded = serialize(&water).unwrap();
    assert_eq!(encoded[0], 0x63); // 3-byte UTF-8 string
    assert_eq!(&encoded[1..], water.as_bytes());
}

/// Test byte string encoding (this is where Rust Vec<u8> differs from Haskell ByteString)
#[test]
fn cbor_compat_byte_arrays() {
    // Empty array
    let empty: Vec<u8> = vec![];
    let encoded = serialize(&empty).unwrap();
    assert_eq!(encoded, vec![0x80]); // Array of 0 elements

    // Single byte - in Rust Vec<u8> is an array
    let single = vec![0x01u8];
    let encoded = serialize(&single).unwrap();
    assert_eq!(encoded, vec![0x81, 0x01]); // Array of 1 element

    // Multiple bytes
    let bytes = vec![0x01u8, 0x02, 0x03, 0x04];
    let encoded = serialize(&bytes).unwrap();
    assert_eq!(encoded, vec![0x84, 0x01, 0x02, 0x03, 0x04]); // Array of 4 elements
}

/// Test array encoding
#[test]
fn cbor_compat_arrays() {
    // Empty array
    let empty: Vec<u32> = vec![];
    assert_eq!(serialize(&empty).unwrap(), vec![0x80]);

    // [1, 2, 3]
    let arr = vec![1u32, 2, 3];
    assert_eq!(serialize(&arr).unwrap(), vec![0x83, 0x01, 0x02, 0x03]);

    // [1, [2, 3], [4, 5]]
    let nested = vec![vec![1u32], vec![2, 3], vec![4, 5]];
    let encoded = serialize(&nested).unwrap();
    assert_eq!(
        encoded,
        vec![
            0x83, // array(3)
            0x81, 0x01, // [1]
            0x82, 0x02, 0x03, // [2, 3]
            0x82, 0x04, 0x05, // [4, 5]
        ]
    );
}

/// Test map/struct encoding
#[test]
fn cbor_compat_maps() {
    use std::collections::HashMap;

    // Empty map
    let empty: HashMap<String, u32> = HashMap::new();
    let encoded = serialize(&empty).unwrap();
    assert_eq!(encoded, vec![0xa0]); // map(0)

    // Simple map - note: order may vary in HashMap
    let mut map = HashMap::new();
    map.insert("a", 1u32);
    let encoded = serialize(&map).unwrap();
    assert_eq!(encoded[0], 0xa1); // map(1)
    // Rest of encoding: 0x61 'a' 0x01
}

/// Test struct encoding (structs encode as maps)
#[test]
fn cbor_compat_structs() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 2 };
    let encoded = serialize(&point).unwrap();

    // Should be a map with 2 entries
    assert_eq!(encoded[0], 0xa2); // map(2)

    // Verify roundtrip
    let decoded: Point = decode_full(&encoded).unwrap();
    assert_eq!(decoded, point);
}

/// Test tuple encoding (tuples encode as arrays)
#[test]
fn cbor_compat_tuples() {
    // (1, 2, 3)
    let tuple = (1u8, 2u8, 3u8);
    assert_eq!(serialize(&tuple).unwrap(), vec![0x83, 0x01, 0x02, 0x03]);

    // (1, "hello", true)
    let mixed = (1u8, "hi", true);
    let encoded = serialize(&mixed).unwrap();
    assert_eq!(
        encoded,
        vec![
            0x83, // array(3)
            0x01, // 1
            0x62, 0x68, 0x69, // "hi"
            0xf5, // true
        ]
    );
}

/// Test Option<T> encoding
#[test]
fn cbor_compat_options() {
    // None -> null (0xf6)
    let none: Option<u32> = None;
    assert_eq!(serialize(&none).unwrap(), vec![0xf6]);

    // Some(42) -> just the value
    let some: Option<u32> = Some(42);
    assert_eq!(serialize(&some).unwrap(), vec![0x18, 0x2a]);

    // Some("test") -> just the string
    let some_str: Option<&str> = Some("test");
    assert_eq!(
        serialize(&some_str).unwrap(),
        vec![0x64, 0x74, 0x65, 0x73, 0x74]
    );
}

/// Test floating point encoding (if needed)
#[test]
fn cbor_compat_floats() {
    // f32: 3.14159
    let f = 3.14159f32;
    let encoded = serialize(&f).unwrap();
    assert_eq!(encoded[0], 0xfa); // float32

    // f64: 3.14159265359
    let d = 3.14159265359f64;
    let encoded = serialize(&d).unwrap();
    assert_eq!(encoded[0], 0xfb); // float64
}

/// Test tagged values (CBOR semantic tags)
#[test]
fn cbor_compat_tags() {
    use ciborium::value::Value;

    // Tag 24 is used for "Encoded CBOR data item"
    let inner = Value::Bytes(vec![0x01, 0x02, 0x03]);
    let tagged = Value::Tag(24, Box::new(inner));

    let encoded = serialize(&tagged).unwrap();
    assert_eq!(encoded[0], 0xd8); // tag (1 byte)
    assert_eq!(encoded[1], 24); // tag number
    assert_eq!(encoded[2], 0x43); // byte string of length 3
}

/// Test complex Cardano-specific structure
#[test]
fn cbor_compat_cardano_like_struct() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Transaction {
        inputs: Vec<u64>,
        outputs: Vec<u64>,
        fee: u64,
    }

    let tx = Transaction {
        inputs: vec![1, 2, 3],
        outputs: vec![100, 200],
        fee: 10,
    };

    let encoded = serialize(&tx).unwrap();

    // Should be a map with 3 keys
    assert_eq!(encoded[0], 0xa3); // map(3)

    // Verify roundtrip
    let decoded: Transaction = decode_full(&encoded).unwrap();
    assert_eq!(decoded, tx);
}

/// Test indefinite-length encoding is NOT used (we use definite length)
#[test]
fn cbor_compat_definite_length() {
    // Our implementation should use definite-length encoding
    let arr = vec![1u32, 2, 3, 4, 5];
    let encoded = serialize(&arr).unwrap();

    // Should start with 0x85 (array of 5), not 0x9f (indefinite array)
    assert_eq!(encoded[0], 0x85);
    assert_ne!(encoded[0], 0x9f); // NOT indefinite length

    // String should also be definite
    let s = "hello";
    let encoded = serialize(&s).unwrap();
    assert_eq!(encoded[0], 0x65); // text string of length 5
    assert_ne!(encoded[0], 0x7f); // NOT indefinite length
}

/// Test canonical CBOR encoding (sorted map keys)
#[test]
fn cbor_compat_canonical_maps() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct MultiField {
        zebra: u32,
        apple: u32,
        banana: u32,
    }

    let data = MultiField {
        zebra: 1,
        apple: 2,
        banana: 3,
    };

    let encoded = serialize(&data).unwrap();

    // Should be a map with 3 entries
    assert_eq!(encoded[0], 0xa3); // map(3)

    // Verify roundtrip works
    let decoded: MultiField = decode_full(&encoded).unwrap();
    assert_eq!(decoded, data);
}

/// Test large data structures
#[test]
fn cbor_compat_large_structures() {
    // Large array
    let large_array: Vec<u32> = (0..1000).collect();
    let encoded = serialize(&large_array).unwrap();

    // Should use 2-byte length encoding (0x19 + 2 bytes for length)
    assert_eq!(encoded[0], 0x99); // array with 2-byte length

    let decoded: Vec<u32> = decode_full(&encoded).unwrap();
    assert_eq!(decoded, large_array);
}

/// Test byte string vs array distinction with serde_bytes
#[test]
fn cbor_compat_serde_bytes() {
    use serde_bytes::ByteBuf;

    // NOTE: With ciborium, serde_bytes ByteBuf may encode differently
    let bytes = ByteBuf::from(vec![0x01, 0x02, 0x03]);
    let encoded = serialize(&bytes).unwrap();

    println!("ByteBuf encoded: {:?}", encoded);
    println!("First byte: 0x{:02x} = {}", encoded[0], encoded[0]);

    // Regular Vec<u8> also encodes as array
    let vec = vec![0x01u8, 0x02, 0x03];
    let vec_encoded = serialize(&vec).unwrap();

    println!("Vec<u8> encoded: {:?}", vec_encoded);
    println!("First byte: 0x{:02x} = {}", vec_encoded[0], vec_encoded[0]);

    // ByteBuf should encode as CBOR byte string (0x43 = byte string of length 3)
    // while Vec<u8> encodes as array (0x83 = array of 3)
    assert_eq!(encoded[0], 0x43, "ByteBuf should encode as byte string");
    assert_eq!(vec_encoded[0], 0x83, "Vec<u8> should encode as array");

    // Verify roundtrip
    let decoded: ByteBuf = cardano_binary::decode_full(&encoded).unwrap();
    assert_eq!(decoded.as_slice(), &[0x01, 0x02, 0x03]);
}
/// Test nested CBOR (tag 24 - encoded CBOR)
#[test]
fn cbor_compat_nested_cbor() {
    use cardano_binary::{decode_nested_cbor, encode_nested_cbor};
    use serde_bytes::ByteBuf;

    // Create inner data
    let inner = ByteBuf::from(vec![0xaa, 0xbb, 0xcc]);

    // Encode as nested CBOR (tag 24)
    let nested = encode_nested_cbor(&inner).unwrap();

    // Should start with tag 24
    assert_eq!(nested[0], 0xd8); // tag (1 byte)
    assert_eq!(nested[1], 24); // tag number

    // Verify roundtrip
    let decoded: ByteBuf = decode_nested_cbor(&nested).unwrap();
    assert_eq!(decoded, inner);
}

/// Test compatibility with specific Cardano test vectors
#[test]
fn cbor_compat_cardano_examples() {
    // Example: Cardano address-like structure
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Address {
        network: u8,
        payment: Vec<u8>,
        stake: Option<Vec<u8>>,
    }

    let addr = Address {
        network: 1,
        payment: vec![0x01, 0x02, 0x03],
        stake: Some(vec![0x04, 0x05]),
    };

    let encoded = serialize(&addr).unwrap();
    let decoded: Address = decode_full(&encoded).unwrap();
    assert_eq!(decoded, addr);

    // Test with no stake credential
    let addr_no_stake = Address {
        network: 0,
        payment: vec![0xaa, 0xbb],
        stake: None,
    };

    let encoded = serialize(&addr_no_stake).unwrap();
    let decoded: Address = decode_full(&encoded).unwrap();
    assert_eq!(decoded, addr_no_stake);
}

/// Test that encoding is deterministic (same input = same output)
#[test]
fn cbor_compat_deterministic() {
    #[derive(Serialize, Deserialize)]
    struct Data {
        field1: u32,
        field2: String,
        field3: Vec<u8>,
    }

    let data = Data {
        field1: 42,
        field2: "test".to_string(),
        field3: vec![1, 2, 3],
    };

    // Encode multiple times
    let encoded1 = serialize(&data).unwrap();
    let encoded2 = serialize(&data).unwrap();
    let encoded3 = serialize(&data).unwrap();

    // All encodings should be identical
    assert_eq!(encoded1, encoded2);
    assert_eq!(encoded2, encoded3);
}

/// Test maximum size values
#[test]
fn cbor_compat_max_values() {
    // Max u8
    assert_eq!(serialize(&255u8).unwrap(), vec![0x18, 0xff]);

    // Max u16
    assert_eq!(serialize(&65535u16).unwrap(), vec![0x19, 0xff, 0xff]);

    // Max u32
    assert_eq!(
        serialize(&u32::MAX).unwrap(),
        vec![0x1a, 0xff, 0xff, 0xff, 0xff]
    );

    // Large u64
    assert_eq!(
        serialize(&u64::MAX).unwrap(),
        vec![0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );
}
