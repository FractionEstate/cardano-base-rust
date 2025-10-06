//! Edwards point operations and cofactor clearing
//!
//! This module implements operations on Edwards curve points, including
//! Cardano-specific cofactor clearing that differs from standard implementations.

use crate::{VrfError, VrfResult};
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use once_cell::sync::Lazy;

/// Prime modulus for Curve25519 field: 2^255 - 19
static P: Lazy<BigUint> = Lazy::new(|| (BigUint::one() << 255) - BigUint::from(19u8));
/// Montgomery curve parameter A = 486662
static A_BIG: Lazy<BigUint> = Lazy::new(|| BigUint::from(486662u32));
/// sqrt(-1) mod p
static SQRT_M1_BIG: Lazy<BigUint> = Lazy::new(|| {
    let exp = (&*P - BigUint::one()) >> 2;
    mod_pow(&BigUint::from(2u8), &exp)
});
/// sqrt(-A-2) mod p (Cardano constant)
static SQRT_AM2_BIG: Lazy<BigUint> = Lazy::new(|| {
    let neg_a_minus_two = mod_neg(&mod_add(&*A_BIG, &BigUint::from(2u8)));
    sqrt_mod(&neg_a_minus_two).expect("sqrt(-A-2) must exist")
});

/// Convert little-endian bytes to BigUint modulo p
fn bytes_to_biguint(bytes: &[u8; 32]) -> BigUint {
    BigUint::from_bytes_le(bytes) % &*P
}

/// Convert BigUint (already reduced) to 32-byte little-endian array
fn biguint_to_bytes(value: &BigUint) -> [u8; 32] {
    let mut bytes = value.to_bytes_le();
    bytes.resize(32, 0);
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    out
}

fn mod_add(a: &BigUint, b: &BigUint) -> BigUint {
    let mut res = a + b;
    if res >= *P {
        res -= &*P;
    }
    res
}

fn mod_sub(a: &BigUint, b: &BigUint) -> BigUint {
    if a >= b { a - b } else { &*P - (b - a) }
}

fn mod_neg(a: &BigUint) -> BigUint {
    if a.is_zero() {
        BigUint::zero()
    } else {
        &*P - (a % &*P)
    }
}

fn mod_mul(a: &BigUint, b: &BigUint) -> BigUint {
    ((a % &*P) * (b % &*P)) % &*P
}

fn mod_pow(base: &BigUint, exp: &BigUint) -> BigUint {
    base.modpow(exp, &*P)
}

fn mod_inv(a: &BigUint) -> BigUint {
    if a.is_zero() {
        BigUint::zero()
    } else {
        let exp = &*P - BigUint::from(2u8);
        mod_pow(a, &exp)
    }
}

fn is_quadratic_residue(n: &BigUint) -> bool {
    if n.is_zero() {
        return true;
    }
    let exp = (&*P - BigUint::one()) >> 1;
    mod_pow(&(n % &*P), &exp) == BigUint::one()
}

fn sqrt_mod(n: &BigUint) -> Option<BigUint> {
    let n = n % &*P;
    if n.is_zero() {
        return Some(BigUint::zero());
    }
    let legendre = mod_pow(&n, &((&*P - BigUint::one()) >> 1));
    if legendre != BigUint::one() {
        return None;
    }
    let exp = (&*P + BigUint::from(3u8)) >> 3;
    let mut root = mod_pow(&n, &exp);
    if mod_mul(&root, &root) != n {
        root = mod_mul(&root, &*SQRT_M1_BIG);
    }
    if mod_mul(&root, &root) == n {
        Some(root)
    } else {
        None
    }
}

fn mont_rhs(x: &BigUint) -> BigUint {
    let x_sq = mod_mul(x, x);
    let x_cu = mod_mul(&x_sq, x);
    let ax_sq = mod_mul(&*A_BIG, &x_sq);
    mod_add(&mod_add(&x_cu, &ax_sq), x)
}

