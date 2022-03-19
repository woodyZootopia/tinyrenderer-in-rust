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
    t0: Coord2<isize>,
    t1: Coord2<isize>,
    t2: Coord2<isize>,
    mut image: &mut Vec<Vec<Color>>,
    color: Color,
) {
    // sort by y coord
    let x = [t0, t1, t2];
    let mut y = [(0, t0), (1, t1), (2, t2)];
    y.sort_by_key(|x| x.1.y);
    let (top, mid, bot) = (x[y[0].0], x[y[1].0], x[y[2].0]);

    for y in top.y..mid.y {
        let x_left =
            top.x as f32 + (top.x - mid.x) as f32 / (top.y - mid.y) as f32 * (y - top.y) as f32;
        let x_right =
            top.x as f32 + (top.x - bot.x) as f32 / (top.y - bot.y) as f32 * (y - top.y) as f32;
        line(
            Coord2 {
                x: x_left as isize,
                y,
            },
            Coord2 {
                x: x_right as isize,
                y,
            },
            &mut image,
            color,
        );
    }
    for y in mid.y..bot.y {
        let x_right =
            top.x as f32 + (top.x - bot.x) as f32 / (top.y - bot.y) as f32 * (y - top.y) as f32;
        let x_left =
            mid.x as f32 + (mid.x - bot.x) as f32 / (mid.y - bot.y) as f32 * (y - mid.y) as f32;
        horiz_line(x_left as isize, x_right as isize, y, &mut image, color);
    }
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
