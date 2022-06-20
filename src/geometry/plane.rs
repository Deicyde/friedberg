use crate::field::Real;
use crate::vector_space::Tuple;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
/// A type representing a plane in N-dimenstional space.
pub struct Plane<const N: usize> {
    initial: Tuple<Real, N>,
    direction_1: Tuple<Real, N>,
    direction_2: Tuple<Real, N>,
}


impl<const N: usize> Plane<N> {
    /// Creates a new plane with the given initial point and direction vectors.
    /// Implicitly assumes that the two direction vectors are not paralllel.
    fn new(initial: Tuple<Real, N>, direction_1: Tuple<Real, N>, direction_2: Tuple<Real, N>) -> Self {
        Self {
            initial,
            direction_1,
            direction_2
        }
    }

    /// Creates a new plane which passes through p1 at (t=0,s=0), p2 at (t=1,s=0), and p3 at (t=0,s=1).
    fn through_points(p1: Tuple<Real, N>, p2: Tuple<Real, N>, p3: Tuple<Real, N>) -> Self {
        let direction_1 = p2 - &p1;
        let direction_2 = p3 - &p1;
        let initial = p1;
        Self {
            initial,
            direction_1,
            direction_2
        }
    }

    /// Evalutates the paramateriazed version of the line at paramaters t,s.
    /// Returns (self.direction_1)*t + (self.direction_2)*s + self.inital.
    fn eval_at(&self, t: Real, s: Real) -> Tuple<Real, N> {
        (self.direction_1.clone()*t) + (self.direction_2.clone()*s) + &self.initial
    }
}

impl<const N: usize> fmt::Display for Plane<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+t{}+s{}", self.initial, self.direction_1, self.direction_2)
    }
}

#[cfg(test)]
mod tests {
    use super::Plane;
    use crate::tuple;

    #[test]
    fn plane_through_points() {
        assert_eq!(Plane::through_points(tuple![2.0, -5.0, -1.0], tuple![0.0, 4.0, 6.0],  tuple![-3.0, 7.0, 1.0]), 
                              Plane::new(tuple![2.0, -5.0, -1.0], tuple![-2.0, 9.0, 7.0], tuple![-5.0, 12.0, 2.0]));

        assert_eq!(Plane::through_points(tuple![3.0, -6.0, 7.0], tuple![-2.0, 0.0, -4.0],  tuple![5.0, -9.0, -2.0]),
                              Plane::new(tuple![3.0, -6.0, 7.0], tuple![-5.0, 6.0, -11.0], tuple![2.0, -3.0, -9.0]));

        assert_eq!(Plane::through_points(tuple![-8.0, 2.0, 0.0], tuple![1.0, 3.0, 0.0], tuple![6.0, -5.0, 0.0]),
                              Plane::new(tuple![-8.0, 2.0, 0.0], tuple![9.0, 1.0, 0.0], tuple![14.0, -7.0, 0.0]));

        assert_eq!(Plane::through_points(tuple![1.0, 1.0, 1.0], tuple![5.0, 5.0, 5.0], tuple![-6.0, 4.0, 2.0]),
                              Plane::new(tuple![1.0, 1.0, 1.0], tuple![4.0, 4.0, 4.0], tuple![-7.0, 3.0, 1.0]));
    }   
}