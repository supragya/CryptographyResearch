use anyhow::Result;
use plonky2::{
    iop::{
        target::Target,
        witness::{PartialWitness, WitnessWrite},
    },
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::CircuitConfig,
        config::{GenericConfig, PoseidonGoldilocksConfig},
    },
};
use plonky2_field::types::Field;

pub fn circuit_builder() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    // 100 bits of security standard recursion config
    let cfg = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(cfg);

    // Add 81 virtual targets (from 0 to 80), these will show up as
    // Target::VirtualTarget { index: row*9 + col} in sudoku
    let targets: Vec<Target> = (0..81).map(|_| builder.add_virtual_target()).collect();

    let constants_0_to_9: Vec<Target> = (0..9).map(|i| builder.constant(F::from(i + 1))).collect();

    // Constraint: each row should have a values 1 through 9
    // This is checked via a permuation check on constants 0-9
    (0..9).for_each(|row| {
        assert_permutation_circuit(builder, constants_0_to_9, targets[row * 9..(row + 1) * 9])
    });

    // plonky2_waksman::permutation::assert_permutation_circuit(builder, a, b);

    // Add constraints: Each row should have
    println!("{:#?}", targets);

    Ok(())
    // let initial_a = builder.add_virtual_target();
    // let initial_b = builder.add_virtual_target();

    // // println!("{:#?}", initial_b);

    // let mut prev_target = initial_a;
    // let mut cur_target = initial_b;
    // for _ in 0..99 {
    //     let temp = builder.add(prev_target, cur_target);
    //     prev_target = cur_target;
    //     cur_target = temp;
    // }

    // // Public inputs are the two initial values (provided below) and the result (which is generated).
    // builder.register_public_input(initial_a);
    // builder.register_public_input(initial_b);
    // builder.register_public_input(cur_target);

    // let mut pw = PartialWitness::new();
    // pw.set_target(initial_a, F::ZERO);
    // pw.set_target(initial_b, F::ONE);

    // let data = builder.build::<C>();
    // let proof = data.prove(pw)?;

    // println!(
    //     "100th Fibonacci number mod |F| (starting with {}, {}) is: {}",
    //     proof.public_inputs[0], proof.public_inputs[1], proof.public_inputs[2]
    // );

    // data.verify(proof)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_facts() {
        circuit_builder().unwrap()
    }
}
