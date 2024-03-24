use std::marker::PhantomData;

use plonky2::{gates::gate::Gate, hash::hash_types::RichField, iop::generator::WitnessGeneratorRef, util::serialization::Write};
use plonky2_field::extension::Extendable;


/// A gate for conditionally swapping input values based on a boolean.
#[derive(Copy, Clone, Debug)]
pub struct SwitchGate<F: RichField + Extendable<D>, const D: usize> {
    _phantom: PhantomData<F>
}

impl<F: RichField + Extendable<D>, const D: usize> Gate<F, D> for SwitchGate<F, D> {
    fn id(&self) -> String {
        format!("{self:?}<D={D}>")
    }

    fn serialize(&self, dst: &mut Vec<u8>, common_data: &plonky2::plonk::circuit_data::CommonCircuitData<F, D>) -> plonky2::util::serialization::IoResult<()> {
        dst.write_bool(false) // TODO: remove
    }

    fn deserialize(src: &mut plonky2::util::serialization::Buffer, common_data: &plonky2::plonk::circuit_data::CommonCircuitData<F, D>) -> plonky2::util::serialization::IoResult<Self>
        where
            Self: Sized {
        Ok(Self)
    }

    fn eval_unfiltered(&self, vars: plonky2::plonk::vars::EvaluationVars<F, D>) -> Vec<<F as Extendable>::Extension> {
        let mut constraints = Vec::with_capacity(4);
        let switch_bool = vars.local_wires[0];
        let not_switch = F::Extension::ONE - switch_bool;

        let input_1 = vars.local_wires[1];
        let input_2 = vars.local_wires[2];
        let output_1 = vars.local_wires[3];
        let output_2 = vars.local_wires[4];

        constraints.push(not_switch * (output_1 - input_1));
        constraints.push(not_switch * (output_2 - input_2));

        constraints.push(switch_bool * (output_2 - input_1));
        constraints.push(switch_bool * (output_1 - input_2));
        constraints
    }

    fn eval_unfiltered_circuit(
            &self,
            builder: &mut plonky2::plonk::circuit_builder::CircuitBuilder<F, D>,
            vars: plonky2::plonk::vars::EvaluationTargets<D>,
        ) -> Vec<plonky2::iop::ext_target::ExtensionTarget<D>> {
        let mut constraints = Vec::with_capacity(4);

        let one = builder.one_extension();

        let switch_bool = vars.local_wires[0];
        let not_switch = builder.sub_extension(one, switch_bool);

        let input_1 = vars.local_wires[1];
        let input_2 = vars.local_wires[2];
        let output_1 = vars.local_wires[3];
        let output_2 = vars.local_wires[4];

        constraints.push(builder.mul_extension(not_switch, builder.sub_extension(input_1, output_1)));
        constraints.push(builder.mul_extension(not_switch, builder.sub_extension(input_2, output_2)));

        constraints.push(builder.mul_extension(switch_bool, builder.sub_extension(input_1, output_2)));
        constraints.push(builder.mul_extension(switch_bool, builder.sub_extension(input_2, output_1)));

        constraints
    }

    fn generators(&self, row: usize, local_constants: &[F]) -> Vec<WitnessGeneratorRef<F, D>> {
        let g: Box<WitnessGeneratorRef<F, D>> = Box::new(SwitchGenerator::<F, D>{
            row,
            gate: *self,
            copy: c,
        });
        g
    }


    fn num_wires(&self) -> usize {
        5
    }

    fn num_constants(&self) -> usize {
        0
    }

    fn degree(&self) -> usize {
        2
    }

    fn num_constraints(&self) -> usize {
        4
    }
}
