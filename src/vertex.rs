use crate::Color;
use cgmath::Point3;

#[derive(Debug)]
pub struct Vertex {
    pub pos: Point3<f64>,
    pub color: Color,
}

impl Vertex {
    pub fn new(pos: Point3<f64>, color: Color) -> Self {
        Vertex { pos, color }
    }

    pub fn white(pos: Point3<f64>) -> Self {
        Vertex {
            pos: pos,
            color: Color::white(),
        }
    }

    pub fn clone(&self) -> Self {
        Vertex {
            pos: Point3::clone(&self.pos),
            color: Color::clone(&self.color),
        }
    }
}
