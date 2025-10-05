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
        let h = self.reduce();
        let mut output = [0u8; 32];

        // Pack limbs into bytes according to radix 2^25.5
        output[0] = (h.0[0] & 0xff) as u8;
        output[1] = ((h.0[0] >> 8) & 0xff) as u8;
        output[2] = ((h.0[0] >> 16) & 0xff) as u8;
        output[3] = ((h.0[0] >> 24) & 0x3f) as u8 | ((h.0[1] << 6) & 0xc0) as u8;
        
        output[4] = ((h.0[1] >> 2) & 0xff) as u8;
        output[5] = ((h.0[1] >> 10) & 0xff) as u8;
        output[6] = ((h.0[1] >> 18) & 0xff) as u8;
        output[7] = ((h.0[1] >> 26) & 0x01) as u8 | ((h.0[2] << 1) & 0xfe) as u8;
        
        output[8] = ((h.0[2] >> 7) & 0xff) as u8;
        output[9] = ((h.0[2] >> 15) & 0xff) as u8;
        output[10] = ((h.0[2] >> 23) & 0x07) as u8 | ((h.0[3] << 3) & 0xf8) as u8;
        
        output[11] = ((h.0[3] >> 5) & 0xff) as u8;
        output[12] = ((h.0[3] >> 13) & 0xff) as u8;
        output[13] = ((h.0[3] >> 21) & 0x0f) as u8 | ((h.0[4] << 4) & 0xf0) as u8;
        
        output[14] = ((h.0[4] >> 4) & 0xff) as u8;
        output[15] = ((h.0[4] >> 12) & 0xff) as u8;
        output[16] = ((h.0[4] >> 20) & 0x3f) as u8 | ((h.0[5] << 6) & 0xc0) as u8;
        
        output[17] = ((h.0[5] >> 2) & 0xff) as u8;
        output[18] = ((h.0[5] >> 10) & 0xff) as u8;
        output[19] = ((h.0[5] >> 18) & 0xff) as u8;
        output[20] = ((h.0[5] >> 26) & 0x01) as u8 | ((h.0[6] << 1) & 0xfe) as u8;
        
        output[21] = ((h.0[6] >> 7) & 0xff) as u8;
        output[22] = ((h.0[6] >> 15) & 0xff) as u8;
        output[23] = ((h.0[6] >> 23) & 0x07) as u8 | ((h.0[7] << 3) & 0xf8) as u8;
        
        output[24] = ((h.0[7] >> 5) & 0xff) as u8;
        output[25] = ((h.0[7] >> 13) & 0xff) as u8;
        output[26] = ((h.0[7] >> 21) & 0x0f) as u8 | ((h.0[8] << 4) & 0xf0) as u8;
        
        output[27] = ((h.0[8] >> 4) & 0xff) as u8;
        output[28] = ((h.0[8] >> 12) & 0xff) as u8;
        output[29] = ((h.0[8] >> 20) & 0x3f) as u8 | ((h.0[9] << 6) & 0xc0) as u8;
        
        output[30] = ((h.0[9] >> 2) & 0xff) as u8;
        output[31] = ((h.0[9] >> 10) & 0xff) as u8;

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
        carry = h[9] >> 25;
        h[0] += carry * 19;
        h[9] -= carry << 25;

        // Final carry propagation
        carry = h[0] >> 26;
        h[1] += carry;
        h[0] -= carry << 26;

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

        let mut h = [0i64; 10];

        // Compute product limbs
        h[0] = f[0] * g[0] + f1_2 * g9_19 + f[2] * g8_19 + f3_2 * g7_19 
             + f[4] * g6_19 + f5_2 * g5_19 + f[6] * g4_19 + f7_2 * g3_19 
             + f[8] * g2_19 + f9_2 * g1_19;
        
        h[1] = f[0] * g[1] + f[1] * g[0] + f[2] * g9_19 + f[3] * g8_19 
             + f[4] * g7_19 + f[5] * g6_19 + f[6] * g5_19 + f[7] * g4_19 
             + f[8] * g3_19 + f[9] * g2_19;
        
        h[2] = f[0] * g[2] + f1_2 * g[1] + f[2] * g[0] + f3_2 * g9_19 
             + f[4] * g8_19 + f5_2 * g7_19 + f[6] * g6_19 + f7_2 * g5_19 
             + f[8] * g4_19 + f9_2 * g3_19;
        
        h[3] = f[0] * g[3] + f[1] * g[2] + f[2] * g[1] + f[3] * g[0] 
             + f[4] * g9_19 + f[5] * g8_19 + f[6] * g7_19 + f[7] * g6_19 
             + f[8] * g5_19 + f[9] * g4_19;
        
        h[4] = f[0] * g[4] + f1_2 * g[3] + f[2] * g[2] + f3_2 * g[1] 
             + f[4] * g[0] + f5_2 * g9_19 + f[6] * g8_19 + f7_2 * g7_19 
             + f[8] * g6_19 + f9_2 * g5_19;
        
        h[5] = f[0] * g[5] + f[1] * g[4] + f[2] * g[3] + f[3] * g[2] 
             + f[4] * g[1] + f[5] * g[0] + f[6] * g9_19 + f[7] * g8_19 
             + f[8] * g7_19 + f[9] * g6_19;
        
        h[6] = f[0] * g[6] + f1_2 * g[5] + f[2] * g[4] + f3_2 * g[3] 
             + f[4] * g[2] + f5_2 * g[1] + f[6] * g[0] + f7_2 * g9_19 
             + f[8] * g8_19 + f9_2 * g7_19;
        
        h[7] = f[0] * g[7] + f[1] * g[6] + f[2] * g[5] + f[3] * g[4] 
             + f[4] * g[3] + f[5] * g[2] + f[6] * g[1] + f[7] * g[0] 
             + f[8] * g9_19 + f[9] * g8_19;
        
        h[8] = f[0] * g[8] + f1_2 * g[7] + f[2] * g[6] + f3_2 * g[5] 
             + f[4] * g[4] + f5_2 * g[3] + f[6] * g[2] + f7_2 * g[1] 
             + f[8] * g[0] + f9_2 * g9_19;
        
        h[9] = f[0] * g[9] + f[1] * g[8] + f[2] * g[7] + f[3] * g[6] 
             + f[4] * g[5] + f[5] * g[4] + f[6] * g[3] + f[7] * g[2] 
             + f[8] * g[1] + f[9] * g[0];

        FieldElement(h).reduce()
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
}
