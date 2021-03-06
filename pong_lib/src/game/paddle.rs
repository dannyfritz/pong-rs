use super::FIELD_TOP;
use game::Collidable;
use ncollide2d::{shape, math::{Isometry, Vector as Vector2}};

pub struct Paddle {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub dim: Vector2<f32>,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32) -> Paddle {
        Paddle {
            pos: Vector2::new(x, FIELD_TOP / 2.0 + 8.0 / 2.0),
            vel: Vector2::new(0.0, 0.0),
            dim: Vector2::new(1.0, 8.0),
            speed: 10.0,
        }
    }
}

impl Collidable for Paddle {
    type S = shape::Cuboid<f32>;
    fn to_iso(&self) -> Isometry<f32> {
        Isometry::new(
            Vector2::new(self.pos.x + self.dim.x / 2.0, self.pos.y - self.dim.y / 2.0),
            0.0,
        )
    }
    fn to_shape(&self) -> Self::S {
        shape::Cuboid::new(self.dim / 2.0)
    }
}
