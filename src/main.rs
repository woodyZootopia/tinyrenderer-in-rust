#![allow(unused_imports)]
#![allow(dead_code)]
use image;
use image::{DynamicImage, GenericImageView};
mod color;
mod draw;
mod linalg;
mod model;
use num_traits::Float;
use rand::prelude::*;

use color::Color;
use draw::draw_print;
use linalg::coord;
use linalg::coord::{Coord2, Coord3};
use linalg::matrix::Matrix3;
use model::Model;
use std::ops::{Add, Div, Mul, Sub};

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

fn main() {
    render();
}
pub fn render() {
    let aspect_ratio = 1f32;
    let image_width = 1000;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let mut image = Image::new(image_width, image_height, Color::new(0., 0., 0.));
    let mut zbuf = Image::new(image_width, image_height, f32::MIN);

    let model = Model::new("obj/african_head.obj").unwrap();

    let texture: Image<Color> = image::open("obj/african_head_diffuse.png").unwrap().into();

    for i in 0..model.nfaces() {
        let face = model.face(i);
        let v3f = [
            model.vert(face.0[0]),
            model.vert(face.0[1]),
            model.vert(face.0[2]),
        ];
        let txt_coords = [
            model.vt(face.1[0]),
            model.vt(face.1[1]),
            model.vt(face.1[2]),
        ]
        .map(|x| (x.x, x.y));

        // normalを計算
        let mut normal = (v3f[2] - v3f[0]).cross(v3f[1] - v3f[0]);
        normal.normalize();
        let light_dir = Coord3 {
            x: 0.,
            y: 0.,
            z: -1.,
        };
        let brightness = normal.dot(&light_dir);

        // 画面バッファに出力
        if brightness > 0. {
            fill_triangle(v3f, &mut image, &texture, txt_coords, brightness, &mut zbuf);
        }
    }

    // output rendering result
    draw_print(&image);
}

/// fills the triangle with the given texture info.
fn fill_triangle<T: Copy + Mul<f32, Output = T> + Add<T, Output = T>>(
    v3f: [Coord3<f32>; 3],
    image: &mut Image<T>,
    texture: &Image<T>,
    txt_coords: [(f32, f32); 3],
    brightness: f32,
    zbuf: &mut Image<f32>,
) {
    // calculate bounding box
    // let P = Matrix3::<f32>([[2., 2., 2.], [2., 2., 2.], [2., 2., 2.]]);
    let P = Matrix3::<f32>([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]) * 2.;
    let v2f: Vec<Coord2<f32>> = v3f
        .iter()
        .map(|v| P.dotv(v))
        .map(|v| Coord2 {
            x: ((v.x + 1f32) * image.width as f32 / 2.),
            y: ((v.y + 1f32) * image.height as f32 / 2.),
        })
        .collect();
    let [a, b, c] = [v2f[0], v2f[1], v2f[2]];
    let bbox = find_bounding_box(a, b, c, image.width as isize, image.height as isize);

    for x in bbox[0].x..bbox[1].x {
        for y in bbox[0].y..bbox[1].y {
            let (u, v, w) = vert_weight(Coord2 { x, y }, a, b, c);
            let z = u * v3f[0].z + v * v3f[1].z + w * v3f[2].z;
            if is_inside(Coord2 { x, y }, a, b, c) && zbuf.get(x as usize, y as usize) < &z {
                // decide color
                let txt_x = u * txt_coords[0].0 + v * txt_coords[1].0 + w * txt_coords[2].0;
                let txt_y = u * txt_coords[0].1 + v * txt_coords[1].1 + w * txt_coords[2].1;
                let txt_x = txt_x * texture.width as f32;
                let txt_y = txt_y * texture.height as f32;
                let color = bilinear_interp(texture, txt_x, txt_y);

                image.set(x as usize, y as usize, color * brightness);
                zbuf.set(x as usize, y as usize, z);
            }
        }
    }
}

