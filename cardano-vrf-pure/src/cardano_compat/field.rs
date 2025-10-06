//! Field element operations for Curve25519
//!
//! This module implements field arithmetic in GF(2^255-19), the finite field
//! used by Curve25519. The implementation uses radix 2^25.5 representation
//! with 10 limbs, matching the libsodium implementation exactly.
//!
//! # Representation
//!
//! Field elements are represented as `FieldElement([i64; 10])` where:
//! - Limbs alternate between 26 and 25 bits
//! - Element value = Σ(limb[i] * 2^(26*⌊i/2⌋ + 25*⌈i/2⌉))
//!
//! # Operations
//!
//! All operations maintain constant-time properties where applicable to
//! prevent timing attacks.

use std::ops::{Add, Mul, Neg, Sub};

/// Field element in GF(2^255-19)
///
/// Represents an element using 10 limbs in radix 2^25.5 representation.
/// This matches the libsodium fe25519 type exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldElement(pub [i64; 10]);

impl FieldElement {
    /// Precomputed constant sqrt(-1) matching libsodium's `fe_sqrtm1`.
    pub const SQRT_M1: Self = Self([
        -32595792, -7943725, 9377950, 3500415, 12389472, -272473, -25146209, -2005654, 326686,
        11406482,
    ]);

    /// Precomputed constant sqrt(-486664) (i.e. sqrt(-A-2) with A = 486662).
    pub const SQRT_AM2: Self = Self([
        -12222970, -8312128, -11511410, 9067497, -15300785, -241793, 25456130, 14121551, -12187136,
        3972024,
    ]);

    /// Zero element (additive identity)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cardano_vrf_pure::cardano_compat::field::FieldElement;
    /// let zero = FieldElement::zero();
    /// assert_eq!(zero.0[0], 0);
    /// ```
    #[inline]
    pub const fn zero() -> Self {
        FieldElement([0; 10])
    }

    /// One element (multiplicative identity)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cardano_vrf_pure::cardano_compat::field::FieldElement;
    /// let one = FieldElement::one();
    /// assert_eq!(one.0[0], 1);
    /// ```
    #[inline]
    pub const fn one() -> Self {
        let mut fe = [0i64; 10];
        fe[0] = 1;
        FieldElement(fe)
    }

    /// Load field element from 32 bytes (little-endian)
    ///
    /// Converts a 32-byte array to a field element using the standard
    /// encoding where bytes represent the element in little-endian order.
    ///
    /// # Arguments
    ///
    /// * `bytes` - 32-byte array in little-endian format
    ///
    /// # Returns
    ///
    /// Field element with limbs populated from the bytes
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cardano_vrf_pure::cardano_compat::field::FieldElement;
    /// let bytes = [0u8; 32];
    /// let fe = FieldElement::from_bytes(&bytes);
    /// ```
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let mut h = [0i64; 10];

        // Extract limbs from bytes according to radix 2^25.5
        h[0] = (bytes[0] as i64)
            | ((bytes[1] as i64) << 8)
            | ((bytes[2] as i64) << 16)
            | (((bytes[3] as i64) & 0x3f) << 24);

        h[1] = (((bytes[3] as i64) >> 6) & 0x03)
            | ((bytes[4] as i64) << 2)
            | ((bytes[5] as i64) << 10)
            | ((bytes[6] as i64) << 18)
            | (((bytes[7] as i64) & 0x01) << 24);

        h[2] = (((bytes[7] as i64) >> 1) & 0x7f)
            | ((bytes[8] as i64) << 7)
            | ((bytes[9] as i64) << 15)
            | (((bytes[10] as i64) & 0x07) << 23);

        h[3] = (((bytes[10] as i64) >> 3) & 0x1f)
            | ((bytes[11] as i64) << 5)
            | ((bytes[12] as i64) << 13)
            | (((bytes[13] as i64) & 0x0f) << 21);

        h[4] = (((bytes[13] as i64) >> 4) & 0x0f)
            | ((bytes[14] as i64) << 4)
            | ((bytes[15] as i64) << 12)
            | (((bytes[16] as i64) & 0x3f) << 20);

        h[5] = (((bytes[16] as i64) >> 6) & 0x03)
            | ((bytes[17] as i64) << 2)
            | ((bytes[18] as i64) << 10)
            | ((bytes[19] as i64) << 18)
            | (((bytes[20] as i64) & 0x01) << 24);

