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

fn line(x0: isize, y0: isize, x1: isize, y1: isize, image: &mut Vec<Vec<Color>>, color: Color) {
    let (y0, y1) = (image.len() as isize - y0, image.len() as isize - y1);
    for x in x0..x1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 + (y1 - y0) as f32 * t;
        image[y as usize][x as usize] = color;
    }
}
