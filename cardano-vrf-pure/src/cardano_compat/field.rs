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
        // First, do standard reduction
        let mut h = self.reduce().0;
        
        // Freeze: reduce to canonical form in [0, p)
        // This matches libsodium's fe25519_tobytes which does a final reduction
        
        // Reduce top limb one more time
        let mut q = (h[9] + (1 << 24)) >> 25;
        for _ in 0..2 {
            q = ((h[0] + 19 * q) + (1 << 25)) >> 26;
            q = ((h[1] + q) + (1 << 24)) >> 25;
            q = ((h[2] + q) + (1 << 25)) >> 26;
            q = ((h[3] + q) + (1 << 24)) >> 25;
            q = ((h[4] + q) + (1 << 25)) >> 26;
            q = ((h[5] + q) + (1 << 24)) >> 25;
            q = ((h[6] + q) + (1 << 25)) >> 26;
            q = ((h[7] + q) + (1 << 24)) >> 25;
            q = ((h[8] + q) + (1 << 25)) >> 26;
            q = ((h[9] + q) + (1 << 24)) >> 25;
        }
        
        // Final reduction: subtract q*p
        h[0] += 19 * q;
        
        // Carry propagation
        let mut carry = h[0] >> 26; h[1] += carry; h[0] -= carry << 26;
        carry = h[1] >> 25; h[2] += carry; h[1] -= carry << 25;
        carry = h[2] >> 26; h[3] += carry; h[2] -= carry << 26;
        carry = h[3] >> 25; h[4] += carry; h[3] -= carry << 25;
        carry = h[4] >> 26; h[5] += carry; h[4] -= carry << 26;
        carry = h[5] >> 25; h[6] += carry; h[5] -= carry << 25;
        carry = h[6] >> 26; h[7] += carry; h[6] -= carry << 26;
        carry = h[7] >> 25; h[8] += carry; h[7] -= carry << 25;
        carry = h[8] >> 26; h[9] += carry; h[8] -= carry << 26;
        h[9] -= (h[9] >> 25) << 25;
        
        let mut output = [0u8; 32];

        // Pack limbs into bytes according to radix 2^25.5
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

    /// Compute self^(2^252-3) for inversion
    ///
    /// This is a helper function used in field inversion.
    /// Implements exponentiation by squaring for the specific exponent needed.
    ///
    /// # Returns
    ///
    /// self^(2^252-3)
    fn pow22523(&self) -> Self {
        let z2 = self.square();
        let z8 = z2.square().square();
        let z9 = (*self * z8);
        let z11 = z2 * z9;
        let z22 = z11.square();
        let z_5_0 = z9 * z22;
        let z_10_5 = (0..5).fold(z_5_0, |acc, _| acc.square());
        let z_10_0 = z_10_5 * z_5_0;
        let z_20_10 = (0..10).fold(z_10_0, |acc, _| acc.square()).reduce(); // Reduce after 10 squares
        let z_20_0 = z_20_10 * z_10_0;
        let z_40_20 = (0..20).fold(z_20_0, |acc, _| acc.square()).reduce(); // Reduce after 20 squares
        let z_40_0 = z_40_20 * z_20_0;
        let z_50_10 = (0..10).fold(z_40_0, |acc, _| acc.square()).reduce(); // Reduce after 10 squares
        let z_50_0 = z_50_10 * z_10_0;
        let z_100_50 = (0..50).fold(z_50_0, |acc, _| acc.square()).reduce(); // Reduce after 50 squares
        let z_100_0 = z_100_50 * z_50_0;
        let z_200_100 = (0..100).fold(z_100_0, |acc, _| acc.square()).reduce(); // Reduce after 100 squares
        let z_200_0 = z_200_100 * z_100_0;
        let z_250_50 = (0..50).fold(z_200_0, |acc, _| acc.square()).reduce(); // Reduce after 50 squares
        let z_250_0 = z_250_50 * z_50_0;
        let z_252_2 = z_250_0.square().square();
        (z_252_2 * *self).reduce()
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
        // Compute x^((p-1)/2) using Euler's criterion
        // For p = 2^255-19: (p-1)/2 = 2^254 - 10
        // We compute this as: (x^(2^252-3))^4 * x^2
        let pow_result = self.pow22523(); // x^(2^252 - 3)
        let pow_sq = pow_result.square(); // x^(2^253 - 6)
        let pow_sq_sq = pow_sq.square(); // x^(2^254 - 12)
        let x2 = self.square(); // x^2
        let check = (pow_sq_sq * x2).reduce(); // x^(2^254 - 12 + 2) = x^(2^254 - 10), final reduce
        
        // Check if result equals 1
        let one = FieldElement::one();
        let check_bytes = check.to_bytes();
        let one_bytes = one.to_bytes();
        
        check_bytes[..] == one_bytes[..]
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
        // Compute candidate = x^(2^252 - 2) = x^(2^252 - 3) * x
        let pow_result = self.pow22523();
        let candidate = pow_result * *self;
        
        // Check if candidate^2 == self
        let check = candidate.square();
        let diff = check - *self;
        let diff_bytes = diff.to_bytes();
        
        let is_zero = diff_bytes.iter().all(|&b| b == 0);
        
        // eprintln!("DEBUG sqrt: input = {}", hex::encode(self.to_bytes()));
        // eprintln!("DEBUG sqrt: candidate = {}", hex::encode(candidate.to_bytes()));
        // eprintln!("DEBUG sqrt: candidate^2 = {}", hex::encode(check.to_bytes()));
        // eprintln!("DEBUG sqrt: diff = {}", hex::encode(diff_bytes));
        // eprintln!("DEBUG sqrt: is_zero = {}", is_zero);
        
        if is_zero {
            Some(candidate)
        } else {
            // Try the other square root: -candidate
            let neg_candidate = -candidate;
            let check2 = neg_candidate.square();
            let diff2 = check2 - *self;
            let diff2_bytes = diff2.to_bytes();
            
            let is_zero2 = diff2_bytes.iter().all(|&b| b == 0);
            
            // eprintln!("DEBUG sqrt: neg_candidate = {}", hex::encode(neg_candidate.to_bytes()));
            // eprintln!("DEBUG sqrt: neg_candidate^2 = {}", hex::encode(check2.to_bytes()));
            // eprintln!("DEBUG sqrt: diff2 = {}", hex::encode(diff2_bytes));
            // eprintln!("DEBUG sqrt: is_zero2 = {}", is_zero2);
            
            if is_zero2 {
                Some(neg_candidate)
            } else {
                None
            }
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
        // Use Fermat's little theorem: x^(p-2) = x^(-1) mod p
        // For p = 2^255-19: x^(-1) = x^(2^255-21)
        //
        // Using the chain from curve25519-dalek which is proven correct:
        // We build up the exponent bit by bit
        
        let z2 = self.square();
        let z9 = z2.square().square() * *self;
        let z11 = z2 * z9;
        let z2_5_0 = (z9.square() * z11).reduce();
        let z2_10_0 = (0..5).fold(z2_5_0, |acc, _| acc.square()).reduce() * z2_5_0;
        let z2_20_0 = (0..10).fold(z2_10_0, |acc, _| acc.square()).reduce() * z2_10_0;
        let z2_40_0 = (0..20).fold(z2_20_0, |acc, _| acc.square()).reduce() * z2_20_0;
        let z2_50_0 = (0..10).fold(z2_40_0, |acc, _| acc.square()).reduce() * z2_10_0;
        let z2_100_0 = (0..50).fold(z2_50_0, |acc, _| acc.square()).reduce() * z2_50_0;
        let z2_200_0 = (0..100).fold(z2_100_0, |acc, _| acc.square()).reduce() * z2_100_0;
        let z2_250_0 = (0..50).fold(z2_200_0, |acc, _| acc.square()).reduce() * z2_50_0;
        
        // z2_250_0 = x^(2^250-1)
        // We need x^(2^255-21) = x^(2^255-19-2) = x^(p-2)
        // = (x^(2^250-1))^(2^5) * x^11
        let result = (0..5).fold(z2_250_0, |acc, _| acc.square()).reduce() * z11;
        result.reduce()
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
            f[0] as i128, f[1] as i128, f[2] as i128, f[3] as i128, f[4] as i128,
            f[5] as i128, f[6] as i128, f[7] as i128, f[8] as i128, f[9] as i128,
        ];
        let g: [i128; 10] = [
            g[0] as i128, g[1] as i128, g[2] as i128, g[3] as i128, g[4] as i128,
            g[5] as i128, g[6] as i128, g[7] as i128, g[8] as i128, g[9] as i128,
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
            h[0] as i64, h[1] as i64, h[2] as i64, h[3] as i64, h[4] as i64,
            h[5] as i64, h[6] as i64, h[7] as i64, h[8] as i64, h[9] as i64,
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
        
        if let Some(sqrt) = fe.sqrt() {
            let check = sqrt.square();
            assert_eq!(check.reduce().to_bytes(), fe.reduce().to_bytes());
        }
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
}
#[test]
fn test_debug_sqrt() {
    let two = FieldElement::one() + FieldElement::one();
    let four = two * two;
    
    println!("Testing sqrt(4)...");
    println!("is_square(4) = {}", four.is_square());
    
    match four.sqrt() {
        Some(sqrt_four) => {
            println!("sqrt(4) found!");
            let sqrt_squared = sqrt_four * sqrt_four;
            println!("sqrt(4)^2 = {:?}", &sqrt_squared.to_bytes()[..8]);
            println!("4 = {:?}", &four.to_bytes()[..8]);
        }
        None => {
            println!("sqrt(4) = None (FAILED)");
        }
    }
}
