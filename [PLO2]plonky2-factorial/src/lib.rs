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
    fn plonky2_factorial_example() {
        // This constant defines the extension that we will
        // use. Almost always it is going to be 2
        const D: usize = 2;

        // Generate configuration where we are using Golidilock's
        // prime field, it's quadratic extension, Poseidon as the
        // hashing function for the merkle tree in FRI stage, and
        // the same Poseidon as the hasher for Fiat-Shamir (for
        // challenge points generation). See `KeccakGoldilocksCo-
        // -nfig`) for when to target Ethereum.
        type C = PoseidonGoldilocksConfig;

        // Get the Field type that we are using
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_recursion_config();

        let mut builder = CircuitBuilder::<F, D>::new(config);

        let initial = builder.add_virtual_target();
        let mut cur_target = initial;
        for i in 2..101 {
            let multiplier = builder.constant(F::from_canonical_u32(i));
            let cur_target = builder.mul(cur_target, multiplier);
        }

        builder.register_public_input(initial);
        builder.register_public_input(cur_target);

        let mut pw = PartialWitness::new();
        pw.set_target(initial, F::ONE);

        let data = builder.build::<C>();
        let proof = data.prove(pw).unwrap();

        core::assert!(data.verify(proof).is_ok());
    }
}
