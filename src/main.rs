mod color;
mod draw;

use color::Color;
use draw::draw_print;

fn main() {
    render();
}
pub fn render() {
    let aspect_ratio = 1f32;
    let image_width = 100;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let mut image = vec![vec![Color::new(0., 0., 0.); image_width]; image_height];

    // line(90, 10, 70, 80, &mut image, Color::new(1., 1., 1.));
    let white = Color::new(1., 1., 1.);
    let red = Color::new(1., 0., 0.);
    line(13, 20, 80, 40, &mut image, white);
    line(20, 13, 40, 80, &mut image, red);
    line(80, 40, 13, 20, &mut image, red);

    // output rendering result
    draw_print(image);
}

fn line(
    mut x0: isize,
    mut y0: isize,
    mut x1: isize,
    mut y1: isize,
    image: &mut Vec<Vec<Color>>,
    color: Color,
) {
    use std::mem;
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
    for x in x0..x1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 + (y1 - y0) as f32 * t;
        if steep {
            image[x as usize][y as usize] = color;
        } else {
            image[y as usize][x as usize] = color;
        }
    }
}
