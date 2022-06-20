use crate::field::Field;
use super::VectorSpace;
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub, Div, AddAssign, MulAssign, SubAssign, DivAssign, Neg};
use std::{fmt, mem};

/// A type representing a function from S to F. 
pub struct Funct<S: Copy + 'static, F: Field>(Box<dyn Fn(S) -> F>);

/// A type representing an infinite integer sequence.
type Sequence<F: Field> = Funct<usize, F>;

impl<S: Copy + 'static, F: Field> Funct<S, F> {
    fn new(f: impl Fn(S) -> F + 'static) -> Self {
        Self(Box::new(f))
    }

    fn apply(&self, s: S) -> F {
        (*self.0)(s)
    }
}

impl<S: Copy + 'static, F: Field> Neg for Funct<S, F> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(move |s: S| -self.apply(s))
    }
}
impl<S: Copy + 'static, F: Field> Add<&'static Self> for Funct<S, F> {
    type Output = Self;
    fn add(self, rhs: &'static Self) -> Self {
        let h = move |s: S| { self.apply(s) + rhs.apply(s) };
        Self(Box::new(h))
    }
}
impl<S: Copy + 'static, F: Field> Add<Self> for Funct<S, F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let h = move |s: S| { self.apply(s) + rhs.apply(s) };
        Self(Box::new(h))
    }
}
impl<S: Copy + 'static, F: Field> Sub<&'static Self> for Funct<S, F> {
    type Output = Self;
    fn sub(self, rhs: &'static Self) -> Self {
        let h = move |s: S| { self.apply(s) - rhs.apply(s) };
        Self(Box::new(h))
    }
}
impl<S: Copy + 'static, F: Field> Sub<Self> for Funct<S, F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let h = move |s: S| { self.apply(s) - rhs.apply(s) };
        Self(Box::new(h))
    }
}
impl<S: Copy + 'static, F: Field> AddAssign<&'static Self> for Funct<S, F> {
    fn add_assign(&mut self, rhs: &'static Self) {
        let mut f = Self::zero();
        mem::swap(&mut self.0, &mut f.0);
        let h = move |s: S| { f.apply(s) + rhs.apply(s) };
        self.0 = Box::new(h);
    }
}
impl<S: Copy + 'static, F: Field> AddAssign<Self> for Funct<S, F> {
    fn add_assign(&mut self, rhs: Self) {
        let mut f = Self::zero();
        mem::swap(&mut self.0, &mut f.0);
        let h = move |s: S| { f.apply(s) + rhs.apply(s) };
        self.0 = Box::new(h);
    }
}
impl<S: Copy + 'static, F: Field> SubAssign<&'static Self> for Funct<S, F> {
    fn sub_assign(&mut self, rhs: &'static Self) {
        let mut f = Self::zero();
        mem::swap(&mut self.0, &mut f.0);
        let h = move |s: S| { f.apply(s) - rhs.apply(s) };
        self.0 = Box::new(h);
    }
}
impl<S: Copy + 'static, F: Field> SubAssign<Self> for Funct<S, F> {
    fn sub_assign(&mut self, rhs: Self) {
        let mut f = Self::zero();
        mem::swap(&mut self.0, &mut f.0);
        let h = move |s: S| { f.apply(s) - rhs.apply(s) };
        self.0 = Box::new(h);
    }
}

// Operator implementations for functions with scalars.
impl<T: Into<F>, S: Copy + 'static, F: Field> Mul<T> for Funct<S,F> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let scale = rhs.into();
        let h = move |s: S| { self.apply(s) * scale };
        Self(Box::new(h))
    }
} 
impl<T: Into<F>, S: Copy + 'static, F: Field> Div<T> for Funct<S,F> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let scale = rhs.into();
        let h = move |s: S| { self.apply(s) / scale };
        Self(Box::new(h))
    }
} 
impl<T: Into<F>, S: Copy + 'static, F: Field> MulAssign<T> for Funct<S,F> {
    fn mul_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        let mut f = Self::zero();
        mem::swap(&mut self.0, &mut f.0);
        let h = move |s: S| { f.apply(s) * scale };
        self.0 = Box::new(h);
    }
} 
impl<T: Into<F>, S: Copy + 'static, F: Field> DivAssign<T> for Funct<S,F> {
    fn div_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        let mut f = Self::zero();
        mem::swap(&mut self.0, &mut f.0);
        let h = move |s: S| { f.apply(s) / scale };
        self.0 = Box::new(h);
    }
} 

impl<F: Field> fmt::Debug for Sequence<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DEBUG_PRINT_LENGTH: usize = 10;
        let mut output = f.debug_struct("Sequence");
        for i in 0..DEBUG_PRINT_LENGTH {
            output.field(&format!("[{}]", i), &self.apply(i));
        }
        output.finish()
    }
}
impl<F: Field> fmt::Display for Sequence<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const DISPLAY_PRINT_LENGTH: usize = 10;
        write!(f, "(")?;
        for i in 0..DISPLAY_PRINT_LENGTH {
            write!(f, "{},", self.apply(i))?;
        }
        write!(f, "…)")
    }
}

impl<S: Copy + 'static, F: Field> VectorSpace<F> for Funct<S, F> {
    fn zero() -> Self {
        Self(Box::new(|_: S| { F::zero() }))
    }
}

#[cfg(test)]
mod tests {
    use super::Funct;
    use crate::field::{Real, Int};

    #[test]
    fn funct_apply() {
        let f = Funct::new(|t: Int| (2*t + 1) as Real);
        let g = Funct::new(|t: Int| (1 + 4*t - 2*t*t) as Real);
        let h = Funct::new(|t: Int| (5i64.pow(t as u32) + 1) as Real);
        assert_eq!(f.apply(0), g.apply(0));
        assert_eq!(f.apply(1), g.apply(1));

        let i = f + g;
        assert_eq!(i.apply(0), h.apply(0));
        assert_eq!(i.apply(1), h.apply(1));
    }


}