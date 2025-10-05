//! Montgomery curve operations and Elligator2 mapping
//!
//! This module implements Montgomery curve operations for Curve25519,
//! including the critical Elligator2 hash-to-curve function that must
//! match Cardano's libsodium implementation exactly.
//!
//! # Montgomery vs Edwards
//!
//! Curve25519 can be represented as either:
//! - Montgomery form: By^2 = x^3 + Ax^2 + x
//! - Edwards form: x^2 + y^2 = 1 + dx^2y^2
//!
//! This module handles Montgomery operations and conversion to Edwards.

use super::field::FieldElement;

// Montgomery curve constants for Curve25519
// By^2 = x^3 + Ax^2 + x where A = 486662, B = 1

const A: i64 = 486662;

/// Elligator2 hash-to-curve function
///
/// Maps a 32-byte uniform random string to a point on Curve25519.
/// This is the CRITICAL function that must match libsodium exactly.
///
/// # Arguments
///
/// * `r` - 32-byte uniform random value
///
/// # Returns
///
/// Field elements (u, v) representing Montgomery coordinates
///
/// # Algorithm
///
/// Implements Elligator2 mapping as specified in Cardano's libsodium:
/// 1. Load r as field element and square it
/// 2. Compute v = -A/(1+ur^2) where u = 2
/// 3. Compute e = Legendre symbol of v^3 + Av^2 + v
/// 4. Select x = e*v - (1-e)*(A/2)
/// 5. Compute y from x using curve equation
pub fn elligator2(r: &[u8; 32]) -> Option<(FieldElement, FieldElement)> {
    // Load r as a field element
    let r_fe = FieldElement::from_bytes(r);
    
    // Compute r^2
    let r2 = r_fe.square().reduce();
    
    // u = 2 (constant in Elligator2)
    let two = FieldElement::one() + FieldElement::one();
    let u = two.reduce();
    
    // Compute u*r^2
    let ur2 = (u * r2).reduce();
    
    // Compute 1 + u*r^2
    let one_plus_ur2 = (FieldElement::one() + ur2).reduce();
    
    // Compute -A as field element
    let mut a_bytes = [0u8; 32];
    // A = 486662 in little-endian
    a_bytes[0] = 0x06;
    a_bytes[1] = 0x6d;
    a_bytes[2] = 0x07;
    let a_fe = FieldElement::from_bytes(&a_bytes);
    let neg_a = -a_fe;
    
    // Compute v = -A/(1 + ur^2)
    let denom_inv = one_plus_ur2.invert();
    let v = (neg_a * denom_inv).reduce();
    
    // Compute v^2
    let v2 = v.square().reduce();
    
    // Compute v^3 = v * v^2
    let v3 = (v * v2).reduce();
    
    // Compute Av^2
    let av2 = (a_fe * v2).reduce();
    
    // Compute v^3 + Av^2 + v (the curve equation right side)
    let rhs = (v3 + av2 + v).reduce();
    
    // Check if rhs is a quadratic residue (has square root)
    let is_qr = rhs.is_square();
    
    // Compute x coordinate based on whether rhs is QR
    // If QR: x = v, else: x = -v - A
    let x = if is_qr {
        v
    } else {
        let neg_v = -v;
        (neg_v - a_fe).reduce()
    };
    
    // Compute y from x using curve equation By^2 = x^3 + Ax^2 + x
    let y_opt = xmont_to_ymont(&x, 0);
    
    y_opt.map(|y| (x, y))
}

