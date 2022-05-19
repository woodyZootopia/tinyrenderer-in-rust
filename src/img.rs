use image::{DynamicImage, GenericImageView};
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

pub struct Image<T> {
    img: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Copy> Image<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            img: vec![default.clone(); width * height],
            width,
            height,
        }
    }
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.img[x * self.height + y]
    }
    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, color: T) {
        self.img[x * self.height + y] = color;
    }
}

impl Into<Image<Color>> for DynamicImage {
    fn into(self) -> Image<Color> {
        let mut output = Image::new(
            self.width() as usize,
            self.height() as usize,
            Color::new(0., 0., 0.),
        );
        match self {
            DynamicImage::ImageRgb8(_) => {
                for x in 0..self.width() {
                    for y in 0..self.height() {
                        let pixel = self.get_pixel(x, y);
                        let val: Color = Color::new(
                            pixel.0[0] as f32 / 255.,
                            pixel.0[1] as f32 / 255.,
                            pixel.0[2] as f32 / 255.,
                        );
                        output.set(x as usize, y as usize, val);
                    }
                }
            }
            _ => {
                panic!("Not supported image type")
            }
        }
        return output;
    }
}
