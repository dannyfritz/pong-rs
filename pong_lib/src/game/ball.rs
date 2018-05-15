use game::Collidable;
use ncollide2d::{shape, math::{Isometry, Vector as Vector2}};

pub struct Ball {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub r: f32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            pos: Vector2::new(32.0, 16.0),
            vel: Vector2::new(5.0, 10.0),
            r: 0.5,
        }
    }
    pub fn interpolate(&mut self, dt: f64) {
        self.pos += self.vel * dt as f32;
    }
    pub fn bounce(&mut self, normal: Vector2<f32>, depth: f32) {
        // reflection around a normal: 2(v·n)n
        self.vel = self.vel - 2.0 * (self.vel.dot(&normal)) * normal;
        self.pos += depth * self.vel.normalize();
    }
}

impl Collidable for Ball {
    type S = shape::Cuboid<f32>;
    fn to_iso(&self) -> Isometry<f32> {
        Isometry::new(self.pos, 0.0)
    }
    fn to_shape(&self) -> Self::S {
        shape::Cuboid::new(Vector2::new(self.r, self.r))
    }
}
