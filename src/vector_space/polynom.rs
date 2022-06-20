use crate::field::{Field, Real, Complex, Int, Rational};
use super::VectorSpace;
use std::ops::{Add, Mul, Sub, Div, AddAssign, MulAssign, SubAssign, DivAssign, Index, IndexMut, Neg};
use std::{fmt, mem};

#[derive(PartialEq, Debug, Clone)]
/// A type representing a polynomial over a given field. The final entry in the vector must always be nonzero.
pub struct Polynom<F: Field>(Vec<F>);

#[macro_export]
macro_rules! polynom {
    () => (
        crate::vector_space::Polynom::from(vec![])
    );
    ($($x:expr),+ $(,)?) => (
        crate::vector_space::Polynom::from([$($x),+])
    );
}

impl<F: Field> Polynom<F> {
    /// Returns the length of the vector stored in the polynomial. 
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Appends zeros to the end of the vector so that it can be indexed up to n.
    /// 
    /// WARNING: Leaves the vector in an invalid state, since the final entry in the vector is no longer nonzero.
    fn alloc_space(&mut self, n: usize) {
        if n > self.len() {
            self.0.append(&mut vec![F::zero(); n - self.len()]);
        }
    }

    /// Removes all zeros from the end of the polynomial. 
    /// Must be called after operations which have the potential to leave the polynomial in an invalid state.
    fn shrink(&mut self) {
        while self.0.last() == Some(&F::zero()) {
            self.0.pop();
        }
    }

    /// Evaluates the polynomial at the given x value.
    fn eval_at(&self, x: F) -> F {
        let mut exponential = F::one();
        let mut accum = F::zero();
        for i in 0..self.len() {
            accum += self[i] * exponential;
            exponential *= x;
        }
        accum
    }
}

// From vec and array
impl<F: Field> From<Vec<F>> for Polynom<F> {
    fn from(v: Vec<F>) -> Self {
        let mut ret = Self(v);
        ret.shrink();
        ret
    }
}
impl<F: Field, const N: usize> From<[F;N]> for Polynom<F> {
    fn from(arr: [F;N]) -> Self {
        let mut ret = Self(Vec::<F>::from(arr));
        ret.shrink();
        ret
    }
}
impl<const N: usize> From<[Int;N]> for Polynom<Rational> {
    fn from(arr: [Int;N]) -> Self {
        let mut new_arr = [Rational::zero(); N];
        for i in 0..N {
            new_arr[i] = arr[i].into();
        }
        Self::from(new_arr)
    }
}

// Polynomial indexing
impl<F: Field> Index<usize> for Polynom<F> {
    type Output = F;
    fn index(&self, idx: usize) -> &F {
        &self.0[idx]
    }
}
impl<F: Field> IndexMut<usize> for Polynom<F> {
    fn index_mut(&mut self, idx: usize) -> &mut F {
        &mut self.0[idx]
    }
}

// Operator implementations for polynomials with polynomials
impl<F: Field> Neg for Polynom<F> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for i in 0..self.len() {
            self[i] = -self[i];
        }
        self
    }
}
impl<F: Field> Add<&Self> for Polynom<F> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}
impl<F: Field> Add<Self> for Polynom<F> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}
impl<F: Field> Sub<&Self> for Polynom<F> {
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self {
        self -= rhs;
        self
    }
}
impl<F: Field> Sub<Self> for Polynom<F> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}
impl<F: Field> AddAssign<&Self> for Polynom<F> {
    fn add_assign(&mut self, rhs: &Self) {
        self.alloc_space(rhs.len());
        for i in 0..rhs.len() {
            self[i] += rhs[i];
        }
        self.shrink();
    }
}
impl<F: Field> AddAssign<Self> for Polynom<F> {
    fn add_assign(&mut self, mut rhs: Self) {
        if rhs.len() > self.len() {
            mem::swap(self, &mut rhs);
        }
        for i in 0..rhs.len() {
            self[i] += rhs[i];
        }
        self.shrink()
    }
}
impl<F: Field> SubAssign<&Self> for Polynom<F> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.alloc_space(rhs.len());
        for i in 0..rhs.len() {
            self[i] -= rhs[i];
        }
        self.shrink()
    }
}
impl<F: Field> SubAssign<Self> for Polynom<F> {
    fn sub_assign(&mut self, mut rhs: Self) {
        if rhs.len() > self.len() {
            mem::swap(self, &mut rhs);
        }
        for i in 0..rhs.len() {
            self[i] -= rhs[i];
        }
        self.shrink()
    }
}

// Operator implementations for polynomials with scalars
impl<T: Into<F>, F: Field> Mul<T> for Polynom<F> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        self *= rhs;
        self
    }
}
impl<T: Into<F>, F: Field> Div<T> for Polynom<F> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        self /= rhs;
        self
    }
}
impl<T: Into<F>, F: Field> MulAssign<T> for Polynom<F> {
    fn mul_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        for i in 0..self.len() {
            self[i] *= scale;
        }
        self.shrink();
    }
}
impl<T: Into<F>, F: Field> DivAssign<T> for Polynom<F> {
    fn div_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        for i in 0..self.len() {
            self[i] /= scale;
        }
        self.shrink();
    }
}

impl<F: Field> fmt::Display for Polynom<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.len() == 0 {
            return write!(f, "0");
        }
        for i in (1..self.len()).rev() {
            write!(f, "{}x^{}+", self[i], i)?;
        }
        write!(f, "{}", self[0])
    }
}

impl<F: Field> VectorSpace<F> for Polynom<F> {
    fn zero() -> Self {
        Self(vec![])
    }
}

impl VectorSpace<Real> for Polynom<Complex> {
    fn zero() -> Self {
        <Polynom<Complex> as VectorSpace<Complex>>::zero()
    }
}
impl VectorSpace<Rational> for Polynom<Real> {
    fn zero() -> Self {
        <Polynom<Real> as VectorSpace<Real>>::zero()
    }
}
impl VectorSpace<Rational> for Polynom<Complex> {
    fn zero() -> Self {
        <Polynom<Complex> as VectorSpace<Complex>>::zero()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn polynomial_scaling() {
        assert_eq!(polynom![0, -3, 8, 0, -6, 0, 0, 2] * 5, polynom![0, -15, 40, 0, -30, 0, 0, 10]);
        assert_eq!(polynom![2, 4, 0, -2, 0, 1] * 3, polynom![6, 12, 0, -6, 0, 3]);
    }

    #[test]
    fn polynomial_addition() {
        assert_eq!(polynom![3, 4, 0, -7, 2] + polynom![7, -6, 2, 8], polynom![10, -2, 2, 1, 2]);
        assert_eq!(polynom![-6, 8, 7, -3] + polynom![10, -8, 0, 2], polynom![4, 0, 7, -1]);
    }
}