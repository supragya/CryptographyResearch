use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    ops::{Mul, Sub},
};
extern crate nalgebra as na;
use na::{ComplexField, DMatrix, RowDVector, Scalar};
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

impl<T> Polynomial<T>
where
    T: One + Clone,
{
    /// Generate powers of `base` raised to a max degree of `deg-1`
    fn generate_powers(base: T, deg: usize) -> Vec<T> {
        let mut powers_vec = Vec::<T>::with_capacity(deg);
        powers_vec.push(T::one());
        for idx in 1..deg {
            powers_vec.push(base.clone() * powers_vec[idx - 1].clone());
        }
        powers_vec
    }
}

impl<T> Polynomial<T>
where
    T: Clone + Scalar + Debug + Mul<Output = T> + One + ComplexField,
{
    /// Generate a polynomials from its evalutation points givent in
    /// a tuple format `(a, b)` such that `poly(a) = b`. Given `n`
    /// points of evaluation, `n-1` degree polynomial is generated
    pub fn new_from_evals(evals: &[(T, T)]) -> Self {
        // We know that if all evalutaions matrix `E` is multiplied by
        // coefficient vector `C`, resultant would be `R`. `evals`
        // essentially is `[E(x) | R]`
        let x_powers_matrix = DMatrix::<T>::from_rows(
            &evals
                .iter()
                .map(|(eval_point, _eval)| {
                    RowDVector::from_iterator(
                        evals.len(),
                        Self::generate_powers(eval_point.clone(), evals.len()),
                    )
                })
                .collect::<Vec<_>>()[..],
        );

        let x_inverse_matrix = x_powers_matrix.qr().try_inverse().unwrap();

        let eval_matrix = DMatrix::<T>::from_rows(
            &evals
                .iter()
                .map(|(_eval_point, eval)| RowDVector::from_iterator(1, [eval.clone()]))
                .collect::<Vec<_>>()[..],
        );

        let coeff_matrix = x_inverse_matrix * eval_matrix;

        Self {
            repr: PolynomialRepr::Coeff(Vec::<T>::from_iter(coeff_matrix.iter().cloned())),
        }
    }
}

impl<T> Polynomial<T>
where
    T: Zero + One + Mul<Output = T> + Sub<Output = T> + Clone + Debug,
{
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

    #[test]
    fn test_generate_power() {
        assert_eq!(Polynomial::generate_powers(2, 5), vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn polynomial_from_evals() {
        // polynomial -> 1 + 4*x + x^2
        let evals = [(1.0, 6.0), (2.0, 13.0), (3.0, 22.0)];
        let polynomial = Polynomial::new_from_evals(&evals);
        assert_eq!(polynomial.eval(10.0) as i64, 141);
    }
}