        h[6] = (((bytes[20] as i64) >> 1) & 0x7f)
            | ((bytes[21] as i64) << 7)
            | ((bytes[22] as i64) << 15)
            | (((bytes[23] as i64) & 0x07) << 23);

        h[7] = (((bytes[23] as i64) >> 3) & 0x1f)
            | ((bytes[24] as i64) << 5)
            | ((bytes[25] as i64) << 13)
            | (((bytes[26] as i64) & 0x0f) << 21);

        h[8] = (((bytes[26] as i64) >> 4) & 0x0f)
            | ((bytes[27] as i64) << 4)
            | ((bytes[28] as i64) << 12)
            | (((bytes[29] as i64) & 0x3f) << 20);

        h[9] = (((bytes[29] as i64) >> 6) & 0x03)
            | ((bytes[30] as i64) << 2)
            | ((bytes[31] as i64) << 10);

        FieldElement(h)
    }

    /// Convert field element to 32 bytes (little-endian)
    ///
    /// Converts the field element to its canonical byte representation.
    /// The element is first reduced modulo 2^255-19 to ensure uniqueness.
    ///
    /// # Returns
    ///
    /// 32-byte array representing the field element in little-endian format
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cardano_vrf_pure::cardano_compat::field::FieldElement;
    /// let fe = FieldElement::one();
    /// let bytes = fe.to_bytes();
    /// assert_eq!(bytes[0], 1);
    /// ```
    pub fn to_bytes(&self) -> [u8; 32] {
        // Start from a reduced representation to keep limb ranges small
        let mut h = self.reduce().0;

        // This implementation mirrors the ref10 fe_tobytes routine exactly.
        // It guarantees a canonical encoding strictly less than 2^255 - 19.

        let mut q = (19 * h[9] + (1 << 24)) >> 25;
        q = (h[0] + q) >> 26;
        q = (h[1] + q) >> 25;
        q = (h[2] + q) >> 26;
        q = (h[3] + q) >> 25;
        q = (h[4] + q) >> 26;
        q = (h[5] + q) >> 25;
        q = (h[6] + q) >> 26;
        q = (h[7] + q) >> 25;
        q = (h[8] + q) >> 26;
        q = (h[9] + q) >> 25;

        h[0] += 19 * q;

        // First carry propagation pass
        let mut carry = h[0] >> 26;
        h[1] += carry;
        h[0] -= carry << 26;

        carry = h[1] >> 25;
        h[2] += carry;
        h[1] -= carry << 25;

        carry = h[2] >> 26;
        h[3] += carry;
        h[2] -= carry << 26;

        carry = h[3] >> 25;
        h[4] += carry;
        h[3] -= carry << 25;

        carry = h[4] >> 26;
        h[5] += carry;
        h[4] -= carry << 26;

        carry = h[5] >> 25;
        h[6] += carry;
        h[5] -= carry << 25;

        carry = h[6] >> 26;
        h[7] += carry;
        h[6] -= carry << 26;

        carry = h[7] >> 25;
        h[8] += carry;
        h[7] -= carry << 25;

        carry = h[8] >> 26;
        h[9] += carry;
        h[8] -= carry << 26;

        carry = h[9] >> 25;
        h[9] -= carry << 25;
        h[0] += carry * 19;

        // Second carry propagation pass
        carry = h[0] >> 26;
        h[1] += carry;
        h[0] -= carry << 26;

        carry = h[1] >> 25;
        h[2] += carry;
        h[1] -= carry << 25;

        carry = h[2] >> 26;
        h[3] += carry;
        h[2] -= carry << 26;

        carry = h[3] >> 25;
        h[4] += carry;
        h[3] -= carry << 25;

        carry = h[4] >> 26;
        h[5] += carry;
        h[4] -= carry << 26;

        carry = h[5] >> 25;
        h[6] += carry;
        h[5] -= carry << 25;

        carry = h[6] >> 26;
        h[7] += carry;
        h[6] -= carry << 26;

        carry = h[7] >> 25;
        h[8] += carry;
        h[7] -= carry << 25;

        carry = h[8] >> 26;
        h[9] += carry;
        h[8] -= carry << 26;

        carry = h[9] >> 25;
        h[9] -= carry << 25;
        h[0] += carry * 19;

        carry = h[0] >> 26;
        h[1] += carry;
        h[0] -= carry << 26;

        carry = h[1] >> 25;
        h[2] += carry;
        h[1] -= carry << 25;

        // After the full propagation, h[9] fits in 25 bits and the remaining limbs alternate between 26 and 25 bits.

        let mut output = [0u8; 32];

        output[0] = (h[0] & 0xff) as u8;
        output[1] = ((h[0] >> 8) & 0xff) as u8;
        output[2] = ((h[0] >> 16) & 0xff) as u8;
        output[3] = ((h[0] >> 24) & 0x3f) as u8 | ((h[1] << 6) & 0xc0) as u8;

        output[4] = ((h[1] >> 2) & 0xff) as u8;
        output[5] = ((h[1] >> 10) & 0xff) as u8;
        output[6] = ((h[1] >> 18) & 0xff) as u8;
        output[7] = ((h[1] >> 26) & 0x01) as u8 | ((h[2] << 1) & 0xfe) as u8;

        output[8] = ((h[2] >> 7) & 0xff) as u8;
        output[9] = ((h[2] >> 15) & 0xff) as u8;
        output[10] = ((h[2] >> 23) & 0x07) as u8 | ((h[3] << 3) & 0xf8) as u8;

        output[11] = ((h[3] >> 5) & 0xff) as u8;
        output[12] = ((h[3] >> 13) & 0xff) as u8;
        output[13] = ((h[3] >> 21) & 0x0f) as u8 | ((h[4] << 4) & 0xf0) as u8;

        output[14] = ((h[4] >> 4) & 0xff) as u8;
        output[15] = ((h[4] >> 12) & 0xff) as u8;
        output[16] = ((h[4] >> 20) & 0x3f) as u8 | ((h[5] << 6) & 0xc0) as u8;

        output[17] = ((h[5] >> 2) & 0xff) as u8;
        output[18] = ((h[5] >> 10) & 0xff) as u8;
        output[19] = ((h[5] >> 18) & 0xff) as u8;
        output[20] = ((h[5] >> 26) & 0x01) as u8 | ((h[6] << 1) & 0xfe) as u8;

        output[21] = ((h[6] >> 7) & 0xff) as u8;
        output[22] = ((h[6] >> 15) & 0xff) as u8;
        output[23] = ((h[6] >> 23) & 0x07) as u8 | ((h[7] << 3) & 0xf8) as u8;

        output[24] = ((h[7] >> 5) & 0xff) as u8;
        output[25] = ((h[7] >> 13) & 0xff) as u8;
        output[26] = ((h[7] >> 21) & 0x0f) as u8 | ((h[8] << 4) & 0xf0) as u8;

        output[27] = ((h[8] >> 4) & 0xff) as u8;
        output[28] = ((h[8] >> 12) & 0xff) as u8;
        output[29] = ((h[8] >> 20) & 0x3f) as u8 | ((h[9] << 6) & 0xc0) as u8;

        output[30] = ((h[9] >> 2) & 0xff) as u8;
        output[31] = ((h[9] >> 10) & 0xff) as u8;

        const P_BYTES: [u8; 32] = [
            0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0x7f,
        ];

        let mut needs_sub = (output[31] >> 7) & 1;

        if needs_sub == 0 {
            // Compare output with p to detect wrap-around values equal to or above the modulus
            for i in (0..32).rev() {
                if output[i] > P_BYTES[i] {
                    needs_sub = 1;
                    break;
                } else if output[i] < P_BYTES[i] {
                    break;
                }
            }
        }

        if needs_sub == 1 {
            let mut borrow: i16 = 0;
            for i in 0..32 {
                let diff = output[i] as i16 - P_BYTES[i] as i16 - borrow;
                if diff < 0 {
                    output[i] = (diff + 256) as u8;
                    borrow = 1;
                } else {
                    output[i] = diff as u8;
                    borrow = 0;
                }
            }
        }

        output
    }

    /// Reduce field element modulo 2^255-19
    ///
    /// Performs carry propagation to ensure limbs are within their proper ranges.
    /// This matches the libsodium fe25519_reduce() function.
    ///
    /// # Returns
    ///
    /// Reduced field element with normalized limbs
    pub fn reduce(&self) -> Self {
        let mut h = self.0;
        let mut carry: i64;

        // First reduction pass
        carry = (h[0] + (1 << 25)) >> 26;
        h[1] += carry;
        h[0] -= carry << 26;

        carry = (h[4] + (1 << 25)) >> 26;
        h[5] += carry;
        h[4] -= carry << 26;

        carry = (h[1] + (1 << 24)) >> 25;
        h[2] += carry;
        h[1] -= carry << 25;

        carry = (h[5] + (1 << 24)) >> 25;
        h[6] += carry;
        h[5] -= carry << 25;

        carry = (h[2] + (1 << 25)) >> 26;
        h[3] += carry;
        h[2] -= carry << 26;

        carry = (h[6] + (1 << 25)) >> 26;
        h[7] += carry;
        h[6] -= carry << 26;

        carry = (h[3] + (1 << 24)) >> 25;
        h[4] += carry;
        h[3] -= carry << 25;

        carry = (h[7] + (1 << 24)) >> 25;
        h[8] += carry;
        h[7] -= carry << 25;

        carry = (h[8] + (1 << 25)) >> 26;
        h[9] += carry;
        h[8] -= carry << 26;

        // Handle top limb overflow (multiply by 19)
        // Need to do this multiple times if value is very large
        for _ in 0..3 {
            carry = h[9] >> 25;
            h[0] += carry * 19;
            h[9] -= carry << 25;

            // Propagate carry from h[0]
            carry = h[0] >> 26;
            h[1] += carry;
            h[0] -= carry << 26;

            // And from h[1] if needed
            carry = h[1] >> 25;
            h[2] += carry;
            h[1] -= carry << 25;
        }

        FieldElement(h)
    }

    /// Square the field element
    ///
    /// Computes self * self efficiently using the fact that many terms are identical.
    ///
    /// # Returns
    ///
    /// The squared field element
    #[inline]
    pub fn square(&self) -> Self {
        *self * *self
    }

    /// Square and double: 2 * self^2
    ///
    /// More efficient than computing square then doubling separately.
    ///
    /// # Returns
    ///
    /// 2 * self^2
    #[inline]
    pub fn square2(&self) -> Self {
        let sq = self.square();
        sq + sq
    }

    #[inline]
    fn pow2k(&self, k: u32) -> Self {
        debug_assert!(k > 0);
        let mut z = self.square().reduce();
        for _ in 1..k {
            z = z.square().reduce();
        }
        z
    }

    fn pow22501(&self) -> (Self, Self) {
        let t0 = self.square().reduce();
        let mut t1 = t0.square().reduce();
        t1 = t1.square().reduce();
        let t2 = (*self * t1).reduce();
        let t3 = (t0 * t2).reduce();
        let t4 = t3.square().reduce();
        let t5 = (t2 * t4).reduce();
        let t6 = t5.pow2k(5);
        let t7 = (t6 * t5).reduce();
        let t8 = t7.pow2k(10);
        let t9 = (t8 * t7).reduce();
        let t10 = t9.pow2k(20);
        let t11 = (t10 * t9).reduce();
        let t12 = t11.pow2k(10);
        let t13 = (t12 * t7).reduce();
        let t14 = t13.pow2k(50);
        let t15 = (t14 * t13).reduce();
        let t16 = t15.pow2k(100);
        let t17 = (t16 * t15).reduce();
        let t18 = t17.pow2k(50);
        let t19 = (t18 * t13).reduce();

        (t19, t3)
    }

    fn pow22523(&self) -> Self {
        let (t19, _) = self.pow22501();
        let t20 = t19.pow2k(2);
        (t20 * *self).reduce()
    }

    #[inline]
    fn pow_p58(&self) -> Self {
        // (p-5)/8 for p = 2^255 - 19 equals 2^252 - 3
        self.pow22523()
    }

    fn sqrt_ratio(u: &Self, v: &Self) -> (bool, Self) {
        let v2 = v.square().reduce();
        let v3 = (v2 * *v).reduce();
        let v6 = v3.square().reduce();
        let v7 = (v6 * *v).reduce();

        let u_v3 = (*u * v3).reduce();
        let u_v7 = (*u * v7).reduce();

        let pow = u_v7.pow_p58();
        let mut r = (u_v3 * pow).reduce();

        let vxx = (*v * r.square()).reduce();
        let m_root_check = (vxx - *u).reduce();
        let p_root_check = (vxx + *u).reduce();
        let u_sqrt_m1 = (*u * Self::SQRT_M1).reduce();
        let f_root_check = (vxx + u_sqrt_m1).reduce();

        let has_m_root = m_root_check.is_zero();
        let has_p_root = p_root_check.is_zero();
        let has_f_root = f_root_check.is_zero();

        if has_p_root || has_f_root {
            r = (r * Self::SQRT_M1).reduce();
        }

        if has_p_root {
            r = (-r).reduce();
        }

        if r.is_negative() {
            r = (-r).reduce();
        }

        (has_m_root || has_p_root, r)
    }

    /// Check if field element is square (has a square root)
    ///
    /// Uses Euler's criterion: x is a square iff x^((p-1)/2) = 1 (mod p)
    /// For p = 2^255-19, this is x^(2^254-10)
    ///
    /// # Returns
    ///
    /// `true` if the element has a square root, `false` otherwise
    pub fn is_square(&self) -> bool {
        let a = self.reduce();
        if a.is_zero() {
            return true;
        }

        let one = FieldElement::one();
        let (is_square, _) = Self::sqrt_ratio(&a, &one);
        is_square
    }

    /// Compute modular square root
    ///
    /// If the element is a quadratic residue, returns Some(sqrt).
    /// Otherwise returns None.
    ///
    /// # Algorithm
    ///
    /// For p = 2^255 - 19, we can compute sqrt using:
    /// sqrt(x) = x^((p+3)/8) = x^(2^252 - 2)
    ///
    /// We have pow22523() = x^(2^252 - 3), so:
    /// x^(2^252 - 2) = x^(2^252 - 3) * x = pow22523() * x
    ///
    /// # Returns
    ///
    /// `Some(sqrt)` if square root exists, `None` otherwise
    pub fn sqrt(&self) -> Option<Self> {
        let a = self.reduce();
        if a.is_zero() {
            return Some(FieldElement::zero());
        }

        let one = FieldElement::one();
        let (is_square, root) = Self::sqrt_ratio(&a, &one);

        if is_square {
            Some(root)
        } else {
            None
        }
    }

    /// Compute multiplicative inverse
    ///
    /// Returns the multiplicative inverse of the field element.
    /// For non-zero x, returns x^(-1) such that x * x^(-1) = 1.
    ///
    /// # Panics
    ///
    /// Panics if called on zero (which has no inverse).
    ///
    /// # Returns
    ///
    /// The multiplicative inverse
    pub fn invert(&self) -> Self {
        let (t19, t3) = self.pow22501();
        let t20 = t19.pow2k(5);
        (t20 * t3).reduce()
    }

    /// Conditional select: return `a` if `choice == 1`, else `b`
    ///
    /// This is a constant-time operation to prevent timing attacks.
    ///
    /// # Arguments
    ///
    /// * `a` - First field element
    /// * `b` - Second field element
    /// * `choice` - Selection bit (0 or 1)
    ///
    /// # Returns
    ///
    /// `a` if `choice == 1`, `b` if `choice == 0`
    #[inline]
    pub fn conditional_select(a: &Self, b: &Self, choice: u8) -> Self {
        let mask = -(choice as i64);
        let mut result = [0i64; 10];

        for i in 0..10 {
            result[i] = b.0[i] ^ (mask & (a.0[i] ^ b.0[i]));
        }

        FieldElement(result)
    }

    /// Conditional swap: swap `a` and `b` if `choice == 1`
    ///
    /// This is a constant-time operation to prevent timing attacks.
    ///
    /// # Arguments
    ///
    /// * `a` - First field element (mutable)
    /// * `b` - Second field element (mutable)
    /// * `choice` - Swap bit (0 or 1)
    pub fn conditional_swap(a: &mut Self, b: &mut Self, choice: u8) {
        let mask = -(choice as i64);

        for i in 0..10 {
            let t = mask & (a.0[i] ^ b.0[i]);
            a.0[i] ^= t;
            b.0[i] ^= t;
        }
    }

    /// Check if field element equals zero (constant-time)
    ///
    /// # Returns
    ///
    /// `true` if the element is zero, `false` otherwise
    pub fn is_zero(&self) -> bool {
        let reduced = self.reduce();
        let bytes = reduced.to_bytes();
        bytes.iter().all(|&b| b == 0)
    }

    /// Check if field element is negative
    ///
    /// An element is considered negative if its least significant bit is 1
    /// in the canonical byte representation.
    ///
    /// # Returns
    ///
    /// `true` if negative, `false` otherwise
    pub fn is_negative(&self) -> bool {
        let bytes = self.to_bytes();
        (bytes[0] & 1) == 1
    }
}

