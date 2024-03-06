use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    ops::{Mul, Sub},
};

use num_traits::{One, Zero};

pub enum PolynomialRepr<T> {
    Points(Vec<(T, T)>),
    Roots(Vec<T>),
    /// Assumed index `i` to hold `C_i` when polynomial looks as follows:
    /// `C_0 + C_1 * x + C_2 * x^2 + ... + C_n * x^n`
    Coeff(Vec<T>),
}

/// A polynomial
pub struct Polynomial<T> {
    repr: PolynomialRepr<T>,
}

impl<T> Polynomial<T>
where
    T: Clone,
{
    /// Generate a polynomial from its coefficients `C_0`, `C_1`, ... `C_n`
    /// when polynomial looks as follows:
    /// `C_0 + C_1 * x + C_2 * x^2 + ... + C_n * x^n`
    pub fn new_from_coeffs(coeffs: &[T]) -> Self {
        Self {
            repr: PolynomialRepr::Coeff(coeffs.to_vec()),
        }
    }
}

impl<T> Polynomial<T> {
    /// Gets the degree of underlying polynomial
    pub fn degree(&self) -> usize {
        match &self.repr {
            PolynomialRepr::Points(_points) => unimplemented!(),
            PolynomialRepr::Roots(roots) => roots.len(),
            PolynomialRepr::Coeff(coeffs) => coeffs.len(),
        }
    }
}

impl<T> Polynomial<T>
where
    T: Clone + Hash + Eq,
{
    /// Generate a polynomial from its roots `r_0`, `r_1`, ... `r_n`
    /// when polynomial looks as follows:
    /// `(x - r_0)(x - r_1)(x - r_2)...(x - r_n)`
    pub fn new_from_roots(roots: &[T]) -> Self {
        let distinct_roots = Vec::from_iter(HashSet::<T>::from_iter(roots.to_owned()));
        Self {
            repr: PolynomialRepr::Roots(distinct_roots.to_vec()),
        }
    }
}

impl<T: Zero + One + Mul<Output = T> + Sub<Output = T> + Clone + Debug> Polynomial<T> {
    /// Evaluates the polynomial at a point.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomial::Polynomial;
    /// let polynomial = Polynomial::<u32>::new_from_coeffs(&[3, 2, 1]);
    /// assert_eq!(polynomial.eval(0), 3);
    /// assert_eq!(polynomial.eval(1), 6);
    /// assert_eq!(polynomial.eval(10), 123);
    /// ```
    #[inline]
    pub fn eval(&self, x: T) -> T {
        match &self.repr {
            PolynomialRepr::Points(_points) => unimplemented!(),
            PolynomialRepr::Roots(roots) => {
                let mut result: T = One::one();
                for n in roots.iter() {
                    result = (x.clone() - n.clone()) * result;
                }
                result
            }
            PolynomialRepr::Coeff(coeffs) => {
                let mut result: T = Zero::zero();
                for n in coeffs.iter().rev() {
                    result = n.clone() + result * x.clone();
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_for_coeff_polynomial_repr() {
        let polynomial = Polynomial::<u32>::new_from_coeffs(&[3, 2, 1]);
        assert_eq!(polynomial.eval(0), 3);
        assert_eq!(polynomial.eval(1), 6);
        assert_eq!(polynomial.eval(10), 123);
    }

    #[test]
    fn eval_for_roots_polynomial_repr() {
        let polynomial = Polynomial::<i32>::new_from_roots(&[1, 2, 3]);
        assert_eq!(polynomial.eval(1), 0);
        assert_eq!(polynomial.eval(2), 0);
        assert_eq!(polynomial.eval(3), 0);
    }

    #[test]
    fn polynomial_degree() {
        assert_eq!(Polynomial::<u32>::new_from_coeffs(&[3, 2, 1]).degree(), 3);
        assert_eq!(Polynomial::<u32>::new_from_roots(&[3, 2, 1]).degree(), 3);
        assert_eq!(Polynomial::<u32>::new_from_roots(&[3, 2, 3]).degree(), 2);
    }
}
