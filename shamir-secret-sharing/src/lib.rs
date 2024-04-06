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
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(3091);
        let coeffs: Vec<i64> = [secret]
            .into_iter()
            .chain((0..threshold).map(|_| rng.next_u32() as i64))
            .collect();

        let polynomial = Polynomial::new_from_coeffs(&coeffs);

        // Ensure that we can get the secret back given we evaluate
        // at 0
        assert_eq!(polynomial.eval(0), secret);

        // Now evaulate at a few random points to make the secret
        // let secret_parts: Vec<(i64, i64)> = (0..threshold)
        //     .map(|_| {
        //         let point_of_eval = rng.next_u32() as i64;
        //         let eval = polynomial.eval(point_of_eval);
        //         (point_of_eval, eval)
        //     })
        //     .collect();

        // Ensure reconstruction is possible
        // let reconstructed_poly = Polynomial::new_from_evalutations()
    }
}
