use game::endzone::Endzone;
use game::paddle::Paddle;

pub struct Player {
    pub paddle: Paddle,
    pub endzone: Endzone,
}

impl Player {
    pub fn new() -> Player {
        Player {
            paddle: Paddle::new(),
            endzone: Endzone::new(),
        }
    }
    pub fn move_up(&mut self, magnitude: f64) {
        self.paddle.y += magnitude as f32;
    }
    pub fn move_down(&mut self, magnitude: f64) {
        self.paddle.y -= magnitude as f32;
    }
}
