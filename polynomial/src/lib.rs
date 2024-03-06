use std::ops::Mul;

use num_traits::Zero;

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

impl<T: Zero + Mul<Output = T> + Clone> Polynomial<T> {
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
            PolynomialRepr::Roots(_roots) => unimplemented!(),
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
}
