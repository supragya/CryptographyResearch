use plonky2::{
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::CircuitConfig,
        config::{GenericConfig, PoseidonGoldilocksConfig},
    },
};
use plonky2_field::types::Field;

pub fn circuit_builder() {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let cfg = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(cfg);

    let initial = builder.add_virtual_target();
    let mut cur_target = initial;

    for i in 1..101 {
        let i_target = builder.constant(F::from_canonical_u32(i));
        let n_plus_i = builder.add(initial, i_target);
        cur_target = builder.mul(cur_target, n_plus_i);
    }

    builder.register_public_input(initial);

    let mut pw = PartialWitness::new();
    pw.set_target(initial, F::ONE);

    let data = builder.build::<C>();
    let proof = data.prove(pw).unwrap();

    println!(
        "Factorial starting at {} is {}",
        proof.public_inputs[0], proof.public_inputs[1]
    );

    data.verify(proof).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_facts() {
        circuit_builder()
    }
}
