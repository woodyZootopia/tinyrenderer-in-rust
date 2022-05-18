use super::coord::Coord3;
use num_traits::{Float, PrimInt};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Matrix3<T: Float>(pub [[T; 3]; 3]);

impl<T: Float> Matrix3<T> {
    pub fn dotv(&self, o: &Coord3<T>) -> Coord3<T> {
        Coord3::<T> {
            x: self.0[0][0] * o.x + self.0[0][1] * o.y + self.0[0][2] * o.z,
            y: self.0[1][0] * o.x + self.0[1][1] * o.y + self.0[1][2] * o.z,
            z: self.0[2][0] * o.x + self.0[2][1] * o.y + self.0[2][2] * o.z,
        }
    }
}

// impl<T: Float> Mul<Coord3<T>> for Matrix3<T> {
//     type Output = Coord3<T>;
//     fn mul(self, other: Coord3<T>) -> Self::Output {
//         self.dotv(&other)
//     }
// }

macro_rules! implBasicArith {
    ($name:tt, $op:tt, $funcname:tt) => {
        impl<T: Clone + Copy + $name<Output = T> + Float> $name<T> for Matrix3<T> {
            type Output = Self;

            fn $funcname(mut self, other: T) -> Self {
                for i in 0..3 {
                    for j in 0..3 {
                        self.0[i][j] = self.0[i][j] * other;
                    }
                }
                self
            }
        }
    };
}

implBasicArith!(Add, +, add);
implBasicArith!(Mul, *, mul);
implBasicArith!(Sub, -, sub);
implBasicArith!(Div, /, div);
