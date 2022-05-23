mod camera;
mod draw;
mod img;
mod linalg;
mod model;
mod render;

use draw::draw_print;
use img::{Color, Image};
use linalg::coord;
use linalg::coord::{Coord2, Coord3};
use model::Model;
use render::render;

fn main() {
    render();
}
