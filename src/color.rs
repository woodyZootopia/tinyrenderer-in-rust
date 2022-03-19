#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3],
}

pub type Color = Vec3;

impl Vec3 {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { e: [r, g, b] }
    }
}
