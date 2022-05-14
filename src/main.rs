#![allow(unused_imports)]
#![allow(dead_code)]
mod color;
mod coord;
mod draw;
mod model;
use num_traits::Float;
use rand::prelude::*;

use color::Color;
use coord::{Coord2, Coord3};
use draw::draw_print;
use model::Model;

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

    for i in 0..model.nfaces() {
        let face = model.face(i);
        let v3f: Vec<Coord3<f32>> = (0..3).map(|i| model.vert(face[i] as usize)).collect();
        let mut normal = (v3f[2] - v3f[0]).cross(v3f[1] - v3f[0]);
        normal.normalize();
        let light_dir = Coord3 {
            x: 0.,
            y: 0.,
            z: -1.,
        };
        let l_dot_n = normal.dot(&light_dir);
        let brightness = l_dot_n;

        if brightness > 0. {
            fill_triangle(
                v3f[0],
                v3f[1],
                v3f[2],
                &mut image,
                Color::new(brightness, brightness, brightness),
                &mut zbuf,
            );
        }
    }

    // output rendering result
    draw_print(&image);
}

/// fills the triangle with the given color.
fn fill_triangle<T: Copy>(
    a: Coord3<f32>,
    b: Coord3<f32>,
    c: Coord3<f32>,
    image: &mut Image<T>,
    color: T,
    zbuf: &mut Image<f32>,
) {
    let v3f = [a, b, c];
    let v2i: Vec<Coord2<isize>> = v3f
        .iter()
        .map(|v| Coord2 {
            x: ((v.x + 1f32) * image.width as f32 / 2.) as isize,
            y: ((v.y + 1f32) * image.height as f32 / 2.) as isize,
        })
        .collect();
    let (ai, bi, ci) = (v2i[0], v2i[1], v2i[2]);
    let bbox = find_bounding_box(ai, bi, ci, image.width as isize, image.height as isize);
    for x in bbox[0].x..bbox[1].x {
        for y in bbox[0].y..bbox[1].y {
            let (u, v, w) = vert_weight(Coord2 { x, y }, ai, bi, ci);
            let z = u * a.z + v * b.z + w * c.z;
            if is_inside(Coord2 { x, y }, ai, bi, ci) && zbuf.get(x as usize, y as usize) < &z {
                image.set(x as usize, y as usize, color);
                zbuf.set(x as usize, y as usize, z);
            }
        }
    }
}

/// Returns the bouding box of the triangle in the format of
/// `[topleft(x,y), bottomright(x,y)]`.
/// Also, clips the given coordinate so that the returned value will be within the image region.
fn find_bounding_box(
    t0: Coord2<isize>,
    t1: Coord2<isize>,
    t2: Coord2<isize>,
    width: isize,
    height: isize,
) -> [Coord2<isize>; 2] {
    let mut x = [t0.x, t1.x, t2.x];
    let mut y = [t0.y, t1.y, t2.y];
    x.sort();
    y.sort();
    return [
        Coord2 {
            x: 0.max(x[0]),
            y: 0.max(y[0]),
        },
        Coord2 {
            x: width.max(x[2]),
            y: height.max(y[2]),
        },
    ];
}

/// Returns if the point `p` is within the triangle connecting `a`, `b`, and `c`.
fn is_inside(p: Coord2<isize>, a: Coord2<isize>, b: Coord2<isize>, c: Coord2<isize>) -> bool {
    let (u, v, w) = vert_weight(p, a, b, c);
    return 0. <= u && 0. <= v && 0. <= w;
}

/// Returns $x,y,z$ where the point `p` is represented as $xa+yb+zc$.
fn vert_weight(
    p: Coord2<isize>,
    a: Coord2<isize>,
    b: Coord2<isize>,
    c: Coord2<isize>,
) -> (f32, f32, f32) {
    let ab = b - a;
    let ac = c - a;
    let pa = a - p;
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
