pub mod ball;
pub mod endzone;
pub mod paddle;
pub mod player;

use Intents;
use PongScene;
use game::ball::Ball;
use game::player::Player;

pub struct State {
    pub score: [usize; 2],
    pub players: [Player; 2],
    pub ball: Option<Ball>,
}

impl State {
    pub fn new() -> State {
        State {
            score: [0, 0],
            players: [Player::new(1.0), Player::new(62.0)],
            ball: Some(Ball::new()),
        }
    }
    pub fn interpolate(&mut self, dt: f64, intents: &Intents) -> Option<PongScene> {
        if intents.paddle_1_up > 0 {
            self.players[0].move_up(dt, intents.paddle_1_up);
        }
        if intents.paddle_1_down > 0 {
            self.players[0].move_down(dt, intents.paddle_1_down);
        }
        if intents.paddle_2_up > 0 {
            self.players[1].move_up(dt, intents.paddle_2_up);
        }
        if intents.paddle_2_down > 0 {
            self.players[1].move_down(dt, intents.paddle_2_down);
        }
        for player in self.players.iter_mut() {
            player.interpolate(dt);
        }
        if let Some(ref mut ball) = self.ball {
            ball.interpolate(dt);
        }
        None
    }
}
