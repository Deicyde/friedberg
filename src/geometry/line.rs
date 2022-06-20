use crate::field::Real;
use crate::vector_space::Tuple;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
/// A type representing a line in N-dimensional space.
pub struct Line<const N: usize> {
    initial: Tuple<Real, N>,
    direction: Tuple<Real, N>,
}

impl<const N: usize> Line<N> {
    /// Creates a new line with the given initial point and direction vector.
    fn new(initial: Tuple<Real, N>, direction: Tuple<Real, N>) -> Self {
        Self {
            initial,
            direction
        }
    }

    /// Creates a new line which starts at p1 and passes through p2 at t=1.
    fn through_points(p1: Tuple<Real, N>, p2: Tuple<Real, N>) -> Self {
        let direction = p2 - &p1;
        let initial = p1;
        Self {
            initial,
            direction
        }
    }

    /// Evalutates the paramateriazed version of the line at time t.
    /// Returns (self.direction)*t + self.inital.
    fn eval_at(&self, t: Real) -> Tuple<Real, N> {
        (self.direction.clone()*t) + &self.initial
    }
}

impl<const N: usize> fmt::Display for Line<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+t{}", self.initial, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::tuple;

    #[test]
    fn line_through_points() {
        assert_eq!(Line::through_points(tuple![3.0, -2.0, 4.0], tuple![-5.0, 7.0, 1.0]), 
                              Line::new(tuple![3.0, -2.0, 4.0], tuple![-8.0, 9.0, -3.0]));

        assert_eq!(Line::through_points(tuple![2.0, 4.0, 0.0], tuple![-3.0, -6.0, 0.0]),
                              Line::new(tuple![2.0, 4.0, 0.0], tuple![-5.0, -10.0, 0.0]));

        assert_eq!(Line::through_points(tuple![3.0, 7.0, 2.0], tuple![3.0, 7.0, -8.0]),
                              Line::new(tuple![3.0, 7.0, 2.0], tuple![0.0, 0.0, -10.0]));

        assert_eq!(Line::through_points(tuple![3.0, 7.0, 2.0], tuple![3.0, 7.0, -8.0]),
                              Line::new(tuple![3.0, 7.0, 2.0], tuple![0.0, 0.0, -10.0]));

        assert_eq!(Line::through_points(tuple![-2.0, -1.0, 5.0], tuple![3.0, 9.0, 7.0]),
                              Line::new(tuple![-2.0, -1.0, 5.0], tuple![5.0, 10.0, 2.0]));
    }   
}