use Intents;
use PongScene;

pub struct Menu {}

impl Menu {
    pub fn interpolate(&mut self, dt: f64, intents: &Intents) -> Option<PongScene> {
        None
    }
}
