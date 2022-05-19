#![allow(dead_code)]
use super::coord::{Coord3, Coord4};
use num_traits::Float;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Matrix3<T: Float>(pub [[T; 3]; 3]);
#[derive(Clone, Copy, Debug)]
pub struct Matrix4<T: Float>(pub [[T; 4]; 4]);

impl<T: Float> Matrix3<T> {
    pub fn dotv(&self, o: &Coord3<T>) -> Coord3<T> {
        let o = [o.x, o.y, o.z];
        let mut res = [T::zero(); 4];
        for i in 0..3 {
            for j in 0..3 {
                res[i] = res[i] + self.0[i][j] * o[j];
            }
        }
        Coord3::<T> {
            x: res[0],
            y: res[1],
            z: res[2],
        }
    }
}
impl<T: Float> Matrix4<T> {
    pub fn dotv(&self, o: &Coord4<T>) -> Coord4<T> {
        let o = [o.x, o.y, o.z, o.w];
        let mut res = [T::zero(); 4];
        for i in 0..4 {
            for j in 0..4 {
                res[i] = res[i] + self.0[i][j] * o[j];
            }
        }
        Coord4::<T> {
            x: res[0],
            y: res[1],
            z: res[2],
            w: res[3],
        }
    }
}

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
        impl<T: Clone + Copy + $name<Output = T> + Float> $name<T> for Matrix4<T> {
            type Output = Self;

            fn $funcname(mut self, other: T) -> Self {
                for i in 0..4 {
                    for j in 0..4 {
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn simple_arith_test() {
        let P3 = Matrix3::<f32>([[3., 7., -3.], [9., 8., 30.], [-3., -4., -10.]]);
        let v3 = Coord3::<f32> {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let res3 = P3.dotv(&v3);
        assert_eq!(res3.x, 8.);
        assert_eq!(res3.y, 115.);
        assert_eq!(res3.z, -41.);

        let P4 = Matrix4::<f32>([
            [1., 5., 9., 13.],
            [2., 6., 0., -14.],
            [0., 0., 0., 0.],
            [0., 0., 1., 1.],
        ]);
        let v4 = v3.homogenize();
        let res4 = P4.dotv(&v4);
        assert_eq!(res4.x, 51.);
        assert_eq!(res4.y, 0.);
        assert_eq!(res4.z, 0.);
        assert_eq!(res4.w, 4.);
    }
}
