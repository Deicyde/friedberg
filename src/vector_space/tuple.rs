use crate::field::Field;
use super::VectorSpace;
use std::ops::{Add, Mul, Index, IndexMut};

pub struct Tuple<F: Field, const N: usize>([F; N]);

impl<F: Field, const N: usize> From<[F;N]> for Tuple<F,N> {
    fn from(arr: [F;N]) -> Self {
        Self(arr)
    }
}

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

impl<F: Field, const N: usize> Add<&Self> for Tuple<F, N> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        for i in 0..N {
            self[i] += rhs[i];
        }
        self
    }
}

impl<F: Field, const N: usize> Mul<F> for Tuple<F, N> {
    type Output = Self;
    fn mul(mut self, rhs: F) -> Self {
        for i in 0..N {
            self[i] *= rhs;
        }
        self
    }
}

impl<F: Field, const N: usize> VectorSpace<F> for Tuple<F, N> {
    fn zero() -> Self {
        Self([F::zero(); N])
    }
}