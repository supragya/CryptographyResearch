//! This module contains the definition of struct `PrimeField`

use std::fmt::{Display, Formatter};

use crate::finitefield::FiniteField;

/// The `PrimeField` struct represents elements of a field of prime order.
/// This field is defined by a prime `P` and elements are integers mod `P`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default, PartialOrd)]
pub struct PrimeField<const P: usize> {
    pub(crate) value: usize,
}

// ------ USEFUL COMPILE TIME FUNCTIONS ------

/// Useful utility function to check results at compile time. Checks whether
/// a number given is prime. `n` should be known at compile time.
const fn check_prime(n: usize) {
    // Run-of-the-mill prime checking logic
    let mut test_num = 2;
    while test_num * test_num <= n {
        if n % test_num == 0 {
            panic!("not a prime");
        }
        test_num += 1;
    }
}

/// Checks prime and only returns value if check passes
const fn ensure_prime(n: usize) -> usize {
    check_prime(n);
    n
}

/// This function finds multiplicative generator given `P`. Weirdly, `P` is
/// needed as a const generic because we need to return `PrimeField<P>`.
///
/// A multiplicative generator should be able to generate all the elements
/// in the multiplicative subgroup in the field.
const fn find_multiplicative_generator<const P: usize>() -> PrimeField<P> {
    const fn gcd(a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    // This is wrong, through the run of the code below it would be
    // hold values which are non-prime. But that's okay... it will only
    // ever possibly pass for primes
    let mut num = 2;
    while num * num <= P {
        if gcd(num, P) == 1 {
            return PrimeField::<P>::new(num);
        }
        i += 1;
    }
    panic!("cannot find generator");
}

impl<const P: usize> Display for PrimeField<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const P: usize> FiniteField for PrimeField<P> {
    const MULTIPLICATIVE_GENERATOR: Self = if P == 2 {
        Self::ONE
    } else {
        find_multiplicative_generator::<P>()
    };
    const ONE: Self = Self { value: 1 };
    const ORDER: usize = ensure_prime(P);
    const ZERO: Self = Self { value: 0 };

    fn multiplicative_inverse(&self) -> Option<Self> {
        if self.value == 0 {
            return None;
        }

        // We know that within a prime field, due to fermat's little theorem
        // any element `e` will have
        //      e^(p-1) = 1 mod P
        // Hence,
        //      e^(p-2) = p^(-1) mod P

        Some(self.clone().pow(Self::ORDER - 2))
    }

    fn pow(self, pow: usize) -> Self {
        let mut pow = pow;
        let result = Self::ONE;

        while pow > 0 {
            if pow & 1 == 1 {
                result *= self;
            }
            self *= self;
            pow >>= 1;
        }

        result
    }
}

// Implement aspects of `PrimeField`
impl<const P: usize> PrimeField<P> {
    // TODO: Check whether this should a const fn
    pub const fn new(value: usize) -> Self {
        is_prime(P);
        Self { value: value % P }
    }

    /// Checks whether the field element is a quadratic residue in field mod P
    /// Returns `true` if it is a quadratic residue
    ///
    /// ## NOTES
    /// We make use of something called the "euler's criterion".
    /// By fermat's little theorem, (assume `is_congruent_to` is =)
    ///     x^(p-1) - 1 = 0 mod P
    ///
    /// All primes > 2 are odd, a.k.a P is odd, hence (p-1) is even.
    /// So, we can split as follows:
    ///     (x^(p-1)/2 - 1)(x^(p-1)/2 + 1) = 0 mod P
    /// or       L        *     R          = 0 mod P
    ///
    /// All quadratic residues are of the form (g^(2k)) where `g` is the
    /// multiplicative generator and k is some natural number. All non-residues
    /// on the other hand are of the form (g^(2k+1)).
    ///
    /// In case of QR, substitute x = g^2k
    ///     g^(2k)((p-1)/2) = 1 mod P
    ///     g^(p-1) = 1 mod P
    /// which is true by fermat's little theorem
    ///
    /// In the other case, the same doesn't hold.
    /// Hence, the case `L` should hold for all quadratic residues and is the
    /// test for quadratic residuosity.
    ///
    /// More info here: https://www.youtube.com/watch?v=2IBPOI43jek
    pub fn is_quadratic_residue(&self) -> bool {
        self.pow((P - 1) / 2).value == Self::ONE
    }
}
