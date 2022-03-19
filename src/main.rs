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
    let image_width = 1000;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let mut image = vec![vec![Color::new(0., 0., 0.); image_width]; image_height];

    let white = Color::new(1., 1., 1.);

    let model = Model::new("obj/african_head.obj").unwrap();

    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            let v1 = model.vert(face[(j + 1) % 3] as usize);

            let x0 = ((v0.x + 1.) * image_width as f32 / 2.) as isize;
            let x1 = ((v1.x + 1.) * image_width as f32 / 2.) as isize;
            let y0 = ((v0.y + 1.) * image_height as f32 / 2.) as isize;
            let y1 = ((v1.y + 1.) * image_height as f32 / 2.) as isize;

            // eprintln!("{x0} {y0} {x1} {y1}");
            line(x0, y0, x1, y1, &mut image, white);
        }
    }

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
