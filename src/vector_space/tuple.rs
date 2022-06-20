use crate::field::{Field, Real, Complex};
use crate::field::rational::{Int, Rational};
use super::VectorSpace;
use std::ops::{Add, Mul, Sub, Div, AddAssign, MulAssign, SubAssign, DivAssign, Index, IndexMut, Neg};
use std::fmt;

/// A type representing a fixed-size column vector of elements from a field.
#[derive(PartialEq, Debug, Clone)]
pub struct Tuple<F: Field, const N: usize>([F; N]);

#[macro_export]
macro_rules! tuple {
    ($($x:expr),+ $(,)?) => (
        crate::vector_space::Tuple::from([$($x),+])
    );
}

impl<F: Field, const N: usize> Tuple<F, N> {
    fn is_parallel(&self, other: &Self) -> bool {
        if N == 0 {
            return true;
        }
        let scale = self[0] / other[0];
        dbg!(scale, self, other);
        for i in 1..N {
            if other[i] * scale != self[i] {
                dbg!(other[i] * scale, self[i]);
                return false;
            }
        }
        true
    }

    pub fn into<G: Field + From<F>>(self) -> Tuple<G, N> {
        let mut new_arr = [G::zero();N];
        for i in 0..N {
            new_arr[i] = G::from(self[i]);
        }
        Tuple::<G,N>(new_arr)
    }
}

// From array
impl<F: Field, const N: usize> From<[F;N]> for Tuple<F,N> {
    fn from(arr: [F;N]) -> Self {
        Self(arr)
    }
}
impl<F: Field, const N: usize> Into<[F;N]> for Tuple<F,N> {
    fn into(self) -> [F;N] {
        self.0
    }
}
impl<const N: usize> From<[Int;N]> for Tuple<Rational,N> {
    fn from(arr: [Int;N]) -> Self {
        let mut new_arr = [Rational::zero(); N];
        for i in 0..N {
            new_arr[i] = arr[i].into();
        }
        Self(new_arr)
    }
}

// Tuple indexing
impl<F: Field, const N: usize> Index<usize> for Tuple<F,N> {
    type Output = F;
    fn index(&self, idx: usize) -> &F {
        &self.0[idx]
    }
}
impl<F: Field, const N: usize> IndexMut<usize> for Tuple<F,N> {
    fn index_mut(&mut self, idx: usize) -> &mut F {
        &mut self.0[idx]
    }
}

// Operator implementations for tuples with tuples.
impl<F: Field, const N: usize> Neg for Tuple<F, N> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for i in 0..N {
            self[i] = -self[i];
        }
        return self;
    }
}
impl<F: Field, const N: usize> Add<&Self> for Tuple<F, N> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}
impl<F: Field, const N: usize> Add<Self> for Tuple<F, N> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}
impl<F: Field, const N: usize> Sub<&Self> for Tuple<F, N> {
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self {
        self -= rhs;
        self
    }
}
impl<F: Field, const N: usize> Sub<Self> for Tuple<F, N> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}
impl<F: Field, const N: usize> AddAssign<&Self> for Tuple<F, N> {
    fn add_assign(&mut self, rhs: &Self) {
        for i in 0..N {
            self[i] += rhs[i];
        }
    }
}
impl<F: Field, const N: usize> AddAssign<Self> for Tuple<F, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] += rhs[i];
        }
    }
}
impl<F: Field, const N: usize> SubAssign<&Self> for Tuple<F, N> {
    fn sub_assign(&mut self, rhs: &Self) {
        for i in 0..N {
            self[i] -= rhs[i];
        }
    }
}
impl<F: Field, const N: usize> SubAssign<Self> for Tuple<F, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] -= rhs[i];
        }
    }
}

// Operator implementations for tuples with scalars.
impl<T: Into<F>, F: Field, const N: usize> Mul<T> for Tuple<F, N> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        let scale = rhs.into();
        for i in 0..N {
            self[i] *= scale;
        }
        self
    }
}
impl<T: Into<F>, F: Field, const N: usize> Div<T> for Tuple<F, N> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        let scale = rhs.into();
        for i in 0..N {
            self[i] /= scale;
        }
        self
    }
}
impl<T: Into<F>, F: Field, const N: usize> MulAssign<T> for Tuple<F, N> {
    fn mul_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        for i in 0..N {
            self[i] *= scale;
        }
    }
}
impl<T: Into<F>, F: Field, const N: usize> DivAssign<T> for Tuple<F, N> {
    fn div_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        for i in 0..N {
            self[i] /= scale;
        }
    }
}

impl<F: Field, const N: usize> VectorSpace<F> for Tuple<F, N> {
    fn zero() -> Self {
        Self([F::zero(); N])
    }
}

// Tuples of complex numbers are a vector space over the real numbers. (1.2 Ex 14)
impl<const N: usize> VectorSpace<Real> for Tuple<Complex, N> {
    fn zero() -> Self {
        <Tuple<Complex, N> as VectorSpace<Complex>>::zero()
    }
}
impl<const N: usize> VectorSpace<Rational> for Tuple<Real, N> {
    fn zero() -> Self {
        <Tuple<Real, N> as VectorSpace<Real>>::zero()
    }
}
impl<const N: usize> VectorSpace<Rational> for Tuple<Complex, N> {
    fn zero() -> Self {
        <Tuple<Complex, N> as VectorSpace<Complex>>::zero()
    }
}

impl<F: Field, const N: usize> fmt::Display for Tuple<F, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if N == 0 {
            return write!(f, "(empty tuple)");
        }
        write!(f, "(")?;
        for i in 0..N-1 {
            write!(f, "{}, ", self[i])?;
        }
        write!(f, "{})", self[N-1])
    }
}

#[cfg(test)]
mod tests {
    use crate::cmplx;
    use super::Tuple;

    #[test]
    fn tuple_scaling() {
        assert_eq!(tuple![1,-2,0] * -5, tuple![-5,10,0]);
        assert_eq!(tuple![cmplx!(1,1), cmplx!(2)] * cmplx!(0,1), tuple![cmplx!(-1,1), cmplx!(0,2)]);
    }

    #[test]
    fn tuple_addition() {
        assert_eq!(tuple![3, -2, 0] + tuple![-1, 1, 4], tuple![2, -1, 4]);
        assert_eq!(tuple![cmplx!(1, 1), cmplx!(2)] + tuple![cmplx!(2,-3), cmplx!(0,4)], tuple![cmplx!(3,-2), cmplx!(2,4)]);
    }

    #[test]
    fn tuple_subtraction() {
        assert_eq!(tuple![4, 5, 3] - tuple![-2, 0, 1], tuple![6, 5, 2]);
        assert_eq!(tuple![-3, -2, 4] - tuple![1, 0, 2], tuple![-4, -2, 2])
    }

    #[test]
    fn tuple_is_parallel() {
        assert!(!tuple![3, 1, 2].is_parallel(&tuple![6, 4, 2]));
        assert!(tuple![-3, 1, 7].is_parallel(&tuple![9, -3, -21]));
        assert!(tuple![5, -6, 7].is_parallel(&tuple![-5, 6, -7]));
        assert!(!tuple![2, 0, -5].is_parallel(&tuple![5, 0, -2]));
    }
}