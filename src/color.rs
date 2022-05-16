use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b)
    }
}

macro_rules! implBasicArith {
    ($name: tt, $op: tt, $funcname:tt) => {
        impl $name for Color {
            type Output = Self;

            fn $funcname(self, other: Self) -> Self {
                Self (
                    self.0 $op other.0,
                    self.1 $op other.1,
                    self.2 $op other.2
                )
            }
        }
        impl $name<f32> for Color {
            type Output = Self;

            fn $funcname(self, other: f32) -> Self {
                Self
                    ( self.0 $op other,
                     self.1 $op other,
                     self.2 $op other,)

            }
        }
    };
}

implBasicArith!(Add, +, add);
implBasicArith!(Mul, *, mul);
implBasicArith!(Sub, -, sub);
implBasicArith!(Div, /, div);
