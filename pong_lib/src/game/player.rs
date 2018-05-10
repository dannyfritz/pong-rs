use game::endzone::Endzone;
use game::paddle::Paddle;

pub struct Player {
    pub paddle: Paddle,
    pub endzone: Endzone,
}

impl Player {
    pub fn new(x: f32) -> Player {
        Player {
            paddle: Paddle::new(x),
            endzone: Endzone::new(),
        }
    }
    pub fn interpolate(&mut self, dt: f64) {}
    pub fn move_up(&mut self, dt: f64, magnitude: u8) {
        self.paddle.y += self.paddle.speed * magnitude as f32 / u8::max_value() as f32 * dt as f32;
    }
    pub fn move_down(&mut self, dt: f64, magnitude: u8) {
        self.paddle.y += -self.paddle.speed * magnitude as f32 / u8::max_value() as f32 * dt as f32;
    }
}
