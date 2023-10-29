use crate::Color;

pub struct Fragment {
    pub color: Color,

    pub x: usize, // pixel coordinate
    pub y: usize, // pixel coordinate
    pub z: f64,   // z-depth in NDC, normalized by [-1, 1]
}
