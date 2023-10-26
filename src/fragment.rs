use crate::Color;

pub struct Fragment {
    pub color: Color,

    pub x: i32, // pixel coordinate
    pub y: i32, // pixel coordinate
    pub z: f64, // z-depth in NDC, normalized by [-1, 1]
}
