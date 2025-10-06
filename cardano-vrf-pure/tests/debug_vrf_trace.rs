//! Debug test to trace VRF computation step-by-step
//!
//! This test uses the vrf_ver03_generated_1 vector to debug our implementation

use cardano_vrf_pure::cardano_compat::field::FieldElement;
use cardano_vrf_pure::cardano_compat::montgomery;
use cardano_vrf_pure::cardano_compat::point::cardano_hash_to_curve;
use cardano_vrf_pure::cardano_compat::{cardano_vrf_prove, cardano_vrf_verify};
use curve25519_dalek::montgomery::MontgomeryPoint as DalekMontgomeryPoint;
use sha2::{Digest, Sha512};

#[test]
fn debug_vrf_ver03_generated_1() {
    // Test vector vrf_ver03_generated_1
    let sk_hex = "0000000000000000000000000000000000000000000000000000000000000000";
    let pk_hex = "3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29";
    let alpha_hex = "00";
    let expected_pi_hex = "000f006e64c91f84212919fe0899970cd341206fc081fe599339c8492e2cea3299ae9de4b6ce21cda0a975f65f45b70f82b3952ba6d0dbe11a06716e67aca233c0d78f115a655aa1952ada9f3d692a0a";
    let expected_beta_hex = "9930b5dddc0938f01cf6f9746eded569ee676bd6ff3b4f19233d74b903ec53a45c5728116088b7c622b6d6c354f7125c7d09870b56ec6f1e4bf4970f607e04b2";

    let sk_seed = hex::decode(sk_hex).unwrap();
    let pk = hex::decode(pk_hex).unwrap();
    let alpha = hex::decode(alpha_hex).unwrap();
    let expected_pi = hex::decode(expected_pi_hex).unwrap();
    let expected_beta = hex::decode(expected_beta_hex).unwrap();

    // Construct 64-byte secret key
    let mut skpk = [0u8; 64];
    skpk[0..32].copy_from_slice(&sk_seed);
    skpk[32..64].copy_from_slice(&pk);

    println!("\n=== VRF Debug Trace ===");
    println!("sk: {}", hex::encode(&sk_seed));
    println!("pk: {}", hex::encode(&pk));
    println!("alpha: {}", hex::encode(&alpha));

    // Step 1: Expand secret key
    let mut hasher = Sha512::new();
    hasher.update(&sk_seed);
    let az_hash = hasher.finalize();
    let mut az = [0u8; 64];
    az.copy_from_slice(&az_hash);
    println!("\naz (before clamp): {}", hex::encode(&az));

    // Step 2: Clamp scalar
    az[0] &= 248;
    az[31] &= 127;
    az[31] |= 64;
    println!("az (after clamp): {}", hex::encode(&az));

    // Step 3: Compute r_string = SHA512(suite || 0x01 || pk || alpha)
    let mut h_hasher = Sha512::new();
    h_hasher.update(&[0x03]); // SUITE_DRAFT03
    h_hasher.update(&[0x01]); // ONE
    h_hasher.update(&pk);
    h_hasher.update(&alpha);
    let r_string = h_hasher.finalize();
    println!("\nr_string: {}", hex::encode(&r_string));

    let mut r_bytes = [0u8; 32];
    r_bytes.copy_from_slice(&r_string[0..32]);
    println!("r_bytes (first 32): {}", hex::encode(&r_bytes));

    r_bytes[31] &= 0x7f;
    println!("r_bytes (sign cleared): {}", hex::encode(&r_bytes));

    if let Some(u_fe) = montgomery::elligator2(&r_bytes) {
        let r_fe = FieldElement::from_bytes(&r_bytes);
        let mut rr2 = r_fe.square2();
        println!("rr2 raw: {}", hex::encode(rr2.to_bytes()));
        rr2 = rr2.reduce();
        println!("rr2 reduced: {}", hex::encode(rr2.to_bytes()));
        let rr2_plus = (rr2 + FieldElement::one()).reduce();
        println!("rr2 + 1: {}", hex::encode(rr2_plus.to_bytes()));
        let rr2_inv = rr2_plus.invert();
        println!("rr2_inv: {}", hex::encode(rr2_inv.to_bytes()));
        let check = (rr2_plus * rr2_inv).reduce();
        println!("rr2 * inv: {}", hex::encode(check.to_bytes()));

        let expected_u_bytes =
            hex::decode("58a9499d48d9ec7ee9aeaf05035c05decff66beca27d8bf7bf374363f5dc0a5e")
                .unwrap();
        let mut expected_u_array = [0u8; 32];
        expected_u_array.copy_from_slice(&expected_u_bytes);
        let expected_u = FieldElement::from_bytes(&expected_u_array);
        let diff_expected = (u_fe - expected_u).reduce();
        println!(
            "diff (ours - expected): {}",
            hex::encode(diff_expected.to_bytes())
        );

        let Some(v_fe) = montgomery::xmont_to_ymont(&u_fe, 0) else {
            println!("Elligator2 produced u without recoverable v");
            return;
        };

        let u_bytes = u_fe.to_bytes();
        let v_bytes = v_fe.to_bytes();
        println!("\nElligator2 u: {}", hex::encode(u_bytes));
        println!("Elligator2 v: {}", hex::encode(v_bytes));

        let u_high_bit = u_bytes[31] >> 7;
        println!("u high bit: {}", u_high_bit);

        // Verify Montgomery curve equation: v^2 = u^3 + Au^2 + u
        let u2 = u_fe.square().reduce();
        let u3 = (u_fe * u2).reduce();
        let a_fe = {
            let mut bytes = [0u8; 32];
            let a: u64 = 486662;
            bytes[0] = (a & 0xff) as u8;
            bytes[1] = ((a >> 8) & 0xff) as u8;
            bytes[2] = ((a >> 16) & 0xff) as u8;
            FieldElement::from_bytes(&bytes)
        };
        let au2 = (a_fe * u2).reduce();
        let rhs = (u3 + au2 + u_fe).reduce();
        let v2 = v_fe.square().reduce();
        println!(
            "Montgomery equation holds: {}",
            v2.to_bytes() == rhs.to_bytes()
        );
        println!("rhs is_square: {}", rhs.is_square());

        let alt_u = ((-u_fe) - a_fe).reduce();
        let alt_u_bytes = alt_u.to_bytes();
        println!("Alt Elligator2 u: {}", hex::encode(alt_u_bytes));
        let mont_primary = DalekMontgomeryPoint(u_bytes);
        let mont_alt = DalekMontgomeryPoint(alt_u_bytes);
        println!(
            "primary to_edwards ok: {}",
            mont_primary.to_edwards(0).is_some()
        );
        println!(
            "primary to_edwards (sign 1) ok: {}",
            mont_primary.to_edwards(1).is_some()
        );
        println!(
            "alternate to_edwards ok: {}",
            mont_alt.to_edwards(0).is_some()
        );

        if let Some((alt_x, alt_y)) = montgomery::mont_to_edwards(&alt_u, &v_fe) {
            println!("alt branch edwards eq: {}", {
                let x2 = alt_x.square().reduce();
                let y2 = alt_y.square().reduce();
                let lhs = (x2 + y2).reduce();
                let d = FieldElement([
                    -10913610, 13857413, -15372611, 6949391, 114729, -8787816, -6275908, -3247719,
                    -18696448, -12055116,
                ]);
                let rhs = (FieldElement::one() + (d * (x2 * y2).reduce()).reduce()).reduce();
                lhs.to_bytes() == rhs.to_bytes()
            });
        }

        if let Some((custom_x, custom_y)) = montgomery::mont_to_edwards(&u_fe, &v_fe) {
            let custom_x_bytes = custom_x.to_bytes();
            let custom_y_bytes = custom_y.to_bytes();
            println!("custom ed_x: {}", hex::encode(custom_x_bytes));
            println!("custom ed_y: {}", hex::encode(custom_y_bytes));

            let custom_x2 = custom_x.square().reduce();
            let custom_y2 = custom_y.square().reduce();
            let lhs = (custom_x2 + custom_y2).reduce();
            let d = FieldElement([
                -10913610, 13857413, -15372611, 6949391, 114729, -8787816, -6275908, -3247719,
                -18696448, -12055116,
            ]);
            let xy2 = (custom_x2 * custom_y2).reduce();
            let rhs = (FieldElement::one() + (d * xy2).reduce()).reduce();
            println!(
                "edwards equation holds: {}",
                lhs.to_bytes() == rhs.to_bytes()
            );
            let diff = (lhs - rhs).reduce();
            println!("edwards lhs-rhs: {}", hex::encode(diff.to_bytes()));

            let one = FieldElement::one();
            let numerator = (u_fe - one).reduce();
            let denominator = (u_fe + one).reduce();
            let denominator_inv = denominator.invert();
            let direct_y = (numerator * denominator_inv).reduce();
            println!("direct ed_y: {}", hex::encode(direct_y.to_bytes()));
            let direct_diff = (direct_y - custom_y).reduce();
            println!(
                "direct_y - custom_y: {}",
                hex::encode(direct_diff.to_bytes())
            );

            let sqrt_am2 = FieldElement::SQRT_AM2;
            let mut direct_x = (sqrt_am2 * u_fe).reduce();
            let v_inv = v_fe.invert();
            direct_x = (direct_x * v_inv).reduce();
            println!("direct ed_x: {}", hex::encode(direct_x.to_bytes()));
            let x_diff = (direct_x - custom_x).reduce();
            println!("direct_x - custom_x: {}", hex::encode(x_diff.to_bytes()));

            let mut y_bytes = custom_y_bytes;
            y_bytes[31] &= 0x7f;
            for bit in 0..=1 {
                y_bytes[31] = (y_bytes[31] & 0x7f) | ((bit as u8) << 7);
                let compressed = curve25519_dalek::edwards::CompressedEdwardsY(y_bytes);
                println!(
                    "compressed with sign {} decompresses: {}",
                    bit,
                    compressed.decompress().is_some()
                );
            }
        }
    } else {
        println!("\nElligator2 returned None");
    }

    match cardano_hash_to_curve(&r_bytes) {
        Ok(hash_point) => {
            let h_bytes = hash_point.compress().to_bytes();
            println!("hash_to_curve -> H: {}", hex::encode(h_bytes));
        },
        Err(e) => {
            println!("hash_to_curve failed: {:?}", e);
        },
    }

    // Now call the actual prove function
    println!("\n=== Calling cardano_vrf_prove ===");
    match cardano_vrf_prove(&skpk, &alpha) {
        Ok(proof) => {
            println!("Generated proof: {}", hex::encode(&proof));
            println!("Expected proof:  {}", hex::encode(&expected_pi));
            println!("Match: {}", proof[..] == expected_pi[..]);

            // Try verification
            let pk_array: [u8; 32] = pk.try_into().unwrap();
            match cardano_vrf_verify(&pk_array, &proof, &alpha) {
                Ok(beta) => {
                    println!("\nGenerated beta: {}", hex::encode(&beta));
                    println!("Expected beta:  {}", hex::encode(&expected_beta));
                    println!("Match: {}", beta[..] == expected_beta[..]);
                },
                Err(e) => {
                    println!("\nVerification failed: {:?}", e);
                },
            }
        },
        Err(e) => {
            println!("Prove failed: {:?}", e);
        },
    }
}
