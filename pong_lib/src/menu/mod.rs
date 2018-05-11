use Intents;
use PongScene;

pub struct State {}

impl State {
    pub fn interpolate(&mut self, _dt: f64, _intents: &Intents) -> Option<PongScene> {
        None
    }
}
