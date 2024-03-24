mod custom_gates;

use plonky2::hash::hash_types::RichField;
use plonky2_field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::iop::target::Target;

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
        2 => assert_permutation_2x2_circuit(
            builder,
            a[0],
            a[1],
            b[0],
            b[1],
        ),
        // For larger lists, we recursively use two smaller permutation networks.
        _ => assert_permutation_helper_circuit(builder, a, b),
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
    let (_switch, gate_out1, gate_out2) = create_switch_circuit(builder, a1, a2);
    for e in 0..chunk_size {
        builder.connect(b1[e], gate_out1[e]);
        builder.connect(b2[e], gate_out2[e]);
    }
}


/// Given two input wire chunks, add a new switch to the circuit (by adding one copy to a switch
/// gate). Returns the wire for the switch boolean, and the two output wire chunks.
fn create_switch_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a1: Target,
    a2: Target,
) -> (Target, Target, Target) {

    let chunk_size = a1.len();

    let gate = SwitchGate::new_from_config(&builder.config, chunk_size);
    let params = vec![F::from_canonical_usize(chunk_size)];
    let (row, next_copy) = builder.find_slot(gate, &params, &[]);

    let mut c = Vec::new();
    let mut d = Vec::new();
    for e in 0..chunk_size {
        builder.connect(
            a1[e],
            Target::wire(row, gate.wire_first_input(next_copy, e)),
        );
        builder.connect(
            a2[e],
            Target::wire(row, gate.wire_second_input(next_copy, e)),
        );
        c.push(Target::wire(row, gate.wire_first_output(next_copy, e)));
        d.push(Target::wire(row, gate.wire_second_output(next_copy, e)));
    }

    let switch = Target::wire(row, gate.wire_switch_bool(next_copy));

    (switch, c, d)
}
