use std::ops::{Add, Sub, Mul, Div};
use super::Real;

#[derive(Clone, Copy)]
struct Complex {
    pub re: Real, // The real component
    pub im: Real // The imaginary component
}

impl Complex {
    fn new(re: Real, im: Real) -> Self {
        Self {
            re,
            im
        }
    }

    fn real(&self) -> Real {
        self.re
    }

    fn imaginary(&self) -> Real {
        self.im
    }

    fn conjugate(mut self) -> Self {
        self.im = -self.im;
        self
    }

    fn mag_sq(&self) -> Real {
        self.re * self.re + self.im * self.im
    }
}

impl From<(Real, Real)> for Complex {
    fn from(x: (Real, Real)) -> Self {
        Complex::new(x.0, x.1)
    }
}

impl From<Real> for Complex {
    fn from(re: Real) -> Self {
        Complex::new(re, 0.0)
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

impl Add<Real> for Complex {
    type Output = Self;
    fn add(mut self, rhs: Real) -> Self {
        self.re += rhs;
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

impl Sub<Real> for Complex {
    type Output = Self;
    fn sub(mut self, rhs: Real) -> Self {
        self.re -= rhs;
        self
    }
}

// (a + bi)(c + di) = (ac - bd) + i(ad + bc)
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex::new(self.re * rhs.re - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re)
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

// (a+bi)/(c+di) = (a+bi)(c-di)/(c^2 + d^2)
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.conjugate() / rhs.mag_sq()
    }
}