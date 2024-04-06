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
        powers.push(powers[idx-1].double());
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
    use ark_ec::Group;
    use polynomial::Polynomial;
    use rand::prelude::*;
    use super::*;

    #[test]
    fn feldman_verifiable_secret_sharing() {
        // Let's say we want to create a verifyable secret scheme
        // for hiding S = 213
        let secret = 213;

        // Assume that we want to break secret into 4 "parts",
        // of which any 2 should be able to reconstruct the
        // original secret
        let parts = 4;
        let threshold = 2;

        // Hence, coefficients will be of form `secret + r1*x + r2*x^2...`
        // where `r` is random values from `(0, threshold)`
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
        let coeffs: Vec<u32> = [secret]
            .into_iter()
            .chain((0..threshold - 1).map(|_| rng.next_u32()))
            .collect();
    
        println!("Original {:?}", coeffs);


        let generator_two_powers = generate_two_powers(ark_bls12_381::G1Projective::generator(), 32);

        let public_g_raised_coeffs: Vec<G1Projective> = coeffs
            .iter()
            .map(|x| point_exponentiation(*x, &generator_two_powers))
            .collect();

        let coeffs: Vec<f64> = coeffs.into_iter().map(|x| x as f64).collect();

        let polynomial = Polynomial::new_from_coeffs(&coeffs);

        // Ensure that we can get the secret back given we evaluate
        // at 0
        assert_eq!(polynomial.eval(0.0) as f64, secret as f64);

        // Now evaulate at a few random points to make the secret
        let secret_parts: Vec<(f64, f64)> = (0..parts)
            .map(|_| {
                let point_of_eval = rng.next_u32() as f64;
                let eval = polynomial.eval(point_of_eval); // + 1.0 required as we do not want to generate at 0
                (point_of_eval, eval)
            })
            .collect();

        // Ensure reconstruction is possible
        let reconstructed_poly = Polynomial::new_from_evals(&[secret_parts[0], secret_parts[3]]);
        assert!((reconstructed_poly.eval(0.0) - (secret as f64)).abs() < 0.0000001); // due to f64 inaccuracy

        let reconstructed_coeffs = reconstructed_poly.get_raw_coefficients();
        println!("{:?}", reconstructed_coeffs);
        println!("{:?}", reconstructed_poly.eval(0.0) - (secret as f64));
        println!("{:?}", secret as f64);
    }
}
