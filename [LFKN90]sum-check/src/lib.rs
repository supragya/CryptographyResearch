use std::{
    cell::RefCell,
    rc::Rc,
};

use ark_ff::Field;
use ark_poly::{
    multivariate::{
        SparsePolynomial,
        Term,
    },
    Polynomial,
};

pub struct PolynomialOracle<F: Field, T: Term> {
    /// Remaining access
    access_counter: usize,

    /// Polynomial
    poly: SparsePolynomial<F, T>,
}

impl<F: Field, T: Term> PolynomialOracle<F, T> {
    pub fn new(
        number_of_allowed_accesses: usize,
        polynomial: SparsePolynomial<F, T>,
    ) -> Self {
        Self {
            access_counter: number_of_allowed_accesses,
            poly: polynomial,
        }
    }
}

pub struct Prover<F: Field, T: Term> {
    /// Oracle access, single use
    poly_oracle: PolynomialOracle<F, T>,

    /// Common IOP data access to get the challenges
    iop: Rc<RefCell<CommonIOPData<F>>>,
}

impl<F: Field, T: Term> Prover<F, T> {
    pub fn new(
        poly_oracle: PolynomialOracle<F, T>,
        iop: Rc<RefCell<CommonIOPData<F>>>,
    ) -> Self {
        Self { poly_oracle, iop }
    }
}

pub struct CommonIOPData<F: Field> {
    /// Number of variables in the original polynomial
    num_of_variables: usize,

    /// Prover's claim at the start of the protocol
    provers_claim: F,

    /// Random challenge points by the verifier
    challenges: Vec<F>,
}

impl<F: Field> CommonIOPData<F> {
    pub fn new(
        num_of_variables: usize,
        provers_claim: F,
    ) -> Self {
        Self {
            num_of_variables,
            provers_claim,
            challenges: vec![],
        }
    }
}

pub struct Verifier<F: Field, T: Term> {
    /// Oracle access, single use
    poly_oracle: PolynomialOracle<F, T>,

    /// Common IOP data access to get the challenges
    iop: Rc<RefCell<CommonIOPData<F>>>,
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        rc::Rc,
    };

    use ark_ff::{
        Field,
        Fp64,
        MontBackend,
        MontConfig,
        One,
        PrimeField,
    };
    use ark_poly::{
        multivariate::{
            self,
            SparsePolynomial,
            SparseTerm,
            Term,
        },
        DenseMVPolynomial,
        Polynomial,
    };
    use ark_std::{
        rand::Rng,
        test_rng,
    };
    use itertools::iproduct;

    use crate::{
        CommonIOPData,
        PolynomialOracle,
        Prover,
    };

    #[derive(MontConfig)]
    #[modulus = "7"]
    #[generator = "2"]
    struct FrConfig;

    type Fp7 = Fp64<MontBackend<FrConfig, 1>>;

    /// A utility method to generate an multivariate polynomial which is
    /// `l`-variate, with each variable having an individual max degree of
    /// `d`. `l` is provided as `number_of_vars`, `d` is provided as
    /// `individual_var_max_degree`
    fn generate_random_poly<R: Rng, F: Field>(
        number_of_vars: usize,
        individual_var_max_degree: usize,
        min_terms: usize,
        max_terms: usize,
        term_generation_probability: f64,
        rng: &mut R,
    ) -> SparsePolynomial<F, SparseTerm> {
        let terms_count = rng.gen_range(min_terms..max_terms);
        let mut poly_terms = Vec::with_capacity(terms_count);

        (1..terms_count).for_each(|_| {
            let term = (0..number_of_vars)
                .filter_map(|i| {
                    let should_generate =
                        rng.gen_bool(term_generation_probability);
                    if !should_generate {
                        return None;
                    }
                    return Some((
                        i,
                        rng.gen_range(1..individual_var_max_degree + 1),
                    ));
                })
                .collect();
            let coeff = F::rand(rng);
            poly_terms.push((coeff, SparseTerm::new(term)));
        });
        SparsePolynomial::from_coefficients_slice(number_of_vars, &poly_terms)
    }

    fn create_term(
        coefficient: u32,
        variable_degrees: Vec<(usize, usize)>,
    ) -> (Fp7, SparseTerm) {
        (
            Fp7::from_bigint(coefficient.into()).unwrap(),
            SparseTerm::new(variable_degrees),
        )
    }

    #[test]
    fn test_1() {
        let rng = &mut test_rng();

        // Create a mulitivariate polynomial in x, y, z:
        // 2(x^3) + (x)(z) + (y)(z)
        // `number_of_vars = 3`
        let number_of_variables = 3;
        let terms = vec![
            create_term(2, vec![(0, 3)]),
            create_term(1, vec![(0, 1), (2, 1)]),
            create_term(1, vec![(1, 1), (2, 1)]),
        ];

        let polynomial = SparsePolynomial::from_coefficients_vec(3, terms);

        let sum = iproduct!(0..2, 0..2, 0..2)
            .map(|eval_point| {
                let eval_point = vec![
                    eval_point
                        .0
                        .into(),
                    eval_point
                        .1
                        .into(),
                    eval_point
                        .2
                        .into(),
                ];
                let a = polynomial.evaluate(&eval_point);
                a
            })
            .sum();
        assert_eq!(sum, Fp7::from_bigint((12u32 % 7).into()).unwrap());

        let mut iop =
            Rc::new(RefCell::new(CommonIOPData::new(number_of_variables, sum)));
        let prover = Prover::new(
            PolynomialOracle::new(3, polynomial.clone()),
            iop.clone(),
        );
    }
}
