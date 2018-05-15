pub mod ball;
pub mod endzone;
pub mod paddle;
pub mod player;

use Intents;
use PongScene;
use game::{ball::Ball, player::Player};
use ncollide2d::{query, math::{Isometry, Vector as Vector2}, shape::Shape};

const FIELD_TOP: f32 = 32.0;
const FIELD_BOTTOM: f32 = 0.0;
const FIELD_LEFT: f32 = 0.0;
const FIELD_RIGHT: f32 = 64.0;

pub trait Collidable {
    type S: Shape<f32>;
    fn to_iso(&self) -> Isometry<f32>;
    fn to_shape(&self) -> Self::S;
}

pub struct State {
    pub players: [Player; 2],
    pub ball: Option<Ball>,
}

impl State {
    pub fn new() -> State {
        State {
            players: [
                Player::new(FIELD_LEFT + 1.0, FIELD_LEFT - 2.0),
                Player::new(FIELD_RIGHT - 2.0, FIELD_RIGHT),
            ],
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
        for player in self.players.iter_mut() {
            let ref mut paddle = player.paddle;
            if paddle.pos.y >= FIELD_TOP {
                paddle.pos.y = FIELD_TOP;
            } else if paddle.pos.y - paddle.dim.y <= FIELD_BOTTOM {
                paddle.pos.y = paddle.dim.y;
            }
        }
        let mut scored = false;
        if let Some(ref mut ball) = self.ball {
            for player in self.players.iter_mut() {
                let ref paddle = player.paddle;
                if let Some(contact) = query::contact(
                    &paddle.to_iso(),
                    &paddle.to_shape(),
                    &ball.to_iso(),
                    &ball.to_shape(),
                    0.0,
                ) {
                    ball.bounce(contact.normal.unwrap(), contact.depth);
                }
                let ref endzone = player.endzone;
                if let Some(_contact) = query::contact(
                    &endzone.to_iso(),
                    &endzone.to_shape(),
                    &ball.to_iso(),
                    &ball.to_shape(),
                    0.0,
                ) {
                    player.score += 1;
                    scored = true;
                }
            }
            let y = ball.pos.y;
            let r = ball.r;
            if y + r >= FIELD_TOP {
                ball.bounce(Vector2::new(0.0, -1.0), y + r - FIELD_TOP);
            } else if y - r <= FIELD_BOTTOM {
                ball.bounce(Vector2::new(0.0, 1.0), y.abs() + r);
            }
        }
        if scored {
            self.ball = Some(Ball::new());
            println!("score: {}:{}", self.players[0].score, self.players[1].score);
        }
        None
    }
}
