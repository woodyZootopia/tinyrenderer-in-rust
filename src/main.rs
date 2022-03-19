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

            let t0 = (v0 + 1f32) * image_width as f32 / 2.;
            let t1 = (v1 + 1f32) * image_width as f32 / 2.;

            line(
                Coord2 {
                    x: t0.x as isize,
                    y: t0.y as isize,
                },
                Coord2 {
                    x: t1.x as isize,
                    y: t1.y as isize,
                },
                &mut image,
                white,
            );
        }
    }

    // output rendering result
    draw_print(image);
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
