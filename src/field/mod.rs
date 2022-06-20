pub mod real;
pub use real::Real;
pub mod complex;
pub use complex::Complex;
pub mod rational;
pub use rational::{Int, Rational};
mod int_mod_2;

use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, SubAssign, Neg};
use std::fmt::{Debug, Display};

/// A trait for numeric types that satisfy the property of being a field.
/// Addition must be commutative, associative, and have identity 0.
/// Multiplication must be commutative, associative, and have identity 1.
/// Additive and multiplicative inverses must be defined for all non-zero elements.
/// Addition and multiplication must obey the distributive law.
pub trait Field: Sized
    + Neg<Output=Self>
    + Add<Output=Self>
    + Sub<Output=Self> 
    + Mul<Output=Self> 
    + Div<Output=Self>
    + AddAssign
    + MulAssign
    + DivAssign
    + SubAssign
    + PartialEq
    + Copy
    + Debug
    + Display
    +'static {
    /// Returns the multiplicative identity
    fn one() -> Self;
    /// Returns the additive identity
    fn zero() -> Self;

    /// Returns the multiplicative inverse of x
    fn mul_inverse(x: Self) -> Self {
        Self::one() / x
    }

    /// Returns the additive inverse of x
    fn add_inverse(x: Self) -> Self {
        Self::zero() - x
    }
}