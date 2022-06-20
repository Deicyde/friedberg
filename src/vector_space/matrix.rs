use crate::field::{Field, Int, Rational, Real, Complex};
use super::VectorSpace;
use std::ops::{Add, Mul, Sub, Div, AddAssign, MulAssign, SubAssign, DivAssign, Index, IndexMut, Neg};

/// A type representing a fixed-size matrix over a given field.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<F: Field, const M: usize, const N: usize>([[F;N];M]);

#[macro_export]
macro_rules! matrix {
    ($($x:expr),+ $(,)?) => (
        crate::vector_space::Matrix::from([$($x),+])
    );
}

impl<F: Field, const M: usize, const N: usize> Matrix<F, M, N> {
    /// Retrieves the `col`'th column. Assumes the input is less than M.
    fn get_column(&self, col: usize) -> [F;M] {
        let mut output = [F::zero(); M];
        for row in 0..M {
            output[row] = self[(row,col)];
        }
        output
    }

    /// Retrieves the `row`'th row. Assumes the input is less than N.
    fn get_row(&self, row: usize) -> [F;N] {
        self[row].clone()
    }

    /// Multiplies this matrix in-place by the inputted matrix on the right.
    fn right_multiply(&mut self, rhs: &Matrix<F, N, N>) {
        for i in 0..M {
            let row = self.get_row(i);
            for j in 0..N {
                self[(i, j)] = F::zero();
                for k in 0..N {
                    self[(i, j)] += row[k] * rhs[(k,j)];
                }
            }
        }
    }

    /// Multiplies this matrix in-place by the inputted matrix on the left.
    fn left_multiply(&mut self, rhs: &Matrix<F, M, M>) {
        for i in 0..N {
            let column: [F;M] = self.get_column(i);
            for j in 0..M {
                self[(i, j)] = F::zero();
                for k in 0..M {
                    self[(i, j)] += column[k] * rhs[(k,j)];
                }
            }
        }
    }
}

// From double array
impl<F: Field, const M: usize, const N: usize> From<[[F;N];M]> for Matrix<F, M, N> {
    fn from(arr: [[F;N];M]) -> Self {
        Self(arr)
    }
}
// Special case from an array of ints — makes it into a matrix of rationals.
impl<const M: usize, const N: usize> From<[[Int;N];M]> for Matrix<Rational,M,N> {
    fn from(arr: [[Int;N];M]) -> Self {
        let mut new_arr = [[Rational::zero(); N];M];
        for i in 0..M {
            for j in 0..N {
                new_arr[i][j] = arr[i][j].into();
            }
        }
        Self(new_arr)
    }
}

// Matrix indexing
impl<F: Field, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<F, M, N> {
    type Output = F;
    fn index(&self, idx: (usize, usize)) -> &F {
        &self.0[idx.0][idx.1]
    }
}
impl<F: Field, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<F, M, N> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut F {
        &mut self.0[idx.0][idx.1]
    }
}
impl<F: Field, const M: usize, const N: usize> Index<usize> for Matrix<F, M, N> {
    type Output = [F;N];
    fn index(&self, idx: usize) -> &[F;N] {
        &self.0[idx]
    }
}
impl<F: Field, const M: usize, const N: usize> IndexMut<usize> for Matrix<F, M, N> {
    fn index_mut(&mut self, idx: usize) -> &mut [F;N] {
        &mut self.0[idx]
    }
}

// Matrix addition and subtraction
impl<F: Field, const M: usize, const N: usize> Neg for Matrix<F, M, N> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] = -self[(i,j)];
            }
        }
        self
    }
}
impl<F: Field, const M: usize, const N: usize> Add<&Self> for Matrix<F, M, N> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self {
        self += rhs;
        self
    }
}
impl<F: Field, const M: usize, const N: usize> Add<Self> for Matrix<F, M, N> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}
impl<F: Field, const M: usize, const N: usize> Sub<&Self> for Matrix<F, M, N> {
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self {
        self -= rhs;
        self
    }
}
impl<F: Field, const M: usize, const N: usize> Sub<Self> for Matrix<F, M, N> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}
impl<F: Field, const M: usize, const N: usize> AddAssign<&Self> for Matrix<F, M, N> {
    fn add_assign(&mut self, rhs: &Self) {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] += rhs[(i,j)]
            }
        }
    }
}
impl<F: Field, const M: usize, const N: usize> AddAssign<Self> for Matrix<F, M, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] += rhs[(i,j)]
            }
        }
    }
}
impl<F: Field, const M: usize, const N: usize> SubAssign<&Self> for Matrix<F, M, N> {
    fn sub_assign(&mut self, rhs: &Self) {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] -= rhs[(i,j)]
            }
        }
    }
}
impl<F: Field, const M: usize, const N: usize> SubAssign<Self> for Matrix<F, M, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] -= rhs[(i,j)]
            }
        }
    }
}

