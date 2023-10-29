use crate::Color;

pub struct Fragment {
    pub color: Color,

    pub x: usize, // pixel coordinate
    pub y: usize, // pixel coordinate
    pub z: f64,   // z-depth in NDC, normalized by [-1, 1]
}

impl Fragment {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0.0,
            color: Color::white(),
        }
    }
}

impl Copy for Fragment {}

impl Clone for Fragment {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            color: self.color,
        }
    }
}
