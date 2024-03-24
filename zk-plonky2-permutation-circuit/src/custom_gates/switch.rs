use array_tool::vec::Union;
use std::marker::PhantomData;

use plonky2::{
    gates::gate::Gate,
    hash::hash_types::RichField,
    iop::{generator::{GeneratedValues, WitnessGenerator, WitnessGeneratorRef}, target::Target, wire::Wire, witness::{Witness, WitnessWrite, PartitionWitness}},
    plonk::circuit_data::CommonCircuitData,
    util::serialization::{Buffer, IoResult, Read, Write},
};
use plonky2_field::types::Field;
use plonky2_field::extension::Extendable;

/// A gate for conditionally swapping input values based on a boolean.
#[derive(Copy, Clone, Debug)]
pub struct SwitchGate<F: RichField + Extendable<D, Extension = F>, const D: usize> {
    _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D, Extension = F>, const D: usize> SwitchGate<F, D> {
    pub fn new() -> Self {
        Self{_phantom: PhantomData}
    }

    pub fn wire_switch_bool() -> usize {
        0
    }

    pub fn wire_input_1() -> usize {
        1
    }

    pub fn wire_input_2() -> usize {
        2
    }

    pub fn wire_output_1() -> usize {
        3
    }

    pub fn wire_output_2() -> usize {
        4
    }
}

impl<F: RichField + Extendable<D, Extension = F>, const D: usize> Gate<F, D> for SwitchGate<F, D> {
    fn id(&self) -> String {
        format!("{self:?}<D={D}>")
    }

    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_bool(false) // TODO: remove
    }

    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self>
    where
        Self: Sized,
    {
        Ok(Self{_phantom: PhantomData})
    }

    fn eval_unfiltered(
        &self,
        vars: plonky2::plonk::vars::EvaluationVars<F, D>,
    ) -> Vec<<F as Extendable<D>>::Extension> {
        let mut constraints = Vec::with_capacity(4);
        let switch_bool = vars.local_wires[Self::wire_switch_bool()];
        let not_switch = F::Extension::ONE - switch_bool;

        let input_1 = vars.local_wires[Self::wire_input_1()];
        let input_2 = vars.local_wires[Self::wire_input_2()];
        let output_1 = vars.local_wires[Self::wire_output_1()];
        let output_2 = vars.local_wires[Self::wire_output_2()];

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

        let switch_bool = vars.local_wires[Self::wire_switch_bool()];
        let not_switch = builder.sub_extension(one, switch_bool);

        let input_1 = vars.local_wires[Self::wire_input_1()];
        let input_2 = vars.local_wires[Self::wire_input_2()];
        let output_1 = vars.local_wires[Self::wire_output_1()];
        let output_2 = vars.local_wires[Self::wire_output_2()];

        constraints
            .push(builder.mul_extension(not_switch, builder.sub_extension(input_1, output_1)));
        constraints
            .push(builder.mul_extension(not_switch, builder.sub_extension(input_2, output_2)));

        constraints
            .push(builder.mul_extension(switch_bool, builder.sub_extension(input_1, output_2)));
        constraints
            .push(builder.mul_extension(switch_bool, builder.sub_extension(input_2, output_1)));

        constraints
    }

