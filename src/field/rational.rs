/// A type representing the integers. Currently an alias for i64.
pub type Int = i64;

use super::{Real, Complex};
use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, SubAssign, Neg};
use std::{fmt, cmp};

use super::Field;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rational {
    pub num: Int, 
    pub denom: Int
}

impl Rational {
    fn new(num: Int, denom: Int) -> Self {
        Self {
            num,
            denom
        }
    }

    //Puts the fraction in simplest possible form. Is automatically run after every operation.
    fn simplify(&mut self) {
        use cmp::Ordering::*;
        let a = self.num;
        let b = self.denom;
        let divisor = match (a.cmp(&0), b.cmp(&0)) {
            (Greater, Greater) => gcd(a,b),
            (Less, Greater) => gcd(-a,b),
            (Greater, Less) => -gcd(a,-b),
            (Less, Less) => -gcd(-a,-b),
            _ => {
                self.num = 0;
                self.denom = 1;
                return;
            },
        };
        self.num /= divisor;
        self.denom /= divisor;
    }
}

/// Greatest common divisor function via the Euclidean algorithm. Used for simplifying fractions.
/// 
/// WARNING: Only works for positive a and b.
fn gcd(mut a: Int, mut b: Int) -> Int {
    let mut r;
    while a % b > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    b


}

impl Into<Real> for Rational {
    fn into(self) -> Real {
        (self.num as Real) / (self.denom as Real)
    }
}
impl Into<Complex> for Rational {
    fn into(self) -> Complex {
        <Self as Into<Real>>::into(self).into()
    }
}

// Operator implementations for rationals.
impl Neg for Rational {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.num = -self.num;
        self
    }
}
impl Add for Rational {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.num = (self.num * rhs.denom) + (self.denom * rhs.num);
        self.denom = self.denom * rhs.denom;
        self.simplify();
        self
    }
}
impl Sub for Rational {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self.num = (self.num * rhs.denom) - (self.denom * rhs.num);
        self.denom = self.denom * rhs.denom;
        self.simplify();
        self
    }
}
impl Mul for Rational {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self.num = self.num * rhs.num;
        self.denom *= self.denom * rhs.denom;
        self.simplify();
        self
    }
}
impl Div for Rational {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        self.num = self.num * rhs.denom;
        self.denom = self.denom * rhs.num;
        self.simplify();
        self
    }
}
impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self) + rhs;
    }
}
impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self) - rhs;
    }
}
impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        *self = (*self) * rhs;
    }
}
impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        *self = (*self) / rhs;
    }
}

// Operator implementations for rationals with ints.
impl From<Int> for Rational {
    fn from(x: Int) -> Self {
        Rational::new(x, 1)
    }
}
impl Add<Int> for Rational {
    type Output = Self;
    fn add(mut self, rhs: Int) -> Self {
        self.num = self.num + (rhs * self.denom);
        self
    }
}
impl Mul<Int> for Rational {
    type Output = Self;
    fn mul(mut self, rhs: Int) -> Self {
        self.num = self.num * rhs;
        self.simplify();
        self
    }
}
impl Div<Int> for Rational {
    type Output = Self;
    fn div(mut self, rhs: Int) -> Self {
        self.denom = self.denom * rhs;
        self.simplify();
        self
    }
}
impl Sub<Int> for Rational {
    type Output = Self;
    fn sub(mut self, rhs: Int) -> Self {
        self.num = self.num + (rhs * self.denom);
        self
    }
}
impl AddAssign<Int> for Rational {
    fn add_assign(&mut self, rhs: Int) {
        *self = (*self) + rhs;
    }
}
impl SubAssign<Int> for Rational {
    fn sub_assign(&mut self, rhs: Int) {
        *self = (*self) - rhs;
    }
}
impl MulAssign<Int> for Rational {
    fn mul_assign(&mut self, rhs: Int) {
        *self = (*self) * rhs;
    }
}
impl DivAssign<Int> for Rational {
    fn div_assign(&mut self, rhs: Int) {
        *self = (*self) / rhs;
    }
}
impl PartialEq<Int> for Rational {
    fn eq(&self, rhs: &Int) -> bool {
        (self.denom == 1) && (self.num == *rhs)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.denom)
        }
    }
}

impl Field for Rational {
    fn one() -> Self {
        Self::from(1)
    }
    
    fn zero() -> Self {
        Self::from(0)
    }
}