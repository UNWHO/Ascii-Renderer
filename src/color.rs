use std::{cmp, ops};

#[derive(Debug)]
pub struct Color<T = u8> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn round(color: Color<f64>) -> Self {
        Color {
            r: color.r.round() as u8,
            g: color.g.round() as u8,
            b: color.b.round() as u8,
        }
    }

    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn lightness(&self) -> f64 {
        let r = self.r as f64 / 255 as f64;
        let g = self.g as f64 / 255 as f64;
        let b = self.b as f64 / 255 as f64;

        (r.max(g).max(b) + r.min(g).min(b)) / 2.0
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color { ..*self }
    }
}

impl Copy for Color {}

impl ops::Mul<f64> for Color {
    type Output = Color<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::<f64> {
            r: (self.r as f64) * rhs,
            g: (self.g as f64) * rhs,
            b: (self.b as f64) * rhs,
        }
    }
}

impl ops::Add for Color<f64> {
    type Output = Color<f64>;

    fn add(self, rhs: Self) -> Self::Output {
        Color::<f64> {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
