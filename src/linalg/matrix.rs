#![allow(dead_code)]
use super::coord::{Coord3, Coord4};
use num_traits::Float;
use std::cmp::Eq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix3<T: Float>(pub [[T; 3]; 3]);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub fn dotm(&self, o: &Matrix4<T>) -> Matrix4<T> {
        let mut res = Matrix4([[T::zero(); 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res.0[i][j] = res.0[i][j] + self.0[i][k] * o.0[k][j];
                }
            }
        }
        res
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
