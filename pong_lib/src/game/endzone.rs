use super::{FIELD_BOTTOM, FIELD_TOP};
use game::Collidable;
use ncollide2d::{shape, math::{Isometry, Vector as Vector2}};

pub struct Endzone {
    pos: Vector2<f32>,
    dim: Vector2<f32>,
}

impl Endzone {
    pub fn new(x: f32) -> Endzone {
        Endzone {
            pos: Vector2::new(x, FIELD_TOP),
            dim: Vector2::new(2.0, FIELD_TOP - FIELD_BOTTOM),
        }
    }
}

impl Collidable for Endzone {
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
