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
    let red = Color::new(1., 0., 0.);
    let green = Color::new(0., 1., 0.);

    sweep_triangle(
        Coord2 { x: 10, y: 70 },
        Coord2 { x: 50, y: 160 },
        Coord2 { x: 70, y: 80 },
        &mut image,
        red,
    );
    sweep_triangle(
        Coord2 { x: 180, y: 50 },
        Coord2 { x: 150, y: 1 },
        Coord2 { x: 70, y: 180 },
        &mut image,
        white,
    );
    sweep_triangle(
        Coord2 { x: 180, y: 150 },
        Coord2 { x: 120, y: 160 },
        Coord2 { x: 130, y: 180 },
        &mut image,
        green,
    );

    // output rendering result
    draw_print(image);
}

fn sweep_triangle(
    a: Coord2<isize>,
    b: Coord2<isize>,
    c: Coord2<isize>,
    mut image: &mut Vec<Vec<Color>>,
    color: Color,
) {
    let bbox = find_bounding_box(a, b, c, image[0].len() as isize, image.len() as isize);
    for x in bbox[0].x..bbox[1].x {
        for y in bbox[0].y..bbox[1].y {
            if inside(Coord2 { x, y }, a, b, c) {
                image[y as usize][x as usize] = color;
            }
        }
    }
}

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

fn inside(p: Coord2<isize>, a: Coord2<isize>, b: Coord2<isize>, c: Coord2<isize>) -> bool {
    let ab = b - a;
    let ac = c - a;
    let pa = a - p;
    let n = [
        ac.x as f32 * pa.y as f32 - ac.y as f32 * pa.x as f32,
        pa.x as f32 * ab.y as f32 - pa.y as f32 * ab.x as f32,
        ab.x as f32 * ac.y as f32 - ab.y as f32 * ac.x as f32,
    ];
    let u = n[0] / n[2];
    let v = n[1] / n[2];
    let w = 1. - u - v;
    return 0. <= u && 0. <= v && 0. <= w;
}

fn horiz_line(
    mut x_left: isize,
    mut x_right: isize,
    y: isize,
    image: &mut Vec<Vec<Color>>,
    color: Color,
) {
    if x_left > x_right {
        use std::mem::swap;
        swap(&mut x_left, &mut x_right);
    }
    for x in x_left..x_right {
        image[y as usize][x as usize] = color;
    }
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
