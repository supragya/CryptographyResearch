use ark_ff::fields::{Fp64, MontBackend, MontConfig};

/// 41 = (2^3 x 5) + 1
/// Hence, a 8th root of unity would exist in this, since 2^3 is a divisor
/// 3 is such root of unity. 3^8 = 6561. And 6561 % 41 = 1
/// Building a field is given at: https://docs.rs/ark-ff/latest/ark_ff/fields/trait.Field.html
#[derive(MontConfig)]
#[modulus = "41"]
#[generator = "2"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

pub enum PolynomialRepr<T> {
    Points(Vec<(T, T)>),
    Roots(Vec<T>),
    Coeff(Vec<T>),
}

/// A polynomial
pub struct Polynomial<T> {
    repr: PolynomialRepr<T>,
}

#[cfg(test)]
mod tests {
    use ark_ff::{BigInt, Field, PrimeField};

    use super::*;

    #[test]
    fn ensure_facts() {
        assert_eq!(<Fq as PrimeField>::MODULUS, BigInt::<1>([41]));
        assert_eq!(Fq::from(3).pow([8]), Fq::ONE);
    }
}
