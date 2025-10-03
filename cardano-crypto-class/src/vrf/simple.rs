use std::convert::TryFrom;
use std::fmt;

use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use cardano_binary::serialize;
use ciborium::value::Value;
use num_bigint::BigUint;

use crate::seed::{Seed, SeedRng};
use crate::util::{splits_at, write_binary_natural};

use super::{OutputVRF, VRFAlgorithm};

const FIELD_DEGREE: u32 = 113;
const FIELD_MASK: u128 = (1u128 << FIELD_DEGREE) - 1;
const REDUCTION_POLY: u128 = (1u128 << FIELD_DEGREE) | (1 << 9) | 1;

const CURVE_A: u128 = 0x0030_8825_0CA6_E7C7_FE64_9CE8_5820_F7;
const CURVE_B: u128 = 0x00E8_BEE4_D3E2_2607_4418_8BE0_E9C7_23;
const BASE_X: u128 = 0x009D_7361_6F35_F4AB_1407_D735_62C1_0F;
const BASE_Y: u128 = 0x00A5_2830_2779_58EE_84D1_315E_D318_86;
const CURVE_ORDER: u128 = 0x0100_0000_0000_0000_D9CC_EC8A_39E5_6F;

const SIMPLE_OUTPUT_SIZE: usize = 8;
const SIMPLE_SEED_ATTEMPTS: usize = 100;
const SIMPLE_SEED_SIZE: usize = 16 * SIMPLE_SEED_ATTEMPTS;

#[derive(Clone, Copy, PartialEq, Eq)]
struct FieldElement(u128);

impl FieldElement {
    fn new(value: u128) -> Self {
        FieldElement(value & FIELD_MASK)
    }

    fn zero() -> Self {
        FieldElement(0)
    }

    fn one() -> Self {
        FieldElement(1)
    }

    fn value(self) -> u128 {
        self.0
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn add(self, other: Self) -> Self {
        FieldElement::new(self.0 ^ other.0)
    }

    fn mul(self, other: Self) -> Self {
        FieldElement::new(gf_mul(self.0, other.0))
    }

    fn square(self) -> Self {
        self.mul(self)
    }

    fn inv(self) -> Self {
        assert!(!self.is_zero(), "field inversion of zero");
        FieldElement::new(gf_pow(self.0, (1u128 << FIELD_DEGREE) - 2))
    }

    fn div(self, other: Self) -> Self {
        self.mul(other.inv())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SimplePoint {
    Infinity,
    Affine { x: FieldElement, y: FieldElement },
}

impl fmt::Debug for SimplePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimplePoint::Infinity => f.write_str("SimplePoint(Infinity)"),
            SimplePoint::Affine { x, y } => f
                .debug_struct("SimplePoint")
                .field("x", &format_args!("0x{:x}", x.value()))
                .field("y", &format_args!("0x{:x}", y.value()))
                .finish(),
        }
    }
}

impl SimplePoint {
    fn affine(x: FieldElement, y: FieldElement) -> Self {
        SimplePoint::Affine { x, y }
    }

    fn is_infinity(&self) -> bool {
        matches!(self, SimplePoint::Infinity)
    }

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (SimplePoint::Infinity, p) | (p, SimplePoint::Infinity) => p,
            (SimplePoint::Affine { x: x1, y: y1 }, SimplePoint::Affine { x: x2, y: y2 }) => {
                if x1 == x2 {
                    let y_sum = y1.add(y2);
                    if y_sum.is_zero() {
                        SimplePoint::Infinity
                    } else {
                        SimplePoint::Affine { x: x1, y: y1 }.double()
                    }
                } else {
                    let numerator = y1.add(y2);
                    let denominator = x1.add(x2);
                    let lambda = numerator.div(denominator);
                    let x3 = lambda
                        .square()
                        .add(lambda)
                        .add(x1)
                        .add(x2)
                        .add(FieldElement::new(CURVE_A));
                    let y3 = lambda.mul(x1.add(x3)).add(x3).add(y1);
                    SimplePoint::affine(x3, y3)
                }
            },
        }
    }

    fn double(self) -> Self {
        match self {
            SimplePoint::Infinity => SimplePoint::Infinity,
            SimplePoint::Affine { x, y } => {
                if x == FieldElement::zero() {
                    SimplePoint::Infinity
                } else {
                    let lambda = x.add(y.div(x));
                    let x3 = lambda.square().add(lambda).add(FieldElement::new(CURVE_A));
                    let y3 = x.square().add(lambda.add(FieldElement::one()).mul(x3));
                    SimplePoint::affine(x3, y3)
                }
            },
        }
    }

    fn negate(self) -> Self {
        match self {
            SimplePoint::Infinity => SimplePoint::Infinity,
            SimplePoint::Affine { x, y } => SimplePoint::affine(x, x.add(y)),
        }
    }

    fn scalar_mul(self, mut scalar: u128) -> Self {
        let mut result = SimplePoint::Infinity;
        let mut addend = self;
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = result.add(addend);
            }
            scalar >>= 1;
            if scalar != 0 {
                addend = addend.double();
            }
        }
        result
    }

    fn is_on_curve(&self) -> bool {
        match self {
            SimplePoint::Infinity => true,
            SimplePoint::Affine { x, y } => {
                let left = y.square().add(x.mul(*y));
                let right = x
                    .square()
                    .mul(*x)
                    .add(FieldElement::new(CURVE_A).mul(x.square()))
                    .add(FieldElement::new(CURVE_B));
                left == right
            },
        }
    }

    fn to_option_coordinates(&self) -> Option<(u128, u128)> {
        match self {
            SimplePoint::Infinity => None,
            SimplePoint::Affine { x, y } => Some((x.value(), y.value())),
        }
    }
}

