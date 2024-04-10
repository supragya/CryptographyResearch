use ark_ff::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "99679"]
#[generator = "13"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

pub struct Prover;
pub struct Verifier;

fn generate_schnorr_id_actors(prover_only_x: u64) -> (Prover, Verifier) {
    (Prover{}, Verifier{})
}

#[cfg(test)]
mod tests {
    use crate::Fq;
    use ark_ff::{PrimeField, Field};

    #[test]
    fn schnorr_dlog_pok() {
        let x = 541;
        let private_x = Fq::from(x);
        let public_g_pow_x = Fq::from(13).pow(&[x]);

        let (prover, verifier) = generate_schnorr_id_actors(541);
    }
}
