use cgmath::{Point3, Vector3, Zero};
#[derive(Debug)]
pub struct Camera {
    pub pos: Point3<f64>,
    pub look_at: Point3<f64>,
    pub up: Vector3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::zero(),
        }
    }

    pub fn move_to(&mut self, v: &Point3<f64>) -> &mut Self {
        self.pos = *v;
        self
    }

    pub fn rotate(&mut self, look_at: &Point3<f64>, up: &Vector3<f64>) -> &mut Self {
        self.look_at = *look_at;
        self.up = *up;
        self
    }
}
