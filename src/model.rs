use cgmath::{Vector3, Zero};

use crate::Triangle;

#[derive(Debug)]
pub struct Model {
    pub pos: Vector3<f64>,
    pub rot: Vector3<f64>,
    pub scale: Vector3<f64>,

    pub polycons: Vec<Triangle>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            polycons: Vec::new(),
        }
    }

    pub fn translate(&mut self, v: Vector3<f64>) -> &mut Self {
        self.pos += v;
        self
    }

    pub fn rotate(&mut self, v: Vector3<f64>) -> &mut Self {
        self.rot += v;
        self
    }

    pub fn resize(&mut self, v: Vector3<f64>) -> &mut Self {
        self.scale += v;
        self
    }

    pub fn add_polygon(&mut self, polygon: Triangle) -> &mut Self {
        self.polycons.push(polygon);
        self
    }
}