fn base_point() -> SimplePoint {
    SimplePoint::affine(FieldElement::new(BASE_X), FieldElement::new(BASE_Y))
}

fn curve_order_big() -> BigUint {
    BigUint::from(CURVE_ORDER)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SimpleVRF;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SimpleVerificationKey(SimplePoint);

impl fmt::Debug for SimpleVerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SimpleVerificationKey")
            .field(&self.0)
            .finish()
    }
}

impl SimpleVerificationKey {
    fn point(&self) -> SimplePoint {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SimpleSigningKey(u128);

impl fmt::Debug for SimpleSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SimpleSigningKey(0x{:x})", self.0)
    }
}

impl SimpleSigningKey {
    #[must_use]
    pub fn value(&self) -> u128 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SimpleCertificate {
    u: SimplePoint,
    c: u128,
    s: u128,
}

impl fmt::Debug for SimpleCertificate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleCertificate")
            .field("u", &self.u)
            .field("c", &format_args!("0x{:x}", self.c))
            .field("s", &format_args!("0x{:x}", self.s))
            .finish()
    }
}

impl SimpleCertificate {
    fn point(&self) -> SimplePoint {
        self.u
    }

    fn challenge(&self) -> u128 {
        self.c
    }

    fn response(&self) -> u128 {
        self.s
    }
}

fn gf_mul(mut a: u128, mut b: u128) -> u128 {
    let mut result = 0u128;
    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }
        b >>= 1;
        a <<= 1;
        if a & (1 << FIELD_DEGREE) != 0 {
            a ^= REDUCTION_POLY;
        }
    }
    result & FIELD_MASK
}

fn gf_pow(mut base: u128, mut exponent: u128) -> u128 {
    let mut result = 1u128;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = gf_mul(result, base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = gf_mul(base, base);
        }
    }
    result & FIELD_MASK
}

fn hash_short(data: &[u8]) -> [u8; SIMPLE_OUTPUT_SIZE] {
    let mut hasher =
        Blake2bVar::new(SIMPLE_OUTPUT_SIZE).expect("Blake2bVar supports 8-byte output");
    hasher.update(data);
    let mut output = [0u8; SIMPLE_OUTPUT_SIZE];
    hasher
        .finalize_variable(&mut output)
        .expect("Blake2bVar finalise");
    output
}

fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    serialize(&bytes.to_vec()).expect("CBOR serialisation failed")
}

fn cbor_unsigned(value: u128) -> Value {
    use ciborium::value::Integer;

    if value <= u64::MAX as u128 {
        Value::Integer(Integer::from(value as u64))
    } else {
        let bytes = value
            .to_be_bytes()
            .into_iter()
            .skip_while(|byte| *byte == 0)
            .collect::<Vec<_>>();
        Value::Tag(2, Box::new(Value::Bytes(bytes)))
    }
}

fn encode_point(point: &SimplePoint) -> Vec<u8> {
    let encoded = match point.to_option_coordinates() {
        None => Value::Null,
        Some((x, y)) => Value::Array(vec![cbor_unsigned(x), cbor_unsigned(y)]),
    };
    serialize(&encoded).expect("CBOR serialisation failed")
}

fn bytes_to_u128(bytes: &[u8]) -> u128 {
    bytes
        .iter()
        .fold(0u128, |acc, &b| (acc << 8) | u128::from(b))
}

