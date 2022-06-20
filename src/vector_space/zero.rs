use super::VectorSpace;
use std::ops::{Add, Mul, Sub, Div, AddAssign, MulAssign, SubAssign, DivAssign, Neg};
use crate::field::Field;

/// An empty type representing the zero vector space, also known as the trivial vector space.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Zero();

impl Neg for Zero {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}
impl AddAssign<Self> for Zero {
    fn add_assign(&mut self, _: Self) {}
}
impl AddAssign<&Self> for Zero {
    fn add_assign(&mut self, _: &Self) {}
}
impl SubAssign<Self> for Zero {
    fn sub_assign(&mut self, _: Self) {}
}
impl SubAssign<&Self> for Zero {
    fn sub_assign(&mut self, _: &Self) {}
}
impl Add<Self> for Zero {
    type Output = Self;
    fn add(self, _: Self) -> Self {
        self
    }
}
impl Add<&Self> for Zero {
    type Output = Self;
    fn add(self, _: &Self) -> Self {
        self
    }
}
impl Sub<Self> for Zero {
    type Output = Self;
    fn sub(self, _: Self) -> Self {
        self
    }
}
impl Sub<&Self> for Zero {
    type Output = Self;
    fn sub(self, _: &Self) -> Self {
        self
    }
}

impl<F: Field> Mul<F> for Zero {
    type Output = Self;
    fn mul(self, _: F) -> Self {
        self
    }
}
impl<F: Field> Div<F> for Zero {
    type Output = Self;
    fn div(self, _: F) -> Self {
        self
    }
}
impl<F: Field> MulAssign<F> for Zero {
    fn mul_assign(&mut self, _: F) {}
}
impl<F: Field> DivAssign<F> for Zero {
    fn div_assign(&mut self, _: F) {}
}

impl<F: Field> VectorSpace<F> for Zero {
    fn zero() -> Self {
        Zero()
    }
}

#[cfg(test)]
mod tests {
    use super::Zero;
    #[test]
    fn zero_space() {
        assert_eq!(Zero() + Zero(), Zero());
        assert_eq!(Zero() * 2.0, Zero());
    }
}