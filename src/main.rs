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

    line(90, 10, 70, 80, &mut image, Color::new(1., 1., 1.));

    // output rendering result
    draw_print(image);
}

fn line(h0: isize, w0: isize, h1: isize, w1: isize, image: &mut Vec<Vec<Color>>, color: Color) {
    let num_point = 100;
    for i in 0..num_point {
        let t = i as f32 / num_point as f32;
        let (h, w) = (
            h0 as f32 + (h1 - h0) as f32 * t,
            w0 as f32 + (w1 - w0) as f32 * t,
        );
        image[h as usize][w as usize] = color;
    }
}
