#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Color = Vec3;

impl Vec3 {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { e: [r, g, b] }
    }
}
