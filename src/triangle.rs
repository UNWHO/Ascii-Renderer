use crate::Vertex;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(points: [Vertex; 3]) -> Self {
        Triangle { vertices: points }
    }

    pub fn clone(&self) -> Self {
        Triangle {
            vertices: [
                Vertex::clone(&self.vertices[0]),
                Vertex::clone(&self.vertices[1]),
                Vertex::clone(&self.vertices[2]),
            ],
        }
    }
}
