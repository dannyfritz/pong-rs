extern crate nalgebra;
extern crate ncollide_geometry;

pub mod game;
pub mod menu;

use game::Game;
use menu::Menu;

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
    Menu(Menu),
    Game(Game),
}

pub struct PongState {
    pub scene: PongScene,
}

impl PongState {
    pub fn new() -> PongState {
        PongState {
            scene: PongScene::Game(Game::new()),
        }
    }
    pub fn interpolate(&mut self, dt: f64, intents: &Intents) {
        let pong_state_option = match &mut self.scene {
            &mut PongScene::Game(ref mut game) => game.interpolate(dt, intents),
            &mut PongScene::Menu(ref mut menu) => menu.interpolate(dt, intents),
        };
        if let Some(new_pong_state) = pong_state_option {
            self.scene = new_pong_state;
        }
    }
}
