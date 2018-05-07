mod ball;
mod endzone;
mod paddle;
mod player;

use game::ball::Ball;
use game::player::Player;
use Intents;
use PongScene;

pub struct Game {
    pub score: [usize; 2],
    pub players: [Player; 2],
    pub ball: Option<Ball>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            score: [0, 0],
            players: [Player::new(), Player::new()],
            ball: Some(Ball::new()),
        }
    }
    pub fn interpolate(&mut self, dt: f64, intents: &Intents) -> Option<PongScene> {
        if intents.paddle_1_up > 0 {
            self.players[0].move_up((dt * intents.paddle_1_up as f64 / 255.0).into());
        } else if intents.paddle_1_down > 0 {
            self.players[0].move_down((dt * intents.paddle_1_down as f64 / 255.0).into());
        }
        None
    }
}
