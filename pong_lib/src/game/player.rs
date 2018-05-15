use game::endzone::Endzone;
use game::paddle::Paddle;

pub struct Player {
    pub paddle: Paddle,
    pub endzone: Endzone,
    pub score: u16,
}

impl Player {
    pub fn new(x: f32, ez_x: f32) -> Player {
        Player {
            paddle: Paddle::new(x),
            endzone: Endzone::new(ez_x),
            score: 0,
        }
    }
    pub fn interpolate(&mut self, _dt: f64) {}
    pub fn move_up(&mut self, dt: f64, magnitude: u8) {
        self.paddle.pos.y +=
            self.paddle.speed * magnitude as f32 / u8::max_value() as f32 * dt as f32;
    }
    pub fn move_down(&mut self, dt: f64, magnitude: u8) {
        self.paddle.pos.y +=
            -self.paddle.speed * magnitude as f32 / u8::max_value() as f32 * dt as f32;
    }
}