fn mont_to_edwards(x: &BigUint, y: &BigUint) -> (BigUint, BigUint) {
    let one = BigUint::one();
    let x_plus_one = mod_add(x, &one);
    let x_minus_one = mod_sub(x, &one);
    let denom = mod_mul(&x_plus_one, y);
    let denom_inv = if denom.is_zero() {
        BigUint::zero()
    } else {
        mod_inv(&denom)
    };

    let mut ed_x = mod_mul(&mod_mul(&*SQRT_AM2_BIG, x), &denom_inv);
    ed_x = mod_mul(&ed_x, &x_plus_one);

    let mut ed_y = mod_mul(&denom_inv, y);
    ed_y = mod_mul(&ed_y, &x_minus_one);
    if denom_inv.is_zero() {
        ed_y = one;
    }

    (ed_x, ed_y)
}

fn is_negative(fe: &BigUint) -> bool {
    (fe % &*P) % 2u8 == BigUint::one()
}

fn hash_to_curve_bigint(r: &[u8; 32], x_sign: u8) -> VrfResult<EdwardsPoint> {
    let r_val = bytes_to_biguint(r);

    let mut rr2 = mod_mul(&r_val, &r_val);
    rr2 = mod_mul(&rr2, &BigUint::from(2u8));
    let denom = mod_add(&rr2, &BigUint::one());
    let denom_inv = mod_inv(&denom);

    let mut x = mod_neg(&mod_mul(&denom_inv, &*A_BIG));
    let mut gx1 = mont_rhs(&x);

    if !is_quadratic_residue(&gx1) {
        x = mod_sub(&mod_neg(&x), &*A_BIG);
        gx1 = mont_rhs(&x);
    }

    let y = sqrt_mod(&gx1).ok_or(VrfError::InvalidPoint)?;
    let (mut ed_x, ed_y) = mont_to_edwards(&x, &y);

    if is_negative(&ed_x) ^ (x_sign != 0) {
        ed_x = mod_neg(&ed_x);
    }

    let mut compressed = biguint_to_bytes(&ed_y);
    let sign_bit = is_negative(&ed_x) as u8;
    compressed[31] = (compressed[31] & 0x7f) | (sign_bit << 7);

    let point = CompressedEdwardsY(compressed)
        .decompress()
        .ok_or(VrfError::InvalidPoint)?;

    // Apply cofactor clearing and re-serialize to get correct sign bit
    let cleared = cardano_clear_cofactor(&point);

    Ok(cleared)
}

/// Cardano-specific hash-to-curve function
///
/// Maps a uniform 32-byte value to a point on the Edwards curve using the exact
/// procedure implemented by libsodium's `cardano_ge25519_from_uniform`.
///
/// # Arguments
///
/// * `r` - 32-byte uniform random value
pub fn cardano_hash_to_curve(r: &[u8; 32]) -> VrfResult<EdwardsPoint> {
    let sign = (r[31] >> 7) & 1;
    let mut r_masked = *r;
    r_masked[31] &= 0x7f;
    hash_to_curve_bigint(&r_masked, sign)
}

/// Cardano-specific cofactor clearing
///
/// Multiplies point by cofactor (8) using Cardano's method.
pub fn cardano_clear_cofactor(point: &EdwardsPoint) -> EdwardsPoint {
    use curve25519_dalek::scalar::Scalar;
    let eight = Scalar::from(8u8);
    eight * point
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
    use curve25519_dalek::traits::Identity;

    #[test]
    fn test_cofactor_clearing_with_basepoint() {
        let cleared = cardano_clear_cofactor(&ED25519_BASEPOINT_POINT);

        let identity = EdwardsPoint::identity();
        assert_ne!(
            cleared.compress().as_bytes(),
            identity.compress().as_bytes()
        );

        use curve25519_dalek::scalar::Scalar;
        let eight = Scalar::from(8u8);
        let expected = eight * ED25519_BASEPOINT_POINT;
        assert_eq!(cleared, expected);
    }
}
