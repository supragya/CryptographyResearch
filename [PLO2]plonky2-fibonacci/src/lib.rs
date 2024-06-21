#[cfg(test)]
mod tests {
    use plonky2::{
        field::types::Field,
        iop::witness::{PartialWitness, WitnessWrite},
        plonk::{
            circuit_builder::CircuitBuilder,
            circuit_data::CircuitConfig,
            config::{GenericConfig, PoseidonGoldilocksConfig},
        },
    };

    #[test]
    fn plonky2_fibonacci_basic() {
        // Defined as 2 for F's quadratic extension used in FE.
        const D: usize = 2;

        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let cfg = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(cfg);

        let initial_a = builder.add_virtual_target();
        let initial_b = builder.add_virtual_target();

        let mut prev_target = initial_a;
        let mut cur_target = initial_b;

        for _ in 0..12 {
            let temp = builder.add(prev_target, cur_target);
            prev_target = cur_target;
            cur_target = temp;
        }

        builder.register_public_input(initial_a);
        builder.register_public_input(initial_b);
        builder.register_public_input(cur_target);

        let mut pw = PartialWitness::new();
        pw.set_target(initial_a, F::ZERO);
        pw.set_target(initial_b, F::ONE);

        let data = builder.build::<C>();
        let proof_with_public_inputs = match data.prove(pw) {
            Ok(proof) => proof,
            Err(e) => {
                eprintln!("Failed to generate proof: {:?}", e);
                return;
            }
        };

        println!(
            "public inputs are: {:?}",
            proof_with_public_inputs.public_inputs
        );

        assert!(data.verify(proof_with_public_inputs).is_ok());
    }
}