fn bilinear_interp<T: Copy + Mul<f32, Output = T> + Add<T, Output = T>>(
    texture: &Image<T>,
    x: f32,
    y: f32,
) -> T {
    assert!(x < texture.width as f32 && y < texture.height as f32);
    let (xi, yi) = (x as i32, y as i32);
    let d = (x - xi as f32, y - yi as f32);
    *texture.get(xi as usize, yi as usize) * (1. - d.0) * (1. - d.1)
        + *texture.get(xi as usize + 1, yi as usize) * d.0 * (1. - d.1)
        + *texture.get(xi as usize, yi as usize + 1) * (1. - d.0) * d.1
        + *texture.get(xi as usize + 1, yi as usize + 1) * d.0 * d.1
}

/// Returns the bouding box of the triangle in the format of
/// `[topleft(x,y), bottomright(x,y)]`.
/// Also, clips the given coordinate so that the returned value will be within the image region.
fn find_bounding_box(
    t0: Coord2<f32>,
    t1: Coord2<f32>,
    t2: Coord2<f32>,
    width: isize,
    height: isize,
) -> [Coord2<isize>; 2] {
    let mut x = [t0.x as isize, t1.x as isize, t2.x as isize];
    let mut y = [t0.y as isize, t1.y as isize, t2.y as isize];
    x.sort();
    y.sort();
    return [
        Coord2 {
            x: 0.max(x[0]),
            y: 0.max(y[0]),
        },
        Coord2 {
            x: width.min(x[2] + 1),
            y: height.min(y[2] + 1),
        },
    ];
}

/// Returns if the pixel `p` is within the triangle connecting `a`, `b`, and `c`.
fn is_inside(p: Coord2<isize>, a: Coord2<f32>, b: Coord2<f32>, c: Coord2<f32>) -> bool {
    let (u, v, w) = vert_weight(p, a, b, c);
    return 0. <= u && 0. <= v && 0. <= w;
}

/// Given an triangle `a,b,c`, return `x,y,z` where the pixel `p` is represented as `xa+yb+zc`.
fn vert_weight(
    p: Coord2<isize>,
    a: Coord2<f32>,
    b: Coord2<f32>,
    c: Coord2<f32>,
) -> (f32, f32, f32) {
    let pf = Coord2 {
        x: p.x as f32,
        y: p.y as f32,
    };

    let ab = b - a;
    let ac = c - a;
    let pa = a - pf;
    let n = Coord3 {
        x: ab.x as f32,
        y: ac.x as f32,
        z: pa.x as f32,
    }
    .cross(Coord3 {
        x: ab.y as f32,
        y: ac.y as f32,
        z: pa.y as f32,
    });
    let u = n.x as f32 / n.z as f32;
    let v = n.y as f32 / n.z as f32;
    let w = 1. - u - v;
    return (u, v, w);
}

fn horiz_line<T: Copy>(
    mut x_left: isize,
    mut x_right: isize,
    y: isize,
    image: &mut Image<T>,
    color: T,
) {
    if x_left > x_right {
        use std::mem::swap;
        swap(&mut x_left, &mut x_right);
    }
    for x in x_left..x_right {
        image.set(x as usize, y as usize, color);
    }
}

/// Draw a triangle connecting the given points.
fn triangle<T: Copy>(
    t0: Coord2<isize>,
    t1: Coord2<isize>,
    t2: Coord2<isize>,
    image: &mut Image<T>,
    color: T,
) {
    line(t0, t1, image, color);
    line(t1, t2, image, color);
    line(t2, t0, image, color);
}

/// Draw a line from `t0` to `t1`.
fn line<T: Copy>(t0: Coord2<isize>, t1: Coord2<isize>, image: &mut Image<T>, color: T) {
    use std::mem;
    let mut x0 = t0.x;
    let mut y0 = t0.y;
    let mut x1 = t1.x;
    let mut y1 = t1.y;
    let steep = if (x0 - x1).abs() < (y0 - y1).abs() {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        true
    } else {
        false
    };
    if x0 > x1 {
        // make it left-to-right
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }
    for mut x in x0..x1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let mut y = (y0 as f32 + (y1 - y0) as f32 * t) as isize;
        if steep {
            mem::swap(&mut x, &mut y); // if transposed, de-transpose
        }
        if (0..image.width).contains(&(x as usize)) && (0..image.height).contains(&(y as usize)) {
            image.set(x as usize, y as usize, color);
        }
    }
}
