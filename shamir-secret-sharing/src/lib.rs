#[cfg(test)]
mod tests {
    use polynomial::Polynomial;
    use rand::prelude::*;

    #[test]
    fn shamir_secret_sharing() {
        // Let's say we want to create a shamir secret scheme
        // for hiding S = 119
        let secret = 119;

        // Assume that we want to break secret into 4 "parts",
        // of which any 2 should be able to reconstruct the
        // original secret
        let parts = 4;
        let threshold = 2;

        // Hence, coefficients will be of form `secret + r1*x + r2*x^2...`
        // where `r` is random values from `(0, threshold)`
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
        let coeffs: Vec<f64> = [secret as f64]
            .into_iter()
            .chain((0..threshold-1).map(|_| rng.next_u32() as f64))
            .collect();

        println!("{:?}", coeffs);

        let polynomial = Polynomial::new_from_coeffs(&coeffs);

        // Ensure that we can get the secret back given we evaluate
        // at 0
        assert_eq!(polynomial.eval(0.0) as i64, secret);

        // Now evaulate at a few random points to make the secret
        let secret_parts: Vec<(f64, f64)> = (0..parts)
            .map(|idx| {
                let eval = polynomial.eval(idx as f64 + 1.0); // + 1.0 required as we do not want to generate at 0
                (idx as f64 + 1.0, eval)
            })
            .collect();

        // Ensure reconstruction is possible
        let reconstructed_poly = Polynomial::new_from_evals(&[secret_parts[0], secret_parts[3]]);

        assert!(reconstructed_poly.eval(0.0)-(secret as f64) < 0.0000001); // due to f64 inaccuracy
    }
}
