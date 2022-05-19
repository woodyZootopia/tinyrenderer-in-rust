mod draw;
mod img;
mod linalg;
mod model;
mod util;

use draw::draw_print;
use img::{Color, Image};
use linalg::coord;
use linalg::coord::{Coord2, Coord3};
use linalg::matrix::Matrix4;
use model::Model;
use util::fill_triangle;

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
