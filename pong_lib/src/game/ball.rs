use ncollide2d::{math::{Isometry, Vector as Vector2}, shape::{self, Shape}};

pub struct Ball {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub r: f32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            pos: Vector2::new(32.0, 16.0),
            vel: Vector2::new(3.0, 0.0),
            r: 2.0,
        }
    }
    pub fn interpolate(&mut self, dt: f64) {
        self.pos += self.vel * dt as f32;
    }
    pub fn to_iso(&self) -> Isometry<f32> {
        Isometry::new(self.pos, 0.0)
    }
    pub fn to_shape(&self) -> impl Shape<f32> {
        shape::Ball::new(self.r)
    }
}
