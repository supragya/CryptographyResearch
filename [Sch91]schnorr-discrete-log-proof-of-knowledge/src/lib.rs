// Much reference borrowed from
// https://github.com/arkworks-rs/r1cs-tutorial/blob/main/simple-payments/src/signature/schnorr/mod.rs#L53

use std::marker::PhantomData;

use ark_crypto_primitives::Error;
use ark_ec::{AffineRepr, CurveGroup, Group};
use ark_ff::PrimeField;

pub struct Schnorr<C: CurveGroup> {
    _group: PhantomData<C>,
}

pub type PublicKey<C> = <C as CurveGroup>::Affine;

#[derive(Clone, Default, Debug)]
pub struct SecretKey<C: CurveGroup + Group> {
    pub secret_key: C::ScalarField,
    pub public_key: PublicKey<C>,
}

pub struct Parameters<C: CurveGroup> {
    pub generator: C::Affine,
    pub salt: Option<[u8; 32]>,
}

impl<C: CurveGroup + Group> Schnorr<C>
where
    C::ScalarField: PrimeField,
{
    fn setup() -> Result<Parameters<C>, Error> {
        Ok(Parameters::<C> {
            generator: C::generator().into(),
            salt: Default::default(),
        })
    }

    fn from_secret(parameters: &Parameters<C>, secret: u64) -> Result<SecretKey<C>, Error> {
        let secret_key = C::ScalarField::from_bigint(secret.into()).unwrap();
        let public_key = parameters.generator.mul_bigint(&[secret]).into();
        Ok(SecretKey::<C> {
            secret_key,
            public_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::G1Projective;
    use ark_ec::{AffineRepr, Group};
    use ark_ff::PrimeField;

    use super::Schnorr;
    use crate::Parameters;

    #[test]
    fn schnorr_dlog_pok() {
        let secret_x = 541;
        let setup_params: Parameters<G1Projective> = Schnorr::setup().unwrap();
        let secret = Schnorr::from_secret(&setup_params, secret_x).unwrap();

        // initiate protocol
        let prover_random_r = <G1Projective as Group>::ScalarField::from(412);
        let verifier_random_c = <G1Projective as Group>::ScalarField::from(31981);
        let prover_computed_z = prover_random_r
            + <G1Projective as Group>::ScalarField::from(secret_x) * verifier_random_c;

        let prover_generated_u = setup_params
            .generator
            .mul_bigint(prover_random_r.into_bigint());

        let verifier_lhs = setup_params
            .generator
            .mul_bigint(prover_computed_z.into_bigint());
        let verifier_rhs = prover_generated_u
            + secret
                .public_key
                .mul_bigint(verifier_random_c.into_bigint());

        assert_eq!(verifier_lhs, verifier_rhs);
    }
}
