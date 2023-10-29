use crate::{Color, Vertex};
use cgmath::Point3;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(points: [Vertex; 3]) -> Self {
        Triangle { vertices: points }
    }

    pub fn default() -> Self {
        Triangle {
            vertices: [
                Vertex::new(Point3::new(0.0, 0.5, 0.0), Color::new(255, 0, 0)),
                Vertex::new(Point3::new(-0.5, -0.5, 0.0), Color::new(0, 255, 0)),
                Vertex::new(Point3::new(0.5, -0.5, 0.0), Color::new(0, 0, 255)),
            ],
        }
    }

    pub fn reverse() -> Self {
        Triangle {
            vertices: [
                Vertex::new(Point3::new(0.5, -0.5, 0.0), Color::new(0, 0, 255)),
                Vertex::new(Point3::new(-0.5, -0.5, 0.0), Color::new(0, 255, 0)),
                Vertex::new(Point3::new(0.0, 0.5, 0.0), Color::new(255, 0, 0)),
            ],
        }
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        Triangle {
            vertices: [
                Vertex::clone(&self.vertices[0]),
                Vertex::clone(&self.vertices[1]),
                Vertex::clone(&self.vertices[2]),
            ],
        }
    }
}
