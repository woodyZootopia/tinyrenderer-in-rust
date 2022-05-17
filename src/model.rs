use super::coord::Coord3;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub struct Model {
    /// tuple of triangles in the form of `(vert_idx, vert_texture_idx)`
    faces: Vec<([usize; 3], [usize; 3])>,
    /// coordinates of each vertices
    verts: Vec<Coord3<f32>>,
    /// coordinates of each vertices in the texture image
    vt: Vec<Coord3<f32>>,
}

impl Model {
    /// Read an `.obj` file of the given path and construct the 3D model
    pub fn new(path: &str) -> io::Result<Self> {
        let (mut verts, mut faces, mut vt) = (Vec::new(), Vec::new(), Vec::new());
        let f = BufReader::new(File::open(path)?);
        for line in f.lines() {
            let line = line.unwrap();
            if line.len() < 2 {
                continue;
            }
            match &line[0..2] {
                "v " => {
                    let mut data = line.split_whitespace();
                    data.next();
                    let x = data.next().unwrap().parse::<f32>().unwrap();
                    let y = data.next().unwrap().parse::<f32>().unwrap();
                    let z = data.next().unwrap().parse::<f32>().unwrap();
                    verts.push(Coord3 { x, y, z });
                }
                "f " => {
                    let mut data = line.split_whitespace();
                    data.next();
                    let mut face = [0; 3];
                    let mut texture = [0; 3];
                    for i in 0..3 {
                        let mut value = data.next().unwrap().split('/');
                        // in wavefront obj all indices start at 1, not zero
                        face[i] = value.next().unwrap().parse::<usize>().unwrap() - 1;
                        texture[(i + 2) % 3] = value.next().unwrap().parse::<usize>().unwrap() - 1;
                    }
                    faces.push((face, texture));
                }
                "vt" => {
                    let mut data = line.split_ascii_whitespace();
                    data.next();
                    let x = data.next().unwrap().parse::<f32>().unwrap();
                    let y = data.next().unwrap().parse::<f32>().unwrap();
                    let z = data.next().unwrap().parse::<f32>().unwrap();
                    vt.push(Coord3 { x, y, z });
                }
                _ => {}
            }
        }
        Ok(Self { verts, faces, vt })
    }
    pub fn face(&self, idx: usize) -> ([usize; 3], [usize; 3]) {
        self.faces[idx]
    }
    pub fn vt(&self, idx: usize) -> Coord3<f32> {
        self.vt[idx]
    }
    pub fn vert(&self, idx: usize) -> Coord3<f32> {
        self.verts[idx]
    }
    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }
}
