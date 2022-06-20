#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IntMod2 {
    Zero,
    One
}

use super::{Real, Complex, Int, Rational};
use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, SubAssign, Neg};
use std::{fmt};
use super::Field;

impl Into<Int> for IntMod2 {
    fn into(self) -> Int {
        match self {
            IntMod2::Zero => 0,
            IntMod2::One => 1,
        }
    }
}

impl From<Int> for IntMod2 {
    fn from(i: Int) -> Self {
        if i % 2 == 0 {
            Self::Zero
        } else {
            Self::One
        }
    }
}

impl Into<Rational> for IntMod2 {
    fn into(self) -> Rational {
        Rational::from(match self {
            IntMod2::Zero => 0,
            IntMod2::One => 1,
        })
    }
}

impl Into<Real> for IntMod2 {
    fn into(self) -> Real {
        match self {
            IntMod2::Zero => 0.0,
            IntMod2::One => 1.0,
        }
    }
}

impl Into<Complex> for IntMod2 {
    fn into(self) -> Complex {
        match self {
            IntMod2::Zero => 0.0,
            IntMod2::One => 1.0,
        }.into()
    }
}

// Operator implementations for integers modulo 2.
impl Neg for IntMod2 {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}
impl Add for IntMod2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::One, Self::One) | (Self::Zero, Self::Zero) => Self::Zero,
            (Self::One, Self::Zero) | (Self::Zero, Self::One) => Self::One
        }
    }
}
impl Sub for IntMod2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::One, Self::One) | (Self::Zero, Self::Zero) => Self::Zero,
            (Self::One, Self::Zero) | (Self::Zero, Self::One) => Self::One
        }
    }
}
impl Mul for IntMod2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        if self == Self::Zero || rhs == Self::Zero {
            Self::Zero
        } else {
            Self::One
        }
    }
}
impl Div for IntMod2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs == Self::Zero {
            panic!("Divide by zero error");
        }
        self
    }
}
impl AddAssign for IntMod2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self) + rhs;
    }
}
impl SubAssign for IntMod2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self) - rhs;
    }
}
impl MulAssign for IntMod2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = (*self) * rhs;
    }
}
impl DivAssign for IntMod2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = (*self) / rhs;
    }
}

// Operator implementations for ints with integers modulo 2.
impl Add<Int> for IntMod2 {
    type Output = Self;
    fn add(self, rhs: Int) -> Self {
        self + Self::from(rhs)
    }
}
impl Mul<Int> for IntMod2 {
    type Output = Self;
    fn mul(self, rhs: Int) -> Self {
        self * IntMod2::from(rhs)
    }
}
impl Sub<Int> for IntMod2 {
    type Output = Self;
    fn sub(self, rhs: Int) -> Self {
        self - IntMod2::from(rhs)
    }
}
impl AddAssign<Int> for IntMod2 {
    fn add_assign(&mut self, rhs: Int) {
        *self = (*self) + rhs;
    }
}
impl SubAssign<Int> for IntMod2 {
    fn sub_assign(&mut self, rhs: Int) {
        *self = (*self) - rhs;
    }
}
impl MulAssign<Int> for IntMod2 {
    fn mul_assign(&mut self, rhs: Int) {
        *self = (*self) * rhs;
    }
}

impl fmt::Display for IntMod2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            IntMod2::Zero => 0,
            IntMod2::One => 1,
        })
    }
}

impl Field for IntMod2 {
    fn one() -> Self {
        Self::One
    }
    
    fn zero() -> Self {
        Self::Zero
    }
}