fn reduce_scalar(value: &BigUint) -> u128 {
    let modulus = curve_order_big();
    let reduced = value % &modulus;
    u128::try_from(reduced).expect("scalar fits into u128")
}

fn hash_point(encoding: &[u8], scalar: u128) -> SimplePoint {
    let hash = hash_short(encoding);
    let hash_value = BigUint::from(bytes_to_u128(&hash));
    let scalar_big = BigUint::from(scalar);
    let product = (scalar_big * hash_value) % curve_order_big();
    base_point().scalar_mul(reduce_scalar(&product))
}

fn pow_base(scalar: u128) -> SimplePoint {
    base_point().scalar_mul(scalar % CURVE_ORDER)
}

fn pow_point(point: SimplePoint, scalar: u128) -> SimplePoint {
    point.scalar_mul(scalar % CURVE_ORDER)
}

fn combine_bytes(chunks: &[Vec<u8>]) -> Vec<u8> {
    let total = chunks.iter().map(Vec::len).sum();
    let mut joined = Vec::with_capacity(total);
    for chunk in chunks {
        joined.extend_from_slice(chunk);
    }
    joined
}

impl VRFAlgorithm for SimpleVRF {
    type VerificationKey = SimpleVerificationKey;
    type SigningKey = SimpleSigningKey;
    type Proof = SimpleCertificate;
    type Context = ();

    const ALGORITHM_NAME: &'static str = "simple";
    const SEED_SIZE: usize = SIMPLE_SEED_SIZE;
    const VERIFICATION_KEY_SIZE: usize = 32;
    const SIGNING_KEY_SIZE: usize = 16;
    const PROOF_SIZE: usize = 64;
    const OUTPUT_SIZE: usize = SIMPLE_OUTPUT_SIZE;

    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey {
        let point = pow_base(signing_key.value());
        SimpleVerificationKey(point)
    }