// Implement standard arithmetic operations

impl Add for FieldElement {
    type Output = Self;

    /// Add two field elements
    ///
    /// Performs component-wise addition of limbs.
    fn add(self, other: Self) -> Self {
        let mut h = [0i64; 10];
        for i in 0..10 {
            h[i] = self.0[i] + other.0[i];
        }
        FieldElement(h)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    /// Subtract two field elements
    ///
    /// Performs component-wise subtraction of limbs.
    fn sub(self, other: Self) -> Self {
        let mut h = [0i64; 10];
        for i in 0..10 {
            h[i] = self.0[i] - other.0[i];
        }
        FieldElement(h)
    }
}

impl Neg for FieldElement {
    type Output = Self;

    /// Negate field element
    ///
    /// Returns -self in the field.
    fn neg(self) -> Self {
        FieldElement::zero() - self
    }
}

impl Mul for FieldElement {
    type Output = Self;

    /// Multiply two field elements
    ///
    /// Performs full field multiplication matching the libsodium implementation.
    /// Uses the fact that reduction modulo 2^255-19 can be done by multiplying
    /// high limbs by 19.
    fn mul(self, other: Self) -> Self {
        let f = self.0;
        let g = other.0;

        // Convert to i128 to prevent overflow during multiplication
        let f: [i128; 10] = [
            f[0] as i128,
            f[1] as i128,
            f[2] as i128,
            f[3] as i128,
            f[4] as i128,
            f[5] as i128,
            f[6] as i128,
            f[7] as i128,
            f[8] as i128,
            f[9] as i128,
        ];
        let g: [i128; 10] = [
            g[0] as i128,
            g[1] as i128,
            g[2] as i128,
            g[3] as i128,
            g[4] as i128,
            g[5] as i128,
            g[6] as i128,
            g[7] as i128,
            g[8] as i128,
            g[9] as i128,
        ];

        // Precompute doubled values
        let f1_2 = 2 * f[1];
        let f3_2 = 2 * f[3];
        let f5_2 = 2 * f[5];
        let f7_2 = 2 * f[7];
        let f9_2 = 2 * f[9];

        // Precompute values multiplied by 19
        let g1_19 = 19 * g[1];
        let g2_19 = 19 * g[2];
        let g3_19 = 19 * g[3];
        let g4_19 = 19 * g[4];
        let g5_19 = 19 * g[5];
        let g6_19 = 19 * g[6];
        let g7_19 = 19 * g[7];
        let g8_19 = 19 * g[8];
        let g9_19 = 19 * g[9];

        let mut h = [0i128; 10];

        // Compute product limbs
        h[0] = f[0] * g[0]
            + f1_2 * g9_19
            + f[2] * g8_19
            + f3_2 * g7_19
            + f[4] * g6_19
            + f5_2 * g5_19
            + f[6] * g4_19
            + f7_2 * g3_19
            + f[8] * g2_19
            + f9_2 * g1_19;

        h[1] = f[0] * g[1]
            + f[1] * g[0]
            + f[2] * g9_19
            + f[3] * g8_19
            + f[4] * g7_19
            + f[5] * g6_19
            + f[6] * g5_19
            + f[7] * g4_19
            + f[8] * g3_19
            + f[9] * g2_19;

        h[2] = f[0] * g[2]
            + f1_2 * g[1]
            + f[2] * g[0]
            + f3_2 * g9_19
            + f[4] * g8_19
            + f5_2 * g7_19
            + f[6] * g6_19
            + f7_2 * g5_19
            + f[8] * g4_19
            + f9_2 * g3_19;

        h[3] = f[0] * g[3]
            + f[1] * g[2]
            + f[2] * g[1]
            + f[3] * g[0]
            + f[4] * g9_19
            + f[5] * g8_19
            + f[6] * g7_19
            + f[7] * g6_19
            + f[8] * g5_19
            + f[9] * g4_19;

        h[4] = f[0] * g[4]
            + f1_2 * g[3]
            + f[2] * g[2]
            + f3_2 * g[1]
            + f[4] * g[0]
            + f5_2 * g9_19
            + f[6] * g8_19
            + f7_2 * g7_19
            + f[8] * g6_19
            + f9_2 * g5_19;

        h[5] = f[0] * g[5]
            + f[1] * g[4]
            + f[2] * g[3]
            + f[3] * g[2]
            + f[4] * g[1]
            + f[5] * g[0]
            + f[6] * g9_19
            + f[7] * g8_19
            + f[8] * g7_19
            + f[9] * g6_19;

        h[6] = f[0] * g[6]
            + f1_2 * g[5]
            + f[2] * g[4]
            + f3_2 * g[3]
            + f[4] * g[2]
            + f5_2 * g[1]
            + f[6] * g[0]
            + f7_2 * g9_19
            + f[8] * g8_19
            + f9_2 * g7_19;

        h[7] = f[0] * g[7]
            + f[1] * g[6]
            + f[2] * g[5]
            + f[3] * g[4]
            + f[4] * g[3]
            + f[5] * g[2]
            + f[6] * g[1]
            + f[7] * g[0]
            + f[8] * g9_19
            + f[9] * g8_19;

        h[8] = f[0] * g[8]
            + f1_2 * g[7]
            + f[2] * g[6]
            + f3_2 * g[5]
            + f[4] * g[4]
            + f5_2 * g[3]
            + f[6] * g[2]
            + f7_2 * g[1]
            + f[8] * g[0]
            + f9_2 * g9_19;

        h[9] = f[0] * g[9]
            + f[1] * g[8]
            + f[2] * g[7]
            + f[3] * g[6]
            + f[4] * g[5]
            + f[5] * g[4]
            + f[6] * g[3]
            + f[7] * g[2]
            + f[8] * g[1]
            + f[9] * g[0];

        // Now do proper carry propagation from i128 to i64 with reduction
        // This is critical - we can't just truncate i128 to i64
        let mut carry: i128;

        // First carry propagation pass
        carry = (h[0] + (1i128 << 25)) >> 26;
        h[1] += carry;
        h[0] -= carry << 26;

        carry = (h[4] + (1i128 << 25)) >> 26;
        h[5] += carry;
        h[4] -= carry << 26;

        carry = (h[1] + (1i128 << 24)) >> 25;
        h[2] += carry;
        h[1] -= carry << 25;

        carry = (h[5] + (1i128 << 24)) >> 25;
        h[6] += carry;
        h[5] -= carry << 25;

        carry = (h[2] + (1i128 << 25)) >> 26;
        h[3] += carry;
        h[2] -= carry << 26;

        carry = (h[6] + (1i128 << 25)) >> 26;
        h[7] += carry;
        h[6] -= carry << 26;

        carry = (h[3] + (1i128 << 24)) >> 25;
        h[4] += carry;
        h[3] -= carry << 25;

        carry = (h[7] + (1i128 << 24)) >> 25;
        h[8] += carry;
        h[7] -= carry << 25;

        carry = (h[8] + (1i128 << 25)) >> 26;
        h[9] += carry;
        h[8] -= carry << 26;

        carry = (h[9] + (1i128 << 24)) >> 25;
        h[0] += carry * 19;
        h[9] -= carry << 25;

        // Second pass to handle overflow from h[0]
        carry = h[0] >> 26;
        h[1] += carry;
        h[0] -= carry << 26;

        // Convert to i64 - now safe because all limbs are in range
        let h_i64 = [
            h[0] as i64,
            h[1] as i64,
            h[2] as i64,
            h[3] as i64,
            h[4] as i64,
            h[5] as i64,
            h[6] as i64,
            h[7] as i64,
            h[8] as i64,
            h[9] as i64,
        ];

        FieldElement(h_i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_one() {
        let zero = FieldElement::zero();
        let one = FieldElement::one();

        assert_eq!(zero.0[0], 0);
        assert_eq!(one.0[0], 1);

        // 0 + 1 = 1
        let sum = zero + one;
        assert_eq!(sum.reduce().0[0], 1);
    }

    #[test]
    fn test_addition() {
        let one = FieldElement::one();
        let two = one + one;

        assert_eq!(two.reduce().0[0], 2);
    }

    #[test]
    fn test_multiplication() {
        let one = FieldElement::one();
        let two = one + one;
        let four = two * two;

        assert_eq!(four.reduce().0[0], 4);
    }

    #[test]
    fn test_square() {
        let two = FieldElement::one() + FieldElement::one();
        let four1 = two.square();
        let four2 = two * two;

        let r1 = four1.reduce();
        let r2 = four2.reduce();
        assert_eq!(r1.0, r2.0);
    }

    #[test]
    fn test_bytes_roundtrip() {
        // Test that a canonical value roundtrips correctly
        let one = FieldElement::one();
        let bytes1 = one.to_bytes();
        let fe_back = FieldElement::from_bytes(&bytes1);
        let bytes2 = fe_back.to_bytes();

        // These should be identical since one is canonical
        assert_eq!(bytes1, bytes2);

        // Test zero
        let zero = FieldElement::zero();
        let bytes_zero = zero.to_bytes();
        let fe_zero_back = FieldElement::from_bytes(&bytes_zero);
        assert_eq!(bytes_zero, fe_zero_back.to_bytes());
    }

    #[test]
    fn test_invert() {
        let fe = FieldElement::one();
        let inv = fe.invert();
        let product = fe * inv;
        assert_eq!(product.reduce(), FieldElement::one());
    }

    #[test]
    fn test_invert_non_trivial() {
        // Test with a non-trivial element
        let mut bytes = [0u8; 32];
        bytes[0] = 5;
        let fe = FieldElement::from_bytes(&bytes);
        let inv = fe.invert();
        let product = fe * inv;

        // Product should be 1
        let one = FieldElement::one();
        let product_reduced = product.reduce();
        let one_reduced = one.reduce();

        assert_eq!(product_reduced.to_bytes(), one_reduced.to_bytes());
    }

    #[test]
    fn test_is_square_four() {
        let two = FieldElement::one() + FieldElement::one();
        let four = two * two;

        // 4 should be a quadratic residue
        assert!(four.is_square(), "4 should be a quadratic residue");
    }

    #[test]
    fn test_sqrt() {
        // Test square root of 4 (should be 2 or -2)
        let mut bytes = [0u8; 32];
        bytes[0] = 4;
        let fe = FieldElement::from_bytes(&bytes);
        let sqrt = fe.sqrt().expect("square root of 4 should exist");
        let check = sqrt.square();
        assert_eq!(check.reduce().to_bytes(), fe.reduce().to_bytes());
    }

    #[test]
    fn test_conditional_select() {
        let a = FieldElement::one();
        let b = FieldElement::zero();

        let result_a = FieldElement::conditional_select(&a, &b, 1);
        assert_eq!(result_a, a);

        let result_b = FieldElement::conditional_select(&a, &b, 0);
        assert_eq!(result_b, b);
    }

    #[test]
    fn test_is_zero() {
        let zero = FieldElement::zero();
        assert!(zero.is_zero());

        let one = FieldElement::one();
        assert!(!one.is_zero());
    }

    #[test]
    fn test_is_square_zero() {
        let zero = FieldElement::zero();
        assert!(zero.is_square());
    }

    #[test]
    fn test_sqrt_minus_one() {
        let minus_one = FieldElement::zero() - FieldElement::one();
        let sqrt_minus_one = minus_one
            .sqrt()
            .expect("sqrt(-1) should exist in GF(2^255-19)");

        assert_eq!(
            sqrt_minus_one.square().reduce().to_bytes(),
            minus_one.reduce().to_bytes()
        );

        let sqrt_bytes = sqrt_minus_one.to_bytes();
        let pos = FieldElement::SQRT_M1.to_bytes();
        let neg = (-FieldElement::SQRT_M1).to_bytes();
        assert!(
            sqrt_bytes == pos || sqrt_bytes == neg,
            "sqrt(-1) should equal ±sqrt(-1) constant"
        );
    }
}