    fn generators(&self, row: usize, local_constants: &[F]) -> Vec<WitnessGeneratorRef<F, D>> {
        // unimplemented!()
        let g = Box::new(SwitchGenerator::<F, D> {
            row,
            gate: *self,
        });
        vec![g]
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

#[derive(Debug)]
struct SwitchGenerator<F: RichField + Extendable<D, Extension = F>, const D: usize> {
    row: usize,
    gate: SwitchGate<F, D>,
}

impl<F: RichField + Extendable<D, Extension = F>, const D: usize> SwitchGenerator<F, D> {
    /// List of wire targets for inputs and outputs
    fn dependencies_inputs_outputs(&self) -> Vec<Target> {
        let local_target = |column| Target::wire(self.row, column);

        let mut deps = Vec::new();

        deps.push(local_target(SwitchGate::wire_first_input()));
        deps.push(local_target(SwitchGate::wire_second_input()));
        deps.push(local_target(SwitchGate::wire_first_output()));
        deps.push(local_target(SwitchGate::wire_second_output()));

        deps
    }

    /// List of wire targets for inputs and switching boolean
    fn dependencies_switch_inputs(&self) -> Vec<Target> {
        let local_target = |column| Target::wire(self.row, column);

        let mut deps = Vec::new();

        deps.push(local_target(SwitchGate::wire_first_input()));
        deps.push(local_target(SwitchGate::wire_second_input()));
        deps.push(local_target(SwitchGate::wire_switch_bool()));

        deps
    }

    /// Run when all input and output wires are present
    fn set_switch_wire(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let get_local_wire = |column| {
            witness.get_wire(Wire {
                row: self.row,
                column,
            })
        };
        let switch_bool_wire = Wire {
            row: self.row,
            column: SwitchGate::wire_switch_bool(),
        };

        let mut input_1 = get_local_wire(SwitchGate::wire_input_1());
        let mut input_2 = get_local_wire(SwitchGate::wire_input_2());
        let mut output_1 = get_local_wire(SwitchGate::wire_output_1());
        let mut output_2 = get_local_wire(SwitchGate::wire_output_2());

        if input_1 == output_1 && input_2 == output_2 {
            out_buffer.set_wire(switch_bool_wire, F::ZERO);
        } else if input_1 == output_2 && input_2 == output_1 {
            out_buffer.set_wire(switch_bool_wire, F::ONE);
        } else {
            panic!("No permutation from given inputs to given outputs");
        }
    }

    /// Run when only inputs and switching boolean is available
    fn set_output_wires(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let get_local_wire = |column| {
            witness.get_wire(Wire {
                row: self.row,
                column,
            })
        };
        let switch_bool_wire = Wire {
            row: self.row,
            column: SwitchGate::wire_switch_bool(),
        };

        let mut input_1 = get_local_wire(SwitchGate::wire_input_1());
        let mut input_2 = get_local_wire(SwitchGate::wire_input_2());
        let mut output_1 = get_local_wire(SwitchGate::wire_output_1());
        let mut output_2 = get_local_wire(SwitchGate::wire_output_2());

        let (expected_output_1, expected_output_2) = if switch_bool_wire == F::ZERO {
            (input_1, input_2)
        } else if switch_bool_wire == F::ONE {
            (input_2, input_1)
        } else {
            panic!("Invalid switch bool value");
        };

        out_buffer.set_wire(output_1, expected_output_1);
        out_buffer.set_wire(output_2, expected_output_2);
    }
}

impl<F: RichField + Extendable<D>, const D: usize> WitnessGenerator<F, D> for SwitchGenerator<F, D> {
    fn id(&self) -> String {
        format!("{self:?}<D={D}>")
    }

    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_usize(self.row)
    }

    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self>
        where
            Self: Sized {
        Ok(Self{row: src.read_bool().unwrap(), gate: SwitchGate::new()})
    }
    /// Register the different columns to watch
    fn watch_list(&self) -> Vec<Target> {
        self.dependencies_inputs_outputs()
            .union(self.dependencies_switch_inputs())
    }

    /// Figure out which columns change and set the remaining
    /// Can work in two modes:
    /// 1. If input and switch wires are pre-populated
    /// 2. If input and output wires are pre-populated
    fn run(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) -> bool {
        if witness.contains_all(&self.dependencies_switch_inputs()) {
            self.set_output_wires(witness, out_buffer);
            true
        } else if witness.contains_all(&self.dependencies_inputs_outputs()) {
            self.set_switch_wire(witness, out_buffer);
            true
        } else {
            false
        }
    }
}
