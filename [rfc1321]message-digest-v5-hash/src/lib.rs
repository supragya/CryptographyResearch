/// A generalized trait for hashing systems using
/// a Merkle–Damgård construction
trait MerkleDamgard {
    type InternalState;
    type Chunk;
    type HashOutput;

    /// An MD-compliant input padding logic
    fn pad_input(message: &[u8]) -> Vec<Self::Chunk>;

    /// Compressor step to consume a chunk and affect the state
    fn apply_compressor(state: &mut Self::InternalState, chunk: Self::Chunk);

    /// Final output producer
    fn finalize() -> Self::HashOutput;
}

struct MessageDigestV5 {
    round_constants: [u32; 64],
    shift_amounts: [u32; 64],
    initialization_state: [u32; 4],
}

impl MessageDigestV5 {
    #[rustfmt::skip]
    pub fn new() -> Self {
        // The table formula function:
        // K[i] = floor(2^32 * abs(sin(i))
        fn round_constant_generator(i: u32) -> u32 {
            let x: f64 = i as f64;
            let sin_eval = x.sin().abs();

            // note: 4294967296 == 2^32
            (4294967296.0 * sin_eval) as u32
        }

        Self {
            round_constants: (0..64)
                .map(|x| round_constant_generator(x+1))
                .collect::<Vec<u32>>()[0..64]
                .try_into()
                .unwrap(),
            shift_amounts: [
                [7, 12, 17, 22].into_iter().cycle().take(16).collect::<Vec<u32>>(),
                [5,  9, 14, 20].into_iter().cycle().take(16).collect::<Vec<u32>>(),
                [4, 11, 16, 23].into_iter().cycle().take(16).collect::<Vec<u32>>(),
                [6, 10, 15, 21].into_iter().cycle().take(16).collect::<Vec<u32>>()
            ].concat()[0..64]
            .try_into()
            .unwrap(),
            initialization_state: [0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32],
        }
    }
}

impl MerkleDamgard for MessageDigestV5 {
    type InternalState = [u32; 4];
    type Chunk = u32;
    type HashOutput = [u32; 4];

    fn pad_input(message: &[u8]) -> Vec<Self::Chunk> {
        todo!()
    }

    fn apply_compressor(state: &mut Self::InternalState, chunk: Self::Chunk) {
        todo!()
    }

    fn finalize() -> Self::HashOutput {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn two() {
        assert!(2 == 2);
    }
}
