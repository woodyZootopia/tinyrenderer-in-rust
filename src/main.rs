#![allow(unused_imports)]
#![allow(dead_code)]
mod color;
mod coord;
mod draw;
mod model;

use color::Color;
use coord::Coord2;
use draw::draw_print;
use model::Model;

fn main() {
    render();
}
pub fn render() {
    let aspect_ratio = 1f32;
    let image_width = 200;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let mut image = vec![vec![Color::new(0., 0., 0.); image_width]; image_height];

    let white = Color::new(1., 1., 1.);

    triangle(
        Coord2 { x: 10, y: 70 },
        Coord2 { x: 50, y: 160 },
        Coord2 { x: 70, y: 80 },
        &mut image,
        white,
    );
    triangle(
        Coord2 { x: 180, y: 50 },
        Coord2 { x: 150, y: 1 },
        Coord2 { x: 70, y: 180 },
        &mut image,
        white,
    );
    triangle(
        Coord2 { x: 180, y: 150 },
        Coord2 { x: 120, y: 160 },
        Coord2 { x: 130, y: 180 },
        &mut image,
        white,
    );

    // output rendering result
    draw_print(image);
}

fn triangle(
    t0: Coord2<isize>,
    t1: Coord2<isize>,
    t2: Coord2<isize>,
    image: &mut Vec<Vec<Color>>,
    color: Color,
) {
    line(t0, t1, image, color);
    line(t1, t2, image, color);
    line(t2, t0, image, color);
}

fn line(t0: Coord2<isize>, t1: Coord2<isize>, image: &mut Vec<Vec<Color>>, color: Color) {
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
        if (0..image[0].len()).contains(&(x as usize)) && (0..image.len()).contains(&(y as usize)) {
            image[y as usize][x as usize] = color;
        }
    }
}
