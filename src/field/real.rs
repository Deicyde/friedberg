use super::Field;

/// A type representing a real number.
pub type Real = f64;

impl Field for Real {
    fn one() -> Self {
        1.0
    }

    fn zero() -> Self {
        0.0
    }
}