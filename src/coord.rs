use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Coord3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Copy)]
pub struct Coord2<T> {
    pub x: T,
    pub y: T,
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
        impl<T: Clone+Copy+$name<Output = T>> $name for Coord3<T> {
            type Output = Self;

            fn $funcname(self, other: Self) -> Self {
                Self {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                    z: self.z $op other.z,
                }
            }
        }
        impl<T: Clone+Copy+$name<Output = T>> $name<T> for Coord3<T> {
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