// Scalar multiplication and division
impl<T: Into<F>, F: Field, const M: usize, const N: usize> Mul<T> for Matrix<F, M, N> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self {
        let scale = rhs.into();
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] *= scale;
            }
        }
        self
    }
}
impl<T: Into<F>, F: Field, const M: usize, const N: usize> Div<T> for Matrix<F, M, N> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        let scale = rhs.into();
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] /= scale;
            }
        }
        self
    }
}
impl<T: Into<F>, F: Field, const M: usize, const N: usize> MulAssign<T> for Matrix<F, M, N> {
    fn mul_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] *= scale;
            }
        }
    }
}
impl<T: Into<F>, F: Field, const M: usize, const N: usize> DivAssign<T> for Matrix<F, M, N> {
    fn div_assign(&mut self, rhs: T) {
        let scale = rhs.into();
        for i in 0..M {
            for j in 0..N {
                self[(i,j)] /= scale;
            }
        }
    }
}

// Matrix multiplication
impl<F: Field, const M: usize, const N: usize, const P: usize> Mul<&Matrix<F, P, N>> for &Matrix<F, M, P> {
    type Output = Matrix<F, M, N>;
    fn mul(self, rhs: &Matrix<F, P, N>) -> Matrix<F, M, N> {
        let mut out = Matrix::<F, M, N>::zero();
        for i in 0..M {
            for j in 0..N {
                for k in 0..P {
                    out[(i,j)] += self[(i,k)] * rhs[(k, j)];
                }
            }
        }
        out
    }
}

impl<F: Field, const M: usize, const N: usize> VectorSpace<F> for Matrix<F, M, N> {
    fn zero() -> Self {
        Self([[F::zero(); N]; M])
    }
}

impl<const M: usize, const N: usize> VectorSpace<Real> for Matrix<Complex, M, N> {
    fn zero() -> Self {
        <Matrix<Complex, M, N> as VectorSpace<Complex>>::zero()
    }
}
impl<const M: usize, const N: usize> VectorSpace<Rational> for Matrix<Real, M, N> {
    fn zero() -> Self {
        <Matrix<Real, M, N> as VectorSpace<Real>>::zero()
    }
}
impl<const M: usize, const N: usize> VectorSpace<Rational> for Matrix<Complex, M, N> {
    fn zero() -> Self {
        <Matrix<Complex, M, N> as VectorSpace<Complex>>::zero()
    }
}


#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::field::Rational;
    use crate::vector_space::VectorSpace;

    #[test]
    fn matrix_zero() {
        assert_eq!(<Matrix::<Rational, 3, 4> as VectorSpace<Rational>>::zero(), matrix![[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn matrix_scaling() {
        assert_eq!(matrix![[1, 0, -2], [-3, 2, 3]] * -3, matrix![[-3, 0, 6], [9, -6, -9]]);
        assert_eq!(matrix![[2, 5, -3], [1, 0, 7]] * 4, matrix![[8, 20, -12], [4, 0, 28]]);
        assert_eq!(matrix![[-6, 4], [3, -2], [1, 8]] * -5,  matrix![[30, -20], [-15, 10], [-5, -40]]);
    }

    #[test]
    fn matrix_addition() {
        assert_eq!(matrix![[2, 0 , -1], [1, -3, 4]] + matrix![[-5, -2, 6], [3, 4, -1]], matrix![[-3, -2, 5], [4, 1, 3]]);
        assert_eq!(matrix![[2, 5, -3], [1, 0, 7]] + matrix![[4, -2, 5], [-5, 3, 2]], matrix![[6, 3, 2], [-4, 3, 9]]);
        assert_eq!(matrix![[-6, 4], [3, -2], [1, 8]] + matrix![[7, -5], [0, -3], [2, 0]], matrix![[1, -1], [3, -5], [3, 8]]);
        assert_eq!(matrix![[8, 3, 1], [3, 0, 0], [3, 0, 0]] + matrix![[9, 1, 4], [3, 0, 0], [1, 1, 0]], matrix![[17, 4, 5], [6, 0, 0], [4, 1, 0]]);
    }

    #[test]
    fn matrix_subtraction() {
        let m = matrix![[4, 2, 1, 3], [5, 1, 1, 4], [3, 1, 2, 6]];
        let a = matrix![[5, 3, 1, 2], [6, 2, 1, 5], [1, 0, 3, 3]];
        assert_eq!(m*2 - a, matrix![[3, 1, 1, 4], [4, 0, 1, 3], [5, 2, 1, 9]]);
    }

    #[test]
    fn matrix_indexing() {
        let m = matrix![[1, 2, 3], [4, 5, 6]];
        assert_eq!(m[(0,2)], 3);
        assert_eq!(m[(1,0)], 4);
        assert_eq!(m[(1,1)], 5);
    }
}