use crate::field::Field;
use super::VectorSpace;
use std::ops::{Add, Mul, Sub, Div, Index, IndexMut};

/// A type representing a fixed-size matrix over a given field.
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<F: Field, const M: usize, const N: usize>([[F;N];M]);

impl<F: Field, const M: usize, const N: usize> From<[[F;N];M]> for Matrix<F, M, N> {
    fn from(arr: [[F;N];M]) -> Self {
        Self(arr)
    }
}

impl<F: Field, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<F, M, N> {
    type Output = F;
    fn index(&self, idx: (usize, usize)) -> &F {
        &self.0[idx.0][idx.1]
    }
}

impl<F: Field, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<F, M, N> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut F {
        &mut self.0[idx.0][idx.1]
    }
}

impl<F: Field, const M: usize, const N: usize> Add<&Self> for Matrix<F, M, N> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] += rhs[(i,j)]
            }
        }
        self
    }
}

impl<F: Field, const M: usize, const N: usize> Sub<&Self> for Matrix<F, M, N> {
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] -= rhs[(i,j)]
            }
        }
        self
    }
}

impl<F: Field, const M: usize, const N: usize> Mul<F> for Matrix<F, M, N> {
    type Output = Self;
    fn mul(mut self, rhs: F) -> Self {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] *= rhs;
            }
        }
        self
    }
}

impl<F: Field, const M: usize, const N: usize> Div<F> for Matrix<F, M, N> {
    type Output = Self;
    fn div(mut self, rhs: F) -> Self {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] /= rhs;
            }
        }
        self
    }
}

impl<F: Field, const M: usize, const N: usize, const P: usize> Mul<&Matrix<F, P, N>> for &Matrix<F, M, P> {
    type Output = Matrix<F, M, N>;
    fn mul(self, rhs: &Matrix<F, P, N>) -> Matrix<F, M, N> {
        let mut out = Matrix::<F, M, N>::zero();
        for i in 0..M {
            for j in 0..N {
                for k in 0..P {
                    out[(i,j)] += self[(i,k)] * rhs[(k, j)];
                }
            }
        }
        out
    }
}

impl<F: Field, const M: usize, const N: usize> VectorSpace<F> for Matrix<F, M, N> {
    fn zero() -> Self {
        Self([[F::zero(); N]; M])
    }
}