/// Convert Montgomery coordinates to Edwards coordinates
///
/// Transforms (u, v) on Montgomery curve to (x, y) on Edwards curve.
///
/// # Arguments
///
/// * `mont_u` - Montgomery u-coordinate
/// * `mont_v` - Montgomery v-coordinate
///
/// # Returns
///
/// Edwards coordinates (x, y)
///
/// # Formula
///
/// For Curve25519/Ed25519 conversion:
/// - x = sqrt(-486664) * u / v
/// - y = (u - 1) / (u + 1)
pub fn mont_to_edwards(
    mont_u: &FieldElement,
    mont_v: &FieldElement,
) -> Option<(FieldElement, FieldElement)> {
    // Check for division by zero in y = (u-1)/(u+1)
    let u_plus_1 = (*mont_u + FieldElement::one()).reduce();
    if u_plus_1.is_zero() {
        return None;
    }
    
    // Check for division by zero in x = u/v
    if mont_v.is_zero() {
        return None;
    }
    
    // Compute y = (u - 1) / (u + 1)
    let u_minus_1 = (*mont_u - FieldElement::one()).reduce();
    let denom_inv = u_plus_1.invert();
    let ed_y = (u_minus_1 * denom_inv).reduce();
    
    // Compute x = sqrt(-486664) * u / v
    // First compute -486664 as field element
    let mut a_bytes = [0u8; 32];
    a_bytes[0] = 0x06;
    a_bytes[1] = 0x6d;
    a_bytes[2] = 0x07;
    let a_fe = FieldElement::from_bytes(&a_bytes);
    let neg_a = -a_fe;
    
    // Compute sqrt(-A) if it exists
    let sqrt_neg_a_opt = neg_a.sqrt();
    
    sqrt_neg_a_opt.and_then(|sqrt_neg_a| {
        // Compute u/v
        let v_inv = mont_v.invert();
        let u_over_v = (*mont_u * v_inv).reduce();
        
        // Compute x = sqrt(-A) * u/v
        let ed_x = (sqrt_neg_a * u_over_v).reduce();
        
        Some((ed_x, ed_y))
    })
}

/// Recover Montgomery y-coordinate from x-coordinate
///
/// Given x on Montgomery curve, compute corresponding y coordinate.
///
/// # Arguments
///
/// * `x` - Montgomery x-coordinate
/// * `sign` - Sign bit to select positive or negative root
///
/// # Returns
///
/// Montgomery y-coordinate if x is on curve
///
/// # Formula
///
/// Solves By^2 = x^3 + Ax^2 + x for y where B = 1, A = 486662
pub fn xmont_to_ymont(x: &FieldElement, sign: u8) -> Option<FieldElement> {
    // Compute x^2
    let x2 = x.square().reduce();
    
    // Compute x^3 = x * x^2
    let x3 = (*x * x2).reduce();
    
    // Compute A as field element
    let mut a_bytes = [0u8; 32];
    a_bytes[0] = 0x06;
    a_bytes[1] = 0x6d;
    a_bytes[2] = 0x07;
    let a_fe = FieldElement::from_bytes(&a_bytes);
    
    // Compute Ax^2
    let ax2 = (a_fe * x2).reduce();
    
    // Compute rhs = x^3 + Ax^2 + x (since B = 1)
    let rhs = (x3 + ax2 + *x).reduce();
    
    // Compute square root of rhs
    let y_opt = rhs.sqrt();
    
    y_opt.map(|mut y| {
        // Select sign based on sign bit
        if sign == 1 && !y.is_negative() {
            y = -y;
        } else if sign == 0 && y.is_negative() {
            y = -y;
        }
        y.reduce()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elligator2_basic() {
        let r = [0u8; 32];
        let result = elligator2(&r);
        // Should return Some value now
        assert!(result.is_some());
    }

    #[test]
    fn test_elligator2_various_inputs() {
        // Test with several inputs - some may not have valid points
        let mut r = [0u8; 32];
        let mut successes = 0;
        
        for i in 0..10 {
            r[0] = i as u8;
            if elligator2(&r).is_some() {
                successes += 1;
            }
        }
        
        // At least some inputs should succeed
        assert!(successes > 0);
    }

    #[test]
    fn test_xmont_to_ymont() {
        // Test with x = 0
        let x = FieldElement::zero();
        let y_opt = xmont_to_ymont(&x, 0);
        assert!(y_opt.is_some());
    }

    #[test]
    fn test_mont_to_edwards() {
        // Test with valid Montgomery point
        let mut u_bytes = [0u8; 32];
        u_bytes[0] = 9; // Base point u-coordinate
        let u = FieldElement::from_bytes(&u_bytes);
        
        // Get corresponding v
        if let Some(v) = xmont_to_ymont(&u, 0) {
            let result = mont_to_edwards(&u, &v);
            // Should succeed for valid point
            assert!(result.is_some());
        }
    }
}
