use cgmath::{Deg, Matrix4, Point3, SquareMatrix, Vector3, Zero};

use crate::{Color, Triangle, Vertex};

#[derive(Debug)]
pub struct Model {
    pos: Vector3<f64>,
    rot: Vector3<f64>,
    scale: Vector3<f64>,

    polygons: Vec<Triangle>,
    model_matrix: Matrix4<f64>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            polygons: Vec::new(),
            model_matrix: Matrix4::zero(),
        }
    }

    pub fn triangle() -> Self {
        Model {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            polygons: vec![Triangle::default(), Triangle::reverse()],
            model_matrix: Matrix4::zero(),
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

    // getter

    pub fn polygons(&self) -> &Vec<Triangle> {
        &self.polygons
    }

    pub fn model_matrix(&mut self) -> &Matrix4<f64> {
        self.model_matrix = Matrix4::<f64>::identity()
            * Matrix4::from_translation(self.pos)
            * Matrix4::from_angle_x(Deg { 0: self.rot.x })
            * Matrix4::from_angle_y(Deg { 0: self.rot.y })
            * Matrix4::from_angle_z(Deg { 0: self.rot.z })
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        &self.model_matrix
    }
}