    fn evaluate_bytes(
        _context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> (OutputVRF<Self>, Self::Proof) {
        let message_enc = encode_bytes(message);
        let sk_value = signing_key.value();

        let u_point = hash_point(&message_enc, sk_value);
        let u_enc = encode_point(&u_point);

        let y_bytes = hash_short(&combine_bytes(&[message_enc.clone(), u_enc.clone()]));
        let output = OutputVRF::<Self>::from_bytes(y_bytes.to_vec())
            .expect("short hash length matches OUTPUT_SIZE");

        let r = bytes_to_u128(&y_bytes) % CURVE_ORDER;

        let vk_point = Self::derive_verification_key(signing_key).point();
        let pow_r = pow_base(r);
        let h_prime_r = hash_point(&message_enc, r);

        let c_bytes = hash_short(&combine_bytes(&[
            message_enc.clone(),
            encode_point(&vk_point),
            encode_point(&pow_r),
            encode_point(&h_prime_r),
        ]));

        let c_value = bytes_to_u128(&c_bytes);

        let r_big = BigUint::from(r);
        let k_big = BigUint::from(sk_value);
        let c_big = BigUint::from(c_value);
        let s_big = (r_big + k_big * c_big) % curve_order_big();
        let s_value = u128::try_from(s_big).expect("scalar fits in u128");

        let cert = SimpleCertificate {
            u: u_point,
            c: c_value,
            s: s_value,
        };

        (output, cert)
    }

    fn verify_bytes(
        _context: &Self::Context,
        verification_key: &Self::VerificationKey,
        message: &[u8],
        proof: &Self::Proof,
    ) -> Option<OutputVRF<Self>> {
        let message_enc = encode_bytes(message);

        let proof_point = proof.point();
        let o_bytes = hash_short(&combine_bytes(&[
            message_enc.clone(),
            encode_point(&proof_point),
        ]));
        let output = OutputVRF::<Self>::from_bytes(o_bytes.to_vec()).ok()?;

        let vk_point = verification_key.point();
        if vk_point.is_infinity() || proof_point.is_infinity() {
            return None;
        }

        if !vk_point.is_on_curve() || !proof_point.is_on_curve() {
            return None;
        }

        let c_value = proof.challenge();
        let c_mod = c_value % CURVE_ORDER;
        let s_value = proof.response() % CURVE_ORDER;

        let lhs = pow_base(s_value).add(pow_point(vk_point, c_mod).negate());
        let rhs_point =
            hash_point(&message_enc, s_value).add(pow_point(proof_point, c_mod).negate());

        let rhs_bytes = hash_short(&combine_bytes(&[
            message_enc,
            encode_point(&vk_point),
            encode_point(&lhs),
            encode_point(&rhs_point),
        ]));

        if c_value == bytes_to_u128(&rhs_bytes) {
            Some(output)
        } else {
            None
        }
    }

    fn gen_key_from_seed_bytes(seed: &[u8]) -> Self::SigningKey {
        assert_eq!(seed.len(), Self::SEED_SIZE, "unexpected seed length");
        let mut rng = SeedRng::new(Seed::from_bytes(seed.to_vec()));
        let mut buf = [0u8; 16];
        for _ in 0..SIMPLE_SEED_ATTEMPTS {
            rng.fill_bytes_checked(&mut buf)
                .expect("insufficient seed material for SimpleVRF");
            let candidate = bytes_to_u128(&buf) % CURVE_ORDER;
            if candidate != 0 {
                return SimpleSigningKey(candidate);
            }
        }
        panic!("failed to derive SimpleVRF signing key from seed");
    }

    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8> {
        match key.point() {
            SimplePoint::Infinity => panic!("cannot serialise point at infinity"),
            SimplePoint::Affine { x, y } => {
                let mut bytes = write_binary_natural(16, &BigUint::from(x.value()));
                bytes.extend(write_binary_natural(16, &BigUint::from(y.value())));
                bytes
            },
        }
    }

    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey> {
        let parts = splits_at(&[16, 16], bytes);
        if parts.len() != 2 {
            return None;
        }
        let x = bytes_to_u128(&parts[0]);
        let y = bytes_to_u128(&parts[1]);
        let point = SimplePoint::affine(FieldElement::new(x), FieldElement::new(y));
        point.is_on_curve().then_some(SimpleVerificationKey(point))
    }

    fn raw_serialize_signing_key(key: &Self::SigningKey) -> Vec<u8> {
        write_binary_natural(16, &BigUint::from(key.value()))
    }

    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey> {
        let value = bytes_to_u128(bytes);
        if value == 0 || value >= CURVE_ORDER {
            None
        } else {
            Some(SimpleSigningKey(value))
        }
    }

    fn raw_serialize_proof(proof: &Self::Proof) -> Vec<u8> {
        match proof.point() {
            SimplePoint::Infinity => panic!("cannot serialise point at infinity"),
            SimplePoint::Affine { x, y } => {
                let mut bytes = write_binary_natural(16, &BigUint::from(x.value()));
                bytes.extend(write_binary_natural(16, &BigUint::from(y.value())));
                bytes.extend(write_binary_natural(16, &BigUint::from(proof.challenge())));
                bytes.extend(write_binary_natural(16, &BigUint::from(proof.response())));
                bytes
            },
        }
    }

    fn raw_deserialize_proof(bytes: &[u8]) -> Option<Self::Proof> {
        let parts = splits_at(&[16, 16, 16, 16], bytes);
        if parts.len() != 4 {
            return None;
        }

        let x = bytes_to_u128(&parts[0]);
        let y = bytes_to_u128(&parts[1]);
        let c = bytes_to_u128(&parts[2]);
        let s = bytes_to_u128(&parts[3]);

        let point = SimplePoint::affine(FieldElement::new(x), FieldElement::new(y));
        if !point.is_on_curve() {
            return None;
        }

        Some(SimpleCertificate { u: point, c, s })
    }
}

impl From<SimpleSigningKey> for SimpleVerificationKey {
    fn from(value: SimpleSigningKey) -> Self {
        SimpleVRF::derive_verification_key(&value)
    }
}

impl From<&SimpleSigningKey> for SimpleVerificationKey {
    fn from(value: &SimpleSigningKey) -> Self {
        SimpleVRF::derive_verification_key(value)
    }
}

/// Convenience helper mirroring the Haskell API.
#[must_use]
pub fn gen_key(seed: &Seed) -> SimpleSigningKey {
    SimpleVRF::gen_key(seed)
}

/// Deterministically derive a keypair from a seed.
#[must_use]
pub fn gen_keypair(seed: &Seed) -> (SimpleSigningKey, SimpleVerificationKey) {
    SimpleVRF::gen_keypair(seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_and_verify_roundtrip() {
        let seed_bytes = vec![42u8; SIMPLE_SEED_SIZE];
        let seed = Seed::from_bytes(seed_bytes);
        let sk = SimpleVRF::gen_key(&seed);
        let vk = SimpleVRF::derive_verification_key(&sk);
        let message = b"hello simple vrf";
        let (output, cert) = SimpleVRF::evaluate_bytes(&(), message.as_ref(), &sk);
        let verified = SimpleVRF::verify_bytes(&(), &vk, message.as_ref(), &cert)
            .expect("verification should succeed");
        assert_eq!(output.as_bytes(), verified.as_bytes());
    }
}
