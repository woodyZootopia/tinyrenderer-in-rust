use num_traits::Float;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Coord2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Coord3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Coord4<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Float> Coord3<T> {
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn len_sq(&self) -> T {
        self.dot(self)
    }
    pub fn normalize(&mut self) {
        *self = *self / self.len_sq().sqrt()
    }
    pub fn homogenize(&self) -> Coord4<T> {
        Coord4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: T::one(),
        }
    }
}

impl<T: Float> Coord4<T> {
    pub fn inhomogenize(&self) -> Coord3<T> {
        Coord3 {
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w,
        }
    }
}

macro_rules! implBasicArith {
    ($name: tt, $op: tt, $funcname:tt) => {
        impl<T: Clone+Copy+$name<Output = T>> $name for Coord2<T> {
            type Output = Self;

            fn $funcname(self, other: Self) -> Self {
                Self {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                }
            }
        }
        impl<T: Clone+Copy+$name<Output = T>> $name<T> for Coord2<T> {
            type Output = Self;

            fn $funcname(self, other: T) -> Self {
                Self {
                    x: self.x $op other,
                    y: self.y $op other,
                }
            }
        }
        impl<T: Clone+Copy+$name<Output = T>+Float> $name for Coord3<T> {
            type Output = Self;

            fn $funcname(self, other: Self) -> Self {
                Self {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                    z: self.z $op other.z,
                }
            }
        }
        impl<T: Clone+Copy+$name<Output = T>+Float> $name<T> for Coord3<T> {
            type Output = Self;

            fn $funcname(self, other: T) -> Self {
                Self {
                    x: self.x $op other,
                    y: self.y $op other,
                    z: self.z $op other,
                }
            }
        }
    };
}

implBasicArith!(Add, +, add);
implBasicArith!(Mul, *, mul);
implBasicArith!(Sub, -, sub);
implBasicArith!(Div, /, div);
