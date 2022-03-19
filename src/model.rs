use super::coord::Coord3;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub struct Model {
    verts: Vec<Coord3<f32>>,
    faces: Vec<[isize; 3]>,
}

impl Model {
    /// Read an `.obj` file of the given path and construct the 3D model
    pub fn new(path: &str) -> io::Result<Self> {
        let (mut verts, mut faces) = (Vec::new(), Vec::new());
        let f = BufReader::new(File::open(path)?);
        for line in f.lines() {
            let line = line.unwrap();
            if line.len() >= 2 {
                match &line[0..2] {
                    "v " => {
                        let data: Vec<&str> = line.split(' ').collect();
                        let x = data[1].parse::<f32>().unwrap();
                        let y = data[2].parse::<f32>().unwrap();
                        let z = data[3].parse::<f32>().unwrap();
                        verts.push(Coord3 { x, y, z });
                    }
                    "f " => {
                        let data: Vec<&str> = line.split(' ').collect();
                        let mut f = [0; 3];
                        for i in 0..3 {
                            let value: Vec<&str> = data[i + 1].split('/').collect();
                            f[i] = value[0].parse::<isize>().unwrap() - 1; // in wavefront obj all indices start at 1, not zero
                        }
                        faces.push(f);
                    }
                    _ => {}
                }
            }
        }
        Ok(Self { verts, faces })
    }
    pub fn face(&self, idx: usize) -> [isize; 3] {
        self.faces[idx]
    }
    pub fn vert(&self, idx: usize) -> Coord3<f32> {
        self.verts[idx]
    }
    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }
}
