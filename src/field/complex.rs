use std::{ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg}, fmt};
use super::{Real, Field};

#[derive(Clone, Copy, PartialEq, Debug)]
/// A type representing a complex number.
pub struct Complex {
    /// The real component
    pub re: Real, 
    /// The imaginary component
    pub im: Real 
}

#[macro_export]
macro_rules! cmplx {
    ($a:expr) => {
        crate::field::Complex::from($a as f64)
    };
    ($a:expr, $b:expr) => {
        crate::field::Complex::new($a as f64, $b as f64)
    };
}

impl Complex {
    pub fn new(re: Real, im: Real) -> Self {
        Self {
            re,
            im
        }
    }

    pub fn real(&self) -> Real {
        self.re
    }

    pub fn imaginary(&self) -> Real {
        self.im
    }

    pub fn conjugate(mut self) -> Self {
        self.im = -self.im;
        self
    }

    pub fn mag_sq(&self) -> Real {
        self.re * self.re + self.im * self.im
    }
}

impl From<(Real, Real)> for Complex {
    fn from(x: (Real, Real)) -> Self {
        Complex::new(x.0, x.1)
    }
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.re = -self.re;
        self.im = -self.im;
        self
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.re += rhs.re;
        self.im += rhs.im;
        self
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self.re -= rhs.re;
        self.im -= rhs.im;
        self
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex::new(self.re * rhs.re - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re)
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.conjugate() / rhs.mag_sq()
    }
}
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self) + rhs;
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self) - rhs;
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = (*self) * rhs;
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = (*self) / rhs;
    }
}

impl From<Real> for Complex {
    fn from(re: Real) -> Self {
        Complex::new(re, 0.0)
    }
}
impl Add<Real> for Complex {
    type Output = Self;
    fn add(mut self, rhs: Real) -> Self {
        self.re += rhs;
        self
    }
}
impl Sub<Real> for Complex {
    type Output = Self;
    fn sub(mut self, rhs: Real) -> Self {
        self.re -= rhs;
        self
    }
}
impl Mul<Real> for Complex {
    type Output = Self;
    fn mul(mut self, rhs: Real) -> Self {
        self.re *= rhs;
        self.im *= rhs;
        self
    }
}
impl Div<Real> for Complex {
    type Output = Self;
    fn div(mut self, rhs: Real) -> Self {
        self.re /= rhs;
        self.im /= rhs;
        self
    }
}
impl AddAssign<Real> for Complex {
    fn add_assign(&mut self, rhs: Real) {
        *self = (*self) + rhs;
    }
}
impl SubAssign<Real> for Complex {
    fn sub_assign(&mut self, rhs: Real) {
        *self = (*self) - rhs;
    }
}
impl MulAssign<Real> for Complex {
    fn mul_assign(&mut self, rhs: Real) {
        *self = (*self) * rhs;
    }
}
impl DivAssign<Real> for Complex {
    fn div_assign(&mut self, rhs: Real) {
        *self = (*self) / rhs;
    }
}

impl Field for Complex {
    fn one() -> Self {
        Complex::from(1.0)
    }

    fn zero() -> Self {
        Complex::from(0.0)
    }
}