use ark_bls12_381::G1Projective;
use ark_ec::Group;

#[allow(dead_code)]
fn generate_two_powers(point: G1Projective, n: usize) -> Vec<G1Projective> {
    if n < 2 {
        panic!();
    }
    let mut powers: Vec<G1Projective> = Vec::with_capacity(n);
    powers.push(G1Projective::default());
    powers.push(point);
    for idx in 2..n {
        powers.push(powers[idx - 1].double());
    }
    powers
}

#[allow(dead_code)]
fn point_exponentiation(exponent: u32, powers_of_2: &Vec<G1Projective>) -> G1Projective {
    // Initialize the result to the identity element (zero point) of the group
    let mut result = G1Projective::default();

    // Iterate through each bit of the exponent
    for i in 0..32 {
        if (exponent >> i) & 1 == 1 {
            // If the i-th bit is set in the exponent, add the corresponding power_of_2 to the result
            result += &powers_of_2[i as usize];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    // use ark_ec::Group;
    // use polynomial::Polynomial;
    // // use rand::prelude::*;
    // use super::*;

    // #[test]
    // fn feldman_verifiable_secret_sharing_low_f64() {
    //     // Let's say we want to create a verifyable secret scheme
    //     // for hiding S = 5
    //     let secret = 5;

    //     // Assume that we want to break secret into 4 "parts",
    //     // of which any 2 should be able to reconstruct the
    //     // original secret
    //     let parts = 4;

    //     let coeffs: Vec<u32> = vec![secret as u32, 4];

    //     let generator_two_powers =
    //         generate_two_powers(ark_bls12_381::G1Projective::generator(), 32);

    //     let public_g_raised_coeffs: Vec<G1Projective> = coeffs
    //         .iter()
    //         .map(|x| point_exponentiation(*x, &generator_two_powers))
    //         .collect();

    //     let coeffs: Vec<f64> = coeffs.into_iter().map(|x| x as f64).collect();

    //     let polynomial = Polynomial::new_from_coeffs(&coeffs);

    //     // Ensure that we can get the secret back given we evaluate
    //     // at 0
    //     assert_eq!(polynomial.eval(0.0) as f64, secret as f64);

    //     // Now evaulate at a few random points to make the secret
    //     let secret_parts: Vec<(f64, f64)> = (1..(parts + 1))
    //         .map(|x| {
    //             let eval = polynomial.eval((x * 10) as f64);
    //             ((x * 10) as f64, eval)
    //         })
    //         .collect();

    //     // Ensure reconstruction is possible
    //     let secrets_revealed = [secret_parts[0], secret_parts[3]];
    //     let reconstructed_poly = Polynomial::new_from_evals(&secrets_revealed);
    //     // Now that we have reconstructed the polynomial from secret parts,
    //     // we need to ensure that those "secrets" are verifyably correct.
    //     // This is where VSS kicks in. For this, we use `public_g_raised_coeffs`
    //     let reconstructed_coeffs: Vec<u32> = reconstructed_poly
    //         .get_raw_coefficients()
    //         .into_iter()
    //         .map(|x| x as u32)
    //         .collect();

    //     let a_powers: Vec<Vec<G1Projective>> = public_g_raised_coeffs
    //         .into_iter()
    //         .map(|x| generate_two_powers(x, 32))
    //         .collect();

    //     for (x, eval) in secrets_revealed {
    //         let mut running_x_pow = 1.0;
    //         let mut running_g_multiplier = G1Projective::default();
    //         for (idx, coeff) in reconstructed_coeffs.iter().enumerate() {
    //             running_g_multiplier =
    //                 running_g_multiplier * point_exponentiation(*coeff, &a_powers[idx])
    //         }
    //     }

    //     // assert!((reconstructed_poly.eval(0.0) - (secret as f64)).abs() < 0.0000001);
    // }
}
