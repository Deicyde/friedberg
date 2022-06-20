mod tuple;
mod matrix;
mod polynom;
mod funct;
mod zero;

pub use matrix::Matrix;
pub use tuple::Tuple;
pub use polynom::Polynom;
pub use funct::Funct; 
use crate::field::Field;
use std::ops::{Add, Mul, Sub, Div, AddAssign, MulAssign, SubAssign, DivAssign, Neg};

/// A trait for vector types satisfying the property of being a vector space over a scalar field.
/// Vector addition must be commutitive, associative, and have identity 0.
/// Scalar multiplication must be associative and have identity Scalar::one().
/// Addition and multiplication must obey distributive laws.
/// All vectors must have an additive inverse.
trait VectorSpace<Scalar: Field>: Sized 
    + 'static
    + Neg<Output=Self>
    + Add<&'static Self, Output=Self>
    + Sub<&'static Self, Output=Self>
    + Add<Self, Output=Self>
    + Sub<Self, Output=Self>
    + Mul<Scalar, Output=Self>
    + Div<Scalar, Output=Self>
    + AddAssign<&'static Self>
    + AddAssign<Self>
    + SubAssign<&'static Self>
    + SubAssign<Self>
    + MulAssign<Scalar>
    + DivAssign<Scalar> {
    // Returns the zero element of the vector space, which must be the identity for vector addition.
    fn zero() -> Self;
}