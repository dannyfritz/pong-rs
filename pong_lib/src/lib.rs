extern crate nalgebra;
extern crate ncollide_geometry;

pub mod game;
pub mod menu;

pub struct Intents {
    pub paddle_1_up: u8,
    pub paddle_1_down: u8,
    pub paddle_2_up: u8,
    pub paddle_2_down: u8,
    pub menu_up: bool,
    pub menu_down: bool,
    pub menu_select: bool,
}

pub enum PongScene {
    Menu(menu::State),
    Game(game::State),
}

pub struct PongState {
    pub scene: PongScene,
}

impl PongState {
    pub fn new() -> PongState {
        PongState {
            scene: PongScene::Game(game::State::new()),
        }
    }
    pub fn interpolate(&mut self, dt: f64, intents: &Intents) {
        let pong_state_option = match &mut self.scene {
            PongScene::Game(game) => game.interpolate(dt, intents),
            PongScene::Menu(menu) => menu.interpolate(dt, intents),
        };
        if let Some(new_pong_state) = pong_state_option {
            self.scene = new_pong_state;
        }
    }
}
