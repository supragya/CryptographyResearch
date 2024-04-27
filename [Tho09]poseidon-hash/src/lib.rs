mod constants;
use ark_bn254::Fr;
use ark_ff::Field;
use ark_std::Zero;
use std::ops::MulAssign;

#[derive(Debug)]
pub struct Constants {
    pub round_constants: Vec<Vec<Fr>>,
    pub mds_matrix: Vec<Vec<Vec<Fr>>>,
    pub full_application_round_count: usize,
    pub partial_application_round_count: Vec<usize>,
}

pub struct PoseidonHasher {
    internal_state: Vec<Fr>,
    constants: Constants,
}

impl PoseidonHasher {
    /// Full SBOX application involves raising all the state
    /// elements `x` by `x^{alpha}`. In our specific case,
    /// `alpha = 3`, hence we do as follows:
    fn apply_sbox_full(&mut self) {
        self
            .internal_state[..]
            .iter_mut()
            .for_each(|x| {
                let x_2 = x.square();
                *x *= x_2;
            })
    }

    /// Partial SBOX application involves raising only the first state
    /// element `x` by `x^{alpha}`. In our specific case,
    /// `alpha = 3`, hence we do as follows:
    fn apply_sbox_partial(&mut self) {
        self
            .internal_state[..1]
            .iter_mut()
            .for_each(|x| {
                let x_2 = x.square();
                *x *= x_2;
            })
    }

    pub fn hash(&self, input: Vec<Fr>) -> Fr {
        // We first have to chunk the input into groups of `T` size.

        // Create a state that will be mutated over multiple rounds
        // Initial state is: `[ 0 | input ]` where `0` is a single
        // zero element followed by the input
        let mut state = vec![Fr::zero(); input.len() + 1];
        state[1..].clone_from_slice(&input);

        Fr::zero()
    }
}
