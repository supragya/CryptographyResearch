mod custom_gates;

use custom_gates::switch::SwitchGate;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_field::extension::Extendable;

// Inspired by https://github.com/0xPolygonZero/plonky2-waksman/blob/main/src/permutation.rs

/// Assert that two set of targets are permutation of each other
pub fn assert_permutation_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Vec<Target>,
    b: Vec<Target>,
) {
    assert_eq!(
        a.len(),
        b.len(),
        "Permutation must have same number of inputs and outputs"
    );

    match a.len() {
        // Two empty lists are permutations of one another, trivially.
        0 => (),
        // Two singleton lists are permutations of one another as long as their items are equal.
        1 => {
            builder.connect(a[0], b[0]);
        }
        2 => assert_permutation_2x2_circuit(builder, a[0], a[1], b[0], b[1]),
        // For larger lists, we recursively use two smaller permutation networks.
        _ => unimplemented!(), // assert_permutation_helper_circuit(builder, a, b),
    }
}

/// Assert that [a1, a2] is a permutation of [b1, b2].
fn assert_permutation_2x2_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a1: Target,
    a2: Target,
    b1: Target,
    b2: Target,
) {
    let (_switch, out_1, out_2) = create_switch_circuit(builder, a1, a2);
    // Add constraints
    builder.connect(b1, out_1);
    builder.connect(b2, out_2);
}

/// Given two input wire chunks, add a new switch to the circuit (by adding one copy to a switch
/// gate). Returns the wire for the switch boolean, and the two output wire chunks.
fn create_switch_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a1: Target,
    a2: Target,
) -> (Target, Target, Target) {
    let gate = SwitchGate::new();
    let (row, _next_copy) = builder.find_slot(gate, &vec![], &[]);

    builder.connect(a1, Target::wire(row, SwitchGate::wire_input_1()));
    builder.connect(a2, Target::wire(row, SwitchGate::wire_input_2()));

    (
        Target::wire(row, SwitchGate::wire_switch_bool()),
        Target::wire(row, SwitchGate::wire_output_1()),
        Target::wire(row, SwitchGate::wire_output_2()),
    )
}
