use ark_bn254::Fr;
use ark_std::Zero;

#[derive(Debug)]
pub struct Constants {
    // TODO: what is this
    pub c: Vec<Vec<Fr>>,

    // TODO: what is this
    pub m: Vec<Vec<Vec<Fr>>>,

    // TODO: what is this
    pub full_application_round_count: usize,

    // TODO: what is this
    pub partial_application_round_count: Vec<usize>,
}

pub struct PoseidonHasher {
    constants: Constants,
}

impl PoseidonHasher {
    pub fn hash(&self, input: Vec<Fr>) -> Fr {
        // First, we cannot handle empty or too large inputs
        if input.is_empty() || input.len() > self.constants.partial_application_round_count.len() {
            panic!("cannot handle");
        }

        // Create a state that will be mutated over multiple rounds
        // Initial state is: `[ 0 | input ]` where `0` is a single
        // zero element followed by the input
        let mut state = vec![Fr::zero(); input.len() + 1];
        state[1..].clone_from_slice(&input);

        Fr::zero()
    }
}
