mod tuple;
mod matrix;

pub use matrix::Matrix;
pub use tuple::Tuple;
use crate::field::Field;
use std::ops::{Add, Mul, Sub, Div};

/// A trait for vector types satisfying the property of being a vector space over a scalar field.
/// Vector addition must be commutitive, associative, and have identity 0.
/// Scalar multiplication must be associative and have identity Scalar::one().
/// Addition and multiplication must obey distributive laws.
trait VectorSpace<Scalar: Field>: Sized 
    + 'static
    + Add<&'static Self, Output=Self>
    + Sub<&'static Self, Output=Self>
    + Mul<Scalar, Output=Self>
    + Div<Scalar, Output=Self>
    + PartialEq {
    // Returns the zero element of the vector space, which must be the identity for vector addition.
    fn zero() -> Self;
